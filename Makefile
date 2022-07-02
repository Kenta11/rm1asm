.PHONY: pre-commit test build-readme

pre-commit:
	cargo check
	cargo fmt
	cargo clippy

test:
	cargo test
	./script/test.sh

build-readme:
	./script/build-README.py
