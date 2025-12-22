# opt\_level::OPT\_LEVEL

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/opt--level-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/opt-level)
[<img alt="crates.io" src="https://img.shields.io/crates/v/opt-level.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/opt-level)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-opt--level-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/opt-level)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/opt-level/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/opt-level/actions?query=branch%3Amaster)

Get the value of rustc's `-Copt-level=` flag at runtime.

Useful for sizing tests to run fewer iterations in slow build modes.

According to <https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level>
the possible values are:

- **0**: no optimizations
- **1**: basic optimizations
- **2**: some optimizations
- **3**: all optimizations
- **s**: optimize for binary size
- **z**: optimize for binary size, but also turn off loop vectorization

## Example

```rust
use rand::rngs::SmallRng;
use rand::{RngCore as _, SeedableRng as _};

const N: usize = if cfg!(miri) {
    500
} else if let b"0" = opt_level::OPT_LEVEL.as_bytes() {
    1_000_000
} else {
    100_000_000
};

#[test]
fn random_test() {
    let mut rng = SmallRng::from_os_rng();

    for _ in 0..N {
        let bits = rng.next_u64();
        ...
        assert_eq!(..., ...);
    }
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
