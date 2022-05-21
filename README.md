# randlib

Dependency-less random value generator using pointer addresses and time.

The generator is using the LCG algorithm seed by a pointer address multiplied by
UNIX_EPOCH.

# Usage

Basic usage

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
