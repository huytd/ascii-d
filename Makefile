setup:
	cp scripts/pre-commit .git/hooks
	chmod +x .git/hooks/pre-commit

web:
	wasm-pack build --target web
	cp www/* pkg/

run:
	cargo run
