# Oerview

CLI url converter into markdown syntac.

## Installation

[DL:URL2MD-rust](https://github.com/snamiki1212/alfred-url2md-rust/releases/download/v1.0.0/URL2MD-rust.alfredworkflow)

## How to: Alfred

```zsh
# INPUT  = https://www.rust-lang.org
# OUTPUT = [https://www.rust-lang.org](Rust Programming Language)
```

## How to: CLI

```zsh
$ url2md https://www.rust-lang.org
#=> [https://www.rust-lang.org](Rust Programming Language)
```

## Note: Build

```zsh
# build
$ cargo build --bin url2md --release
$ ls ./target/release/url2md

# alfred
$ cp ./target/release/url2md <alfred-dir>
# open alfred preference > right click > export
```

## LICENSE

MIT