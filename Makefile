# RUSTFLAGS := "-Z threads=8 -C target-cpu=native"

# CARGO = RUSTFLAGS=$(RUSTFLAGS) cargo
CARGO = cargo
RUSTC = rustc
CROSS = CROSS_REMOTE=1 cross

all: build

build: ui
	$(CARGO) build -p venus

release: ui-release
	$(CARGO) build -p venus --release

dev:
	VENUS_LOG=debug $(CARGO) watch -x "run -p venus"

run:
	VENUS_LOG=debug $(CARGO) run -p venus

ui:
	cd venus-ui \
		&& trunk build

ui-dev:
	cd venus-ui \
		&& trunk serve

ui-release: ui-clean
	cd venus-ui \
		&& trunk build --release

ui-clean:
	cd venus-ui \
		&& rm -rf dist

test:
	$(CARGO) test

clean:
	$(CARGO) clean

clean-release:
	rm -rf ./venus/target/release/
	rm -rf ./venus/target/debug/

check:
	$(CARGO) check

format:
	$(CARGO) fmt

lint:
	$(CARGO) clippy

fix:
	cd venus-ui \
		&& leptosfmt . \
		&& $(CARGO) fix --allow-dirty --all-features && $(CARGO) fmt \
		&& cd .. \
		&& $(CARGO) fix --allow-dirty --all-features && $(CARGO) fmt

linux-musl: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-musl

aarch64-unknown-linux-musl: clean-release
	$(CROSS) build -p venus --release --target aarch64-unknown-linux-musl

linux-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-gnu

windows-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-pc-windows-gnu

freebsd: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-freebsd

loongarch: clean-release
	$(CROSS) build -p venus --release --target loongarch64-unknown-linux-gnu

deps:
	$(CARGO) install --locked trunk \
		&& $(CARGO) install --locked leptosfmt \
		&& rustup target add wasm32-unknown-unknown \
		&& cd venus-ui \
		&& deno install \
		&& cd ..

v2ray:
	python -m venv .venus \
		&& source .venus/bin/activate \
		&& pip install -r scripts/requirements.txt \
		&& python scripts/download-core.py \
		&& chmod +x v2ray-core/v2ray


.PHONY: all
