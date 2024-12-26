use std::{
    env,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc::Sender,
    thread,
};

use anyhow::{anyhow, Ok as AOk};
use config::Config;
use consts::VENUS_V2RAY_PATH;
use error::{log_err, VenusError, VenusResult};
use message::MessageType;

pub mod config;
pub mod consts;
pub mod error;
pub mod message;

pub mod v2ray_core {
    tonic::include_proto!("v2ray.core.app.stats.command");
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

impl Venus {
    /// Spawn a thread to execute v2ray core binary
    pub fn spawn_core(&mut self) -> VenusResult<()> {
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
    pub fn kill_core(&mut self) -> VenusResult<()> {
        if let Some(core) = self.child.as_mut() {
            core.kill()?;
            self.child = None;
            Ok(())
        } else {
            Err(VenusError::Core("core not running".into()))
        }
    }

    /// Kill core and spawn new one
    pub fn restart(&mut self) -> VenusResult<()> {
        self.kill_core()?;
        self.spawn_core()?;
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
