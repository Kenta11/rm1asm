# Maintainer: Kenta Arai @isKenta14
pkgname=rm1asm
pkgver=1.0.1
pkgrel=1
makedepends=()
arch=('x86_64')
pkgdesc="MICRO-1 assembler written in Rust"
license=('MIT')

build() {
    cd $startdir
    cargo build --release
    make target/man/rm1asm.1
}

package() {
    install -Dm755 $startdir/target/release/rm1asm $pkgdir/usr/bin/rm1asm
    install -Dm644 $startdir/target/man/rm1asm.1 $pkgdir/usr/share/man/man1/rm1asm.1
    install -Dm644 $startdir/completions/rm1asm $pkgdir/usr/share/bash-completion/completions/rm1asm
    install -Dm644 $startdir/completions/_rm1asm $pkgdir/usr/share/zsh/site-functions/_rm1asm
    install -Dm644 $startdir/LICENSE $pkgdir/usr/share/licenses/$pkgname/LICENSE
}
