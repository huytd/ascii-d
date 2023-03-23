dev: build patch copy-dev
	cd dist-dev && serve

patch: build
	comby -in-place 'getObject(arg0).font = :[1];' 'getObject(arg0).font = "16px SF Mono, ui-monospace, monospace"' pkg/ascii_d.js

build:
	wasm-pack build --target web --dev

release:
	wasm-pack build --target web

copy:
	mkdir -p dist && cp -rf assets dist/ && cp web-resources/* dist/ && cp ascii-d.js dist/ && cp index.html dist/ && cp -rf pkg dist/pkg && cp vercel.json dist/

copy-dev:
	mkdir -p dist-dev && cp -rf assets dist-dev/ && cp web-resources/* dist-dev/ && cp ascii-d.js dist-dev/ && cp index.html dist-dev/ && cp -rf pkg dist-dev/pkg

deploy: release patch copy
	cd dist && vercel --prod
