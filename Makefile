all: build release test

build:
	cargo build

release:
	cargo build --release

test:
	@rm -rf tests/generator_test/
	@cargo new tests/generator_test
	cargo test

install: release
	cp target/release/cargo-mod ~/.cargo/bin
