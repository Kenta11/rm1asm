.PHONY: check documents test clean

check:
	cargo check
	cargo fmt
	cargo clippy

documents: README.md target/man/rm1asm.1 PKGBUILD

README.md: Cargo.toml script/build-a-file.py script/templates/README.md
	./script/build-a-file.py --helpmessage script/templates/README.md README.md

target/man/rm1asm.1: target/man/rm1asm.1.md
	pandoc --standalone -f markdown -t man $^ > $@

target/man/rm1asm.1.md: Cargo.toml script/build-a-file.py script/templates/rm1asm.1.md
	mkdir -p target/man
	./script/build-a-file.py script/templates/rm1asm.1.md target/man/rm1asm.1.md

PKGBUILD: Cargo.toml script/build-a-file.py script/templates/PKGBUILD
	./script/build-a-file.py script/templates/PKGBUILD PKGBUILD

test:
	cargo test
	./script/test.sh

clean:
	rm -rf README.md target PKGBUILD
