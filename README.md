# Oerview

CLI url converter into markdown syntac.

![77156668-2343fc80-6ae3-11ea-8969-f0316bee8e94](https://user-images.githubusercontent.com/26793088/137627563-d28838ef-d35e-44dc-aefd-b111828c69ad.gif)

## Installation

[Download: URL2MD-rust](https://github.com/snamiki1212/alfred-url2md-rust/releases/download/v1.0.0/URL2MD-rust.alfredworkflow)

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
