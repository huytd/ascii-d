setup:
	cp scripts/pre-commit .git/hooks
	chmod +x .git/hooks/pre-commit

build:
	wasm-pack build --target web --dev