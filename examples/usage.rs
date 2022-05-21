use randlib::Rand;

fn main() {
    let mut rng = Rand::new();
    for _ in 0..3 {
        println!("usize MAX:\t{}\t\t\t RNG: {}", usize::MAX, rng.rand());
        println!("u8 MAX:\t\t{}\t\t\t\t\t RNG: {}", u8::MAX, rng.rand_u8());
        println!("i8 MAX:\t\t{}\t\t\t\t\t RNG: {}", i8::MAX, rng.rand_i8());
        println!("u16 MAX:\t{}\t\t\t\t\t RNG: {}", u16::MAX, rng.rand_u16());
        println!("i16 MAX:\t{}\t\t\t\t\t RNG: {}", i16::MAX, rng.rand_i16());
        println!("u32 MAX:\t{}\t\t\t\t RNG: {}", u32::MAX, rng.rand_u32());
        println!("i32 MAX:\t{}\t\t\t\t RNG: {}", i32::MAX, rng.rand_i32());
        println!("u64 MAX:\t{}\t\t\t RNG: {}", u64::MAX, rng.rand_u64());
        println!("i64 MAX:\t{}\t\t\t RNG: {}", i64::MAX, rng.rand_i64());
        println!("u128 MAX:\t{}\t RNG: {}", u128::MAX, rng.rand_u128());
        println!("i128 MAX:\t{}\t RNG: {}", i128::MAX, rng.rand_i128());
        println!("bool RNG:\t{:?}", rng.rand_bool());
    }
}
