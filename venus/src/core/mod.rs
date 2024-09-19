use std::{
    process::exit,
    sync::{
        mpsc::{self},
        LazyLock, Mutex,
    },
};
use tracing::error;

use venus_core::{message::Message, Venus};

pub static MSG: LazyLock<Mutex<Message>> = LazyLock::new(|| Mutex::new(mpsc::channel()));

pub static CORE: LazyLock<Mutex<Venus>> = LazyLock::new(|| {
    let venus = {
        let msg = match MSG.lock() {
            Ok(m) => m,
            Err(err) => {
                error!("cannot initialize venus core {err}");
                exit(1);
            }
        };
        match Venus::new(msg.0.clone()) {
            Ok(v) => v,
            Err(err) => {
                error!("cannot initialize venus core {err}");
                exit(1);
            }
        }
    };
    Mutex::new(venus)
});
