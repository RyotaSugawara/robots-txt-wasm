.PHONY: wasm

wasm:
	wasm-pack build --target web -d public --no-typescript --no-pack
