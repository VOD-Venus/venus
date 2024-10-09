use std::{borrow::Cow, env, sync::LazyLock};

use tracing::warn;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DEFAULT_PORT: u16 = 4000;
pub const RUA_COMPILER: &str = env!("RUA_COMPILER");

/// Default ui assets path
pub const DEFAULT_VENUS_UI_PATH: &str = "./venus-ui/dist";
/// ui assets path
pub static VENUS_UI_PATH: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    env::var("VENUS_UI_PATH")
        .map_err(|err| {
           warn!("VENUS_UI_PATH env not specified: {err}. using default localtion {DEFAULT_VENUS_UI_PATH}");
        })
        .unwrap_or(DEFAULT_VENUS_UI_PATH.into()).into()
});
