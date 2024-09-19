CARGO = cargo
RUSTC = rustc
CROSS = cross

all: build

build:
	$(CARGO) build -p venus

release: clean
	$(CARGO) build -p venus --release

dev:
	VENUS_LOG=debug $(CARGO) watch -x run -p venus

run:
	$(CARGO) run -p venus

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
	$(CARGO) +nightly fmt

lint:
	$(CARGO) +nightly clippy

fix:
	$(CARGO) +nightly fix --allow-dirty --all-features && $(CARGO) +nightly fmt

linux-musl: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-musl

linux-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-gnu

windows-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-pc-windows-gnu

freebsd: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-freebsd

loongarch: clean-release
	$(CROSS) build -p venus --release --target loongarch64-unknown-linux-gnu

deps:
	python -m venv .venus \
		&& source .venus/bin/activate \
		&& pip install -r scripts/requirements.txt \
		&& python scripts/download-core.py

.PHONY: all
