use std::{
    process::exit,
    sync::{LazyLock, Mutex},
};
use tracing::error;

use venus_core::Venus;

pub static CORE: LazyLock<Mutex<Venus>> = LazyLock::new(|| {
    let venus = match Venus::new() {
        Ok(v) => v,
        Err(err) => {
            error!("cannot initialize venus core {err}");
            exit(1);
        }
    };
    Mutex::new(venus)
});
