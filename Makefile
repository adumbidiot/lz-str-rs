.PHONY: build-wasm

# --reference-types

build-wasm:
	wasm-pack build --target nodejs bindings/lz-str-wasm
	
build-wasm-browser:
	wasm-pack build --target web bindings/lz-str-wasm 