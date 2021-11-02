setup:
	cp scripts/pre-commit .git/hooks
	chmod +x .git/hooks/pre-commit

run:
	cargo run
