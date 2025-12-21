//! Get the value of rustc's `-Copt-level=` flag at runtime.
//!
//! Useful for sizing tests to run fewer iterations in slow build modes.
//!
//! According to <https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level>
//! the possible values are:
//!
//! - `0`: no optimizations
//! - `1`: basic optimizations
//! - `2`: some optimizations
//! - `3`: all optimizations
//! - `s`: optimize for binary size
//! - `z`: optimize for binary size, but also turn off loop vectorization
//!
//! # Example
//!
//! ```
//! use rand::rngs::SmallRng;
//! use rand::{RngCore as _, SeedableRng as _};
//!
//! const N: usize = if cfg!(miri) {
//!     500
//! } else if let b"0" = opt_level::OPT_LEVEL.as_bytes() {
//!     10_000
//! } else {
//!     100_000_000
//! };
//!
//! #[test]
//! fn random_test() {
//!     let mut rng = SmallRng::from_os_rng();
//!
//!     for _ in 0..N {
//!         let bits = rng.next_u64();
//! # const _: &str = stringify! {
//!         ...
//!         assert_eq!(..., ...);
//! # };
//!     }
//! }
//! ```

#![no_std]

pub const OPT_LEVEL: &str = include_str!(concat!(env!("OUT_DIR"), "/opt-level"));
