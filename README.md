# `opt_level::OPT_LEVEL`

Get the value of rustc's `-Copt-level=` flag at runtime.

Useful for sizing tests to run fewer iterations in slow build modes.

According to <https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level>
the possible values are:

- `0`: no optimizations
- `1`: basic optimizations
- `2`: some optimizations
- `3`: all optimizations
- `s`: optimize for binary size
- `z`: optimize for binary size, but also turn off loop vectorization

<br>

## Example

```rust
use rand::rngs::SmallRng;
use rand::{RngCore as _, SeedableRng as _};

const N: usize = if cfg!(miri) {
    500
} else if let b"0" = opt_level::OPT_LEVEL.as_bytes() {
    10_000
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
