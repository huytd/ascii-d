patch: build
	comby -in-place 'getObject(arg0).font = :[1];' 'getObject(arg0).font = "16px SF Mono, ui-monospace, monospace"' pkg/ascii_d.js

build:
	wasm-pack build --target web --dev

release:
	wasm-pack build --target web

deploy: release patch
	mkdir -p dist && cp ascii-d.js dist/ && cp index.html dist/ && cp -rf pkg dist/pkg && cp vercel.json dist/ && cd dist && vercel --prod
