use anyhow::Context;
use error::{ConfigError, ConfigResult};
use json_comments::StripComments;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
use types::{CoreConfig, VenusConfig};

use crate::consts::{VENUS_CONFIG_PATH, VENUS_V2RAY_PATH, VERSION};

pub mod error;
pub mod types;

/// All config field
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Core config from `config.json`
    pub core: Option<CoreConfig>,
    /// Venus config from `config.toml`
    pub venus: VenusConfig,
}

/// Core config and global states
impl Config {
    pub fn new() -> ConfigResult<Self> {
        let v_config = VenusConfig::default();

        let config = Self {
            core: None,
            venus: v_config,
        };

        Ok(config)
    }

    pub fn reload_rua(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(VENUS_CONFIG_PATH.as_ref());
        if !path.exists() {
            info!("venus config file not exist, creating default");
            self.write_rua()?;
            return Ok(());
        }
        let mut config_file = File::open(path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let mut rua_config = toml::from_str::<VenusConfig>(&buffer)?;
        debug!("reloading rua config: {:?}", rua_config);
        rua_config.version = VERSION.into();
        self.venus = rua_config;
        Ok(())
    }

    /// Reload core config file from VConfig
    pub fn reload_core(&mut self) -> ConfigResult<()> {
        let path_str = format!("{}/config.json", &*VENUS_V2RAY_PATH);
        let path_ctx = path_str.clone();
        let ctx = || format!("reload config {} failed", path_ctx);

        let path = PathBuf::from(path_str);
        let core_file = File::open(path).with_context(ctx)?;
        let stripped = StripComments::new(core_file);
        let core_config: CoreConfig = serde_json::from_reader(stripped).with_context(ctx)?;
        debug!("reloading core config: {:?}", core_config);
        self.core = Some(core_config);
        Ok(())
    }

    ///  Write core config to config file
    pub fn write_core(&mut self) -> ConfigResult<()> {
        let path = format!("{}/config.json", &*VENUS_V2RAY_PATH);
        let path_ctx = path.clone();
        let ctx = || format!("write config {} failed", path_ctx);

        let path = PathBuf::from(path);
        let config = self.core.as_ref().ok_or(ConfigError::Empty(
            "write_core: v2ray core config is empty".into(),
        ))?;
        let core_file = OpenOptions::new()
            .write(true)
            .open(path)
            .with_context(ctx)?;
        core_file.set_len(0).with_context(ctx)?;
        serde_json::to_writer_pretty(&core_file, &config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(VENUS_CONFIG_PATH.as_ref());
        let path_ctx = path.clone();
        let ctx = || format!("write config {:?} failed", path_ctx);

        let mut rua_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let rua_string = toml::to_string(&self.venus).with_context(ctx)?;
        rua_file.set_len(0).with_context(ctx)?;
        rua_file
            .write_all(rua_string.as_bytes())
            .with_context(ctx)?;
        Ok(())
    }
}
