# dbg_hex
display dbg result in hexadecimal `{:#x?}` format.

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
