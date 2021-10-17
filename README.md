# Oerview

CLI url converter into markdown syntac.

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

## Build

```zsh
$ cargo build --bin url2md --release
$ ls ./target/release/url2md
```

## LICENSE

MIT
