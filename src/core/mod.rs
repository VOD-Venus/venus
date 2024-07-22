use std::{
    process::exit,
    sync::{Mutex, OnceLock},
};
use tracing::error;

use venus_core::Venus;

pub static CORE: OnceLock<Mutex<Venus>> = OnceLock::new();

pub fn get_core() -> &'static Mutex<Venus> {
    CORE.get_or_init(|| {
        let venus = match Venus::new() {
            Ok(v) => v,
            Err(err) => {
                error!("cannot initialize venus core {err}");
                exit(1);
            }
        };
        Mutex::new(venus)
    })
}
