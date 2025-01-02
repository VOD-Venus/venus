use std::{
    process::exit,
    sync::mpsc::{self},
};
use tokio::sync::{Mutex, OnceCell};
use tracing::error;

use venus_core::{message::Message, Venus};

static MSG: OnceCell<Mutex<Message>> = OnceCell::const_new();
pub async fn global_message() -> &'static Mutex<Message> {
    MSG.get_or_init(|| async { Mutex::new(mpsc::channel()) })
        .await
}

static CORE: OnceCell<Mutex<Venus>> = OnceCell::const_new();
pub async fn global_core() -> &'static Mutex<Venus> {
    CORE.get_or_init(|| async {
        let msg = global_message().await.lock().await;
        match Venus::new(msg.0.clone()) {
            Ok(v) => Mutex::new(v),
            Err(err) => {
                error!("cannot initialize venus core {err}");
                exit(1);
            }
        }
    })
    .await
}
