# Venus core

[Venus](https://github.com/VOD-Venus/venus) core library. Spawn v2ray core as a child process and controls its configs.

## Design

The instances will accept a `mpsc::channel` Sender as core `stdio` messenger and pipe it into the channel.

## Example

```rust
// sender and receiver
let msg = mpsc::channel();

let mut venus = match Venus::new(msg.0.clone()) {
    Ok(v) => v,
    Err(err) => {
        error!("cannot initialize venus core {err}");
        exit(1);
    }
};

venus
    .config
    .reload_rua()
    .with_context(|| "reading venus configuration failed")?;
venus
    .config
    .reload_core()
    .with_context(|| "reading core configuration failed")?;
venus
    .config
    .write_core()
    .with_context(|| "write core configuration failed")?;
venus.spawn_core().with_context(|| "staring core failed")?;
// global message handler
thread::spawn(move || {
    let child_rx = msg.1;
    while let Ok(msg) = child_rx.recv() {
        match msg {
            MessageType::Core(msg) => {
                let core_span = span!(Level::INFO, "CORE").entered();
                info!("{msg}");
                core_span.exit();
            }
        }
    }
});
```

### Use with mutex

Because the `async_trait` can't used with std mutex.
So we need to use `tokio::sync::Mutex` to store core as global variable.

```rust
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
```

And core will send a termination message by the channel we passed in.
So we can drop the mutex when we receive the termination message.

```rust
{
    info!("venus {RUA_COMPILER}");
    let venus = &mut global_core().await.lock().await;
    info!("string core");
    venus
        .config
        .reload_rua()
        .with_context(|| "reading venus configuration failed")?;
    venus
        .config
        .reload_core()
        .with_context(|| "reading core configuration failed")?;
    venus
        .config
        .write_core()
        .with_context(|| "write core configuration failed")?;
    venus.spawn_core().with_context(|| "staring core failed")?;
}
tokio::spawn(async move {
    // global message handler
    let child_rx = &global_message().await.lock().await.1;
    let core_span = span!(Level::INFO, "CORE").entered();
    while let Ok(msg) = child_rx.recv() {
        match msg {
            MessageType::Core(msg) => {
                info!("{msg}");
            }
            MessageType::Terminate => {
                info!("core stopping");
                break;
            }
        }
    }
    core_span.exit();
});
```

## Communicate with v2ray core

Venus core use gRPC to communicate with v2ray core. First, we need compile the proto files that from v2ray core: https://github.com/v2fly/v2ray-core.

Copy all the proto files to `venus-core/proto` folder and run `make` to generate the rust code.

macOS:

```bash
rsync -R **/*/*.proto ../venus/venus-core/proto/
```
