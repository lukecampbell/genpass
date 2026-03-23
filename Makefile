.PHONY: build

built:
	cargo build

lint:
	cargo clippy -- -Dwarnings
