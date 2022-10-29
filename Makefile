export WASM_FEATURES = 

.PHONY: build-wasm

# --reference-types

build-wasm:
	wasm-pack build --target nodejs bindings/lz-str-wasm --features=$(WASM_FEATURES)
	cd bindings/lz-str-wasm && python inject-inline-js.py
	
build-wasm-browser:
	wasm-pack build --target web bindings/lz-str-wasm --features=$(WASM_FEATURES)
	cd bindings/lz-str-wasm && python inject-inline-js.py