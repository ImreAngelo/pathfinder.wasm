.phony: all build

all: build

build: 
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/pathfinding.wasm --out-dir ../build --target web