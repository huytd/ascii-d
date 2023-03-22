patch: build
	comby -in-place 'getObject(arg0).font = :[1];' 'getObject(arg0).font = "16px SF Mono, ui-monospace, monospace"' pkg/ascii_d.js

build:
	wasm-pack build --target web --dev
