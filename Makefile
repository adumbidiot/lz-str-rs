.PHONY: build-wasm

# --reference-types

build-wasm:
	wasm-pack build --target nodejs bindings/lz-str-wasm
	cd bindings/lz-str-wasm && python inject-inline-js.py
	
build-wasm-browser:
	wasm-pack build --target web bindings/lz-str-wasm 
	cd bindings/lz-str-wasm && python inject-inline-js.py