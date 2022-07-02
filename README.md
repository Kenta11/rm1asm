# MICRO-1 assembler written in Rust

[![Tests](https://github.com/Kenta11/rm1asm/actions/workflows/main.yml/badge.svg)](https://github.com/Kenta11/rm1asm/actions/workflows/main.yml)

`rm1asm` is the assembler for MICRO-1, a tiny microprogram-controlled computer for educational purposes.

## Command-line options

```
$ rm1asm --help
rm1asm 1.0.0
MICRO-1 machine language assembler written in Rust

USAGE:
    rm1asm [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    output file name

ARGS:
    <input>    source code

```

## Installing

### From source

Make sure you have installed the following:

- `git`
- stable Rust toolchain

```
cargo install --git https://github.com/Kenta11/rm1asm
```

### From binary

- Windows (x64): https://github.com/Kenta11/rm1asm/releases/download/v1.0.0/rm1asm_windows.zip
- Linux (x64)
    - RPM: https://github.com/Kenta11/rm1asm/releases/download/v1.0.0/rm1asm-1.0.0-1.el7.x86_64.rpm
    - DEB: https://github.com/Kenta11/rm1asm/releases/download/v1.0.0/rm1asm_1.0.0_amd64.deb
- macOS (x64): https://github.com/Kenta11/rm1asm/releases/download/v1.0.0/rm1asm_macos.tar.gz

## Reference

- 馬場敬信：マイクロプログラミング，昭晃堂（1985）

## License

`rm1asm` is dual-licensed under MIT license and Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
