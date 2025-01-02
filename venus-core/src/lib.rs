use std::{
    env,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc::Sender,
    thread,
};

use anyhow::{anyhow, Ok as AOk};
use base64::{engine::general_purpose, Engine};
use config::{
    types::{Node, NodeType, Subscription},
    Config,
};
use consts::{NAME, VENUS_V2RAY_PATH, VERSION};
use error::{log_err, SubscriptionError, VenusError, VenusResult};
use log::debug;
use message::MessageType;
use reqwest::header::USER_AGENT;
use tonic::async_trait;

pub mod config;
pub mod consts;
pub mod error;
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

#[async_trait]
pub trait VenusSubscriptor {
    /// Add subscription
    async fn add_subscription(&mut self, name: String, url: String) -> VenusResult<()>;
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
            Err(VenusError::Core("core not running".into()))
        }
    }

    /// Kill core and spawn new one
    fn restart(&mut self) -> VenusResult<()> {
        self.kill_core()?;
        self.spawn_core()?;
        Ok(())
    }
}

#[async_trait]
impl VenusSubscriptor for Venus {
    async fn add_subscription(&mut self, name: String, url: String) -> VenusResult<()> {
        let subscriptions = &self.config.venus.subscriptions;
        if subscriptions.iter().any(|s| s.url == url) {
            return Err(SubscriptionError::AlreadyExist.into());
        }
        let subs = request_subs(&name, &url).await?;
        let subscription = Subscription {
            name: name.into(),
            url: url.into(),
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
    let core = Command::new(core_exec_path).args(["version"]).output()?;
    let output = String::from_utf8_lossy(&core.stdout);
    let stdout = output.split(' ').collect::<Vec<_>>();
    let version = stdout.get(1).map(|v| v.to_string()).ok_or(anyhow!(""))?;
    Ok(version)
}

/// Send http request to download subscription info
async fn request_subs(name: &str, url: &str) -> VenusResult<Vec<Node>> {
    let client = reqwest::ClientBuilder::new().no_proxy().build()?;
    let result = client
        .get(url)
        .header(USER_AGENT, format!("{}/{}", NAME, VERSION))
        .send()
        .await?
        .text()
        .await?;

    // Decode result to vmess://...
    let subscription = general_purpose::STANDARD.decode(result)?;
    // let subscription = String::from_utf8_lossy(&subscription).to_string();
    let subscription = String::from_utf8(subscription)?.to_string();
    // Serizlize outbound nodes to json
    let name = name.to_string();
    let sub_handler = |(index, line): (usize, &str)| -> VenusResult<Node> {
        let (node_type, link) = line
            .split_once("://")
            .ok_or(anyhow!("Cannot serialize node link"))?;
        let link = general_purpose::STANDARD.decode(link)?;
        let link = String::from_utf8_lossy(&link).to_string();
        let mut node = serde_json::from_str::<Node>(&link)?;

        node.subs = Some(name.clone().into());
        // Add unique id
        let id = md5::compute(format!("{}-{}-{}-{}", node.ps, node.add, node.port, index));
        node.node_id = Some(format!("{:?}", id).into());
        node.raw_link = Some(line.to_string().into());
        node.node_type = Some(NodeType::from(node_type));
        Ok(node)
    };
    let subscription = subscription
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(sub_handler)
        .collect::<VenusResult<Vec<_>>>()?;
    debug!("{subscription:?}");
    Ok(subscription)
}
