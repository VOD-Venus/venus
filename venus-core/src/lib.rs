use std::{
    env,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc::Sender,
    thread,
};

use anyhow::{anyhow, Context, Ok as AOk};
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use config::{
    types::{Node, NodeType, Subscription},
    Config,
};
use consts::{NAME, VENUS_V2RAY_PATH, VERSION};
use error::{log_err, SubscriptionError, VenusError, VenusResult};
use message::MessageType;
use reqwest::header::USER_AGENT;

pub mod config;
pub mod consts;
pub mod error;
pub mod grpc;
pub mod message;

pub mod v2ray_core {
    tonic::include_proto!("v2ray.core.app.stats.command");
}

pub trait VenusCore {
    /// Spawn a thread to execute v2ray core binary
    fn spawn_core(&mut self) -> VenusResult<()>;

    /// Kill core process if exist
    fn kill_core(&mut self) -> VenusResult<()>;

    /// Kill core and spawn new one
    fn restart(&mut self) -> VenusResult<()>;
}

pub trait VenusSubscriptor {
    /// Add subscription
    ///
    /// # Parameters
    /// * `name`: subscription name
    /// * `url`: subscription url
    ///
    /// # Returns
    /// * `VenusResult<()>`
    fn add_subscription(
        &mut self,
        name: String,
        url: String,
    ) -> impl std::future::Future<Output = VenusResult<()>> + Send;
}

#[derive(Debug)]
pub struct Venus {
    /// v2ray and venus's self config
    pub config: Config,

    /// v2ray version
    pub version: String,
    /// v2ray process
    child: Option<Child>,

    /// message
    message_tx: Sender<MessageType>,
}

impl Venus {
    /// Create a new `Venus` instance
    ///
    /// # Parameters
    /// * `message_tx`: message sender
    ///
    /// # Returns
    /// * `VenusResult<Self>`
    pub fn new(message_tx: Sender<MessageType>) -> VenusResult<Self> {
        let config = Config::new()?;

        let asset_path = PathBuf::from(VENUS_V2RAY_PATH.as_ref());
        env::set_var("V2RAY_LOCATION_ASSET", asset_path);

        Ok(Self {
            config,
            version: String::new(),
            child: None,
            message_tx,
        })
    }
}

impl VenusCore for Venus {
    /// Spawn a thread to execute v2ray core binary
    fn spawn_core(&mut self) -> VenusResult<()> {
        self.version = core_version()?;

        let core_exec_path = format!("{}/v2ray", &*VENUS_V2RAY_PATH);
        let mut child = Command::new(core_exec_path)
            .args(["run"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let tx = &self.message_tx;

        let stdout = child.stdout.take().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "child stdout is empty",
        ))?;
        let stderr = child.stderr.take().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "child stderr is empty",
        ))?;
        let tx = tx.clone();
        let child_handler = move || {
            let stdout_tx = tx.clone();
            let mut handlers = Vec::with_capacity(2);
            let stdout_handler = thread::spawn(move || {
                let mut lines = BufReader::new(stdout).lines();
                lines.try_for_each(|line| {
                    stdout_tx.send(MessageType::Core(line?))?;
                    AOk(())
                })?;
                AOk(())
            });
            let stderr_tx = tx.clone();
            let stderr_handler = thread::spawn(move || {
                let mut lines = BufReader::new(stderr).lines();
                lines.try_for_each(|line| {
                    stderr_tx.send(MessageType::Core(line?))?;
                    AOk(())
                })?;
                AOk(())
            });

            handlers.push(stdout_handler);
            handlers.push(stderr_handler);
            handlers
                .into_iter()
                .try_for_each(|handler| {
                    handler
                        .join()
                        .map_err(|err| anyhow!("child join failed {err:?}"))??;
                    AOk(())
                })
                .map_err(log_err)?;
            AOk(())
        };
        thread::spawn(child_handler);

        self.child = Some(child);
        Ok(())
    }

    /// Kill core process if exist
    fn kill_core(&mut self) -> VenusResult<()> {
        if let Some(core) = self.child.as_mut() {
            self.message_tx.send(MessageType::Terminate)?;
            core.kill()?;
            self.child = None;
            Ok(())
        } else {
            Err(VenusError::CoreLaunch("core not running".into()))
        }
    }

    /// Kill core and spawn new one
    fn restart(&mut self) -> VenusResult<()> {
        self.kill_core()?;
        self.spawn_core()?;
        Ok(())
    }
}

impl VenusSubscriptor for Venus {
    async fn add_subscription(&mut self, name: String, url: String) -> VenusResult<()> {
        let subscriptions = &self.config.venus.subscriptions;
        if subscriptions.iter().any(|s| s.url == url) {
            return Err(SubscriptionError::AlreadyExist(name.clone()).into());
        }
        let subs = fetch_subscription(&name, &url).await?;
        let subscription = Subscription {
            name: name.into(),
            url: url.into(),
            updated: Utc::now(),
            nodes: subs,
        };
        self.config.venus.subscriptions.push(subscription);
        self.config.write_rua()?;
        Ok(())
    }
}

/// Detect the v2ray core version
pub fn core_version() -> VenusResult<String> {
    let core_exec_path = format!("{}/v2ray", &*VENUS_V2RAY_PATH);
    let output = Command::new(core_exec_path)
        .args(["version"])
        .output()
        .map_err(|e| VenusError::CoreLaunch(e.to_string()))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let version = output_str
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| VenusError::VersionParse(String::from_utf8_lossy(&output.stdout).into()))?;

    Ok(version.to_string())
}

/// Send http request to download subscription info
///
/// # Parameters
/// * `name`: subscription name
/// * `url`: subscription url
async fn fetch_subscription(name: &str, url: &str) -> VenusResult<Vec<Node>> {
    let client = reqwest::ClientBuilder::new()
        .no_proxy()
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(url)
        .header(USER_AGENT, format!("{NAME}/{VERSION}"))
        .send()
        .await
        .context("Failed to send subscription request")?;

    let content = response
        .text()
        .await
        .context("Failed to read subscription response")?;

    parse_subscription_content(name, url, &content)
}

/// 解析订阅内容
fn parse_subscription_content(name: &str, url: &str, content: &str) -> VenusResult<Vec<Node>> {
    let decoded = general_purpose::STANDARD
        .decode(content)
        .map_err(|e| SubscriptionError::Base64Decode(url.to_string(), e))?;

    let content_str = String::from_utf8(decoded)
        .map_err(|e| SubscriptionError::Utf8Conversion(url.to_string(), e))?;

    content_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(idx, line)| parse_node(line, name, idx))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}

/// 解析单节点信息
fn parse_node(line: &str, subs_name: &str, index: usize) -> Result<Node, SubscriptionError> {
    let (protocol, payload) = line
        .split_once("://")
        .ok_or_else(|| SubscriptionError::InvalidFormat(line.to_string()))?;

    let decoded = general_purpose::STANDARD
        .decode(payload)
        .map_err(|e| SubscriptionError::Base64Decode(line.into(), e))?;

    let mut node: Node = serde_json::from_slice(&decoded).map_err(SubscriptionError::JsonParse)?;

    // 生成唯一标识
    let id_data = format!("{}-{}-{}-{}", node.ps, node.add, node.port, index);
    node.node_id = Some(format!("{:x}", md5::compute(id_data)).into());

    node.subs = Some(subs_name.to_string().into());
    node.raw_link = Some(line.to_string().into());
    node.node_type = Some(NodeType::from(protocol));

    Ok(node)
}
