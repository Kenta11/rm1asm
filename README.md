# MICRO-1 assembler written in Rust

[![Tests](https://github.com/Kenta11/rm1asm/actions/workflows/main.yml/badge.svg)](https://github.com/Kenta11/rm1asm/actions/workflows/main.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

`rm1asm` is the assembler for MICRO-1, a tiny microprogram-controlled computer for educational purposes.

## Command-line options

```
$ rm1asm --help
rm1asm 1.0.3
MICRO-1 machine language assembler written in Rust

USAGE:
    rm1asm [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Sets output path

ARGS:
    <input>    source code
```

## Installing

### Cargo

```
cargo install rm1asm
```

### Packages

- Debian: https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm_1.0.3_amd64.deb
- RedHat: https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm-1.0.3-1.el7.x86_64.rpm
- Arch Linux: https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm-1.0.3-1-x86_64.pkg.tar.zst

### Tarbolls

- Windows (x64): https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm_windows.zip
- Linux (x64): https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm_linux.tar.gz
- macOS (x64): https://github.com/Kenta11/rm1asm/releases/download/v1.0.3/rm1asm_macos.tar.gz

## Reference

- 馬場敬信：マイクロプログラミング，昭晃堂（1985）

## Link

- simulator: [m1sim](https://github.com/kaien3/micro1)
- micro assembler: [rm1masm](https://github.com/Kenta11/rm1masm)
- cpu: [micro-alpha](https://github.com/Kenta11/micro-alpha)

## License

`rm1asm` is licensed under MIT license. See [LICENSE](LICENSE) for details.