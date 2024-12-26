# Venus core

[Venus](https://github.com/VOD-Venus/venus) core library. Spawn v2ray core as a child process and controls its configs.

## Design

The instances will accept a `mpsc::channel` Sender as core `stdio` messenger and pipe it into the channel.

## Example

```rust
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
    let lock = &MSG.lock();
    let child_rx = match lock {
        Ok(msg) => &msg.1,
        Err(err) => {
            error!("lock message failed {err}");
            return;
        }
    };
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

## Communicate with v2ray core

Venus core use gRPC to communicate with v2ray core. First, we need compile the proto files that from v2ray core: https://github.com/v2fly/v2ray-core.

Copy all the proto files to `venus-core/proto` folder and run `make` to generate the rust code.

macOS:

```bash
rsync -R **/*/*.proto ../venus/venus-core/proto/
```
