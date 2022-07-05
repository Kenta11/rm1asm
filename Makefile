.PHONY: pre-commit test build-readme clean

pre-commit:
	cargo check
	cargo fmt
	cargo clippy

test:
	cargo test
	./script/test.sh

build-documents: README.md target/man/rm1asm.1

README.md: Cargo.toml script/build-README.py script/templates/README.md
	./script/build-README.py

target/man/rm1asm.1: Cargo.toml script/build-man.py script/templates/rm1asm.1.md
	mkdir -p target/man
	./script/build-man.py
	pandoc --standalone -f markdown -t man target/man/rm1asm.1.md > target/man/rm1asm.1

clean:
	rm -rf README.md target
