.PHONY: build-wasm

build-wasm:
	wasm-pack build --target nodejs bindings/lz-str-wasm