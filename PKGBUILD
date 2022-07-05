# Maintainer: Kenta Arai @isKenta14
pkgname=rm1asm
pkgver=1.0.0
pkgrel=1
makedepends=()
arch=('x86_64')
pkgdesc="MICRO-1 assembler written in Rust"
license=('MIT OR Apache-2.0')

build() {
    cd $startdir
    cargo build --release
    make build-documents
}

package() {
    install -Dm755 $startdir/target/release/rm1asm $pkgdir/usr/bin/rm1asm
    install -Dm644 $startdir/target/man/rm1asm.1 $pkgdir/usr/share/man/man1/rm1asm.1
    install -Dm644 $startdir/completions/rm1asm $pkgdir/usr/share/bash-completion/completions/rm1asm
    install -Dm644 $startdir/completions/_rm1asm $pkgdir/usr/share/zsh/site-functions/_rm1asm
    install -Dm644 $startdir/LICENSE-APACHE $pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE
    install -Dm644 $startdir/LICENSE-MIT $pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT
}