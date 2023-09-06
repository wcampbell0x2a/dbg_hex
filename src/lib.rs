//! # dbg_hex
//! display dbg result in hexadecimal `{:#x?}` format.
//!
//! # usage
//! Replace `dbg!()` with `dbg_hex!()`
//!
//! # example
//! ```rust, no_run
//! use dbg_hex::dbg_hex;
//! dbg_hex!(0x16 + 0x16);
//! ```
//!
//! output
//! ```text
//! [src/lib.rs:38] 0x16 + 0x16 = 0x2c
//! ```
#[macro_export]
macro_rules! dbg_hex {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:#x?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_hex!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        dbg_hex!(0x16 + 0x16);
    }

    #[test]
    fn it_works_multiple() {
        dbg_hex!(0x16 + 0x16, 0x32 - 0x16);
    }
}
