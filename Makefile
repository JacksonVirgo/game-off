
install:
	rustup target install wasm32-unknown-unknown
	cargo install wasm-server-runner


run:
	cargo run

run-web:
	cargo build --target wasm32-unknown-unknown
	wasm-server-runner target/wasm32-unknown-unknown/debug/game-off.wasm

lint:
	cargo clippy

.PHONY: install run run-web lint
