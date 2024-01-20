dbg_hex
===========================

[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/dbg_hex-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/dbg_hex)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dbg_hex.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dbg_hex)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dbg_hex-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dbg_hex)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/dbg_hex/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/dbg_hex/actions?query=branch%3Amaster)

Display dbg result in hexadecimal `{:#x?}` format.

# usage
Replace `dbg!()` with `dbg_hex!()`

# example
```rust, no_run
use dbg_hex::dbg_hex;
dbg_hex!(0x16 + 0x16);
```

output
```text
[src/lib.rs:38] 0x16 + 0x16 = 0x2c
```
