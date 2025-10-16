.phony: all build clean publish

all: build

build: 
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/pathfinding.wasm --out-dir ./dist --target bundler

clean:
	rm -r dist
	rm -r target

publish:
	npm version patch
	npm publish