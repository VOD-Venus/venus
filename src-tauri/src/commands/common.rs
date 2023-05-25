use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use base64::engine::general_purpose;
use base64::Engine;
use log::debug;
use tauri::State;

use crate::config::{ConfigState, Node, Subscription, VConfig};
use crate::utils::error::VResult;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn current_dir() -> Result<PathBuf, String> {
    Ok(env::current_dir().expect("error while read current dir"))
}

#[tauri::command]
pub async fn add_subscription(url: String, config: State<'_, Arc<Mutex<VConfig>>>) -> VResult<()> {
    let result = reqwest::get(&url).await?.text().await?;

    let mut config = config.lock()?;

    // Decode result to vmess://...
    let subscripition = general_purpose::STANDARD.decode(result)?;
    let subscripition = String::from_utf8_lossy(&subscripition).to_string();
    // Serizlize outbound nodes to json
    let mut subscripition = subscripition
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.replace("vmess://", "");
            let line = general_purpose::STANDARD.decode(line)?;
            let line = String::from_utf8_lossy(&line).to_string();
            Ok(serde_json::from_str::<Node>(&line)?)
        })
        .collect::<VResult<Vec<_>>>()?;
    debug!("{subscripition:?}");

    // Write subscription and nodes to config file
    let nodes = config.rua.nodes.take();
    if let Some(mut nodes) = nodes {
        nodes.append(&mut subscripition);
        config.rua.nodes = Some(nodes);
    } else {
        config.rua.nodes = Some(subscripition)
    };
    let sub = Subscription {
        name: "test".to_owned(),
        url,
    };
    let rua_subs = config.rua.subscriptions.take();
    if let Some(mut subscriptions) = rua_subs {
        subscriptions.push(sub);
        config.rua.subscriptions = Some(subscriptions);
    } else {
        config.rua.subscriptions = Some(vec![sub])
    }
    config.write_rua()?;
    Ok(())
}

#[tauri::command]
pub fn get_rua_nodes(state: State<'_, ConfigState>) -> VResult<String> {
    let config = state.lock()?;
    let nodes = serde_json::to_string(&config.rua.nodes)?;
    Ok(nodes)
}

#[tauri::command]
pub fn get_config(state: State<'_, ConfigState>) -> VResult<String> {
    let config = serde_json::to_string(state.inner())?;
    Ok(config)
}
