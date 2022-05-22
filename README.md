# randlib

Dependency-less random value generator using pointer addresses and time.

The generator is using the LCG algorithm seed by a pointer address multiplied by
UNIX_EPOCH.

# Features

- Functions returning random types:
	- u8, u16, u32, u64, u128
	- i8, i16, i32, i64, i128
	- usize
	- bool
- Get a random value in a range
- Shuffle elements of a vector
- Get a random element of a vector

# Usage

Basic usage (checkout the examples/usage.rs file for more examples)

```rust
use randlib::Rand;

fn main() {
    let mut rng = Rand::new();
	println!("random usize: {}", rng.rand());
	println!("random u8: {}", rng.rand_u8());
	println!("random i8: {}", rng.rand_i8());
	println!("random u16: {}", rng.rand_u16());
	println!("random i16: {}", rng.rand_i16());
	println!("random u32: {}", rng.rand_u32());
	println!("random i32: {}", rng.rand_i32());
	println!("random u64: {}", rng.rand_u64());
	println!("random i64: {}", rng.rand_i64());
	println!("random u128: {}", rng.rand_u128());
	println!("random i128: {}", rng.rand_i128());
	println!("random usize: {}", rng.rand_usize());
	println!("random bool: {:?}", rng.rand_bool());
}
```

# LICENSE

This project is distributed under MIT license.
