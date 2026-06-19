.PHONY: build test ci clean

build:
	cargo build --target wasm32-unknown-unknown

test:
	cargo test

ci: build test

clean:
	cargo clean
