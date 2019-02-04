# UFO (Unsigned Float Objects)
 
[![Latest Version](https://img.shields.io/crates/v/ufo.svg)](https://crates.io/crates/ufo) 
 
  This crate provides unsigned floats by defining two wrappers - a UF32 and UF64.
  The type is equivalent to storing a positive number of the underlying type (f32 or f64)
  and no optimisations have been carried out to use the sign bit to store another bit of
  the data.
 
  ## Installation
 
  To use this as a dependency, add it to your Cargo.toml file:
 
  ```toml
  ufo = "0.1"
  ```
 
  If you need serialization, enable the serde feature
  ```toml
  ufo = { version = "0.1", features = ["serde"] }
  ```
 
  There is also a nightly feature that takes advantage of the TryFrom/TryInto traits
 
  ## Usage
 
  ```rust
  use ufo::Uf32;
 
  fn main() {
      let a = Uf32::try_new(1.0).expect("invalid number");
      let b = Uf32::try_new(2.0).expect("invalid number");
      assert_eq!(a + b, Uf32::try_new(3.0).expect("invalid number"))
  }
  ```
## Contribution

If you want to suggest any new feature or report a bug, you can open an issue here or drop in a pull request directly.

Right now, I still need to tests for most of the functions, so you can test it locally by running:

```bash
cargo test 
```

When submitting a Pull request, run `cargo fmt` on the latest nightly before committing.

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
 