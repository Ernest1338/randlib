//! # randlib
//!
//! Dependency-less random value generator using pointer addresses and time
//!
//! The generator is using the LCG algorithm seed by a pointer address multiplied by
//! UNIX_EPOCH.
//!
//! # Usage
//!
//! Basic usage
//! 
//! ```rust
//! use randlib::Rand;
//! 
//! fn main() {
//!     let mut rng = Rand::new();
//!     println!("random usize: {}", rng.rand());
//!     println!("random u8: {}", rng.rand_u8());
//!     println!("random i8: {}", rng.rand_i8());
//!     println!("random u16: {}", rng.rand_u16());
//!     println!("random i16: {}", rng.rand_i16());
//!     println!("random u32: {}", rng.rand_u32());
//!     println!("random i32: {}", rng.rand_i32());
//!     println!("random u64: {}", rng.rand_u64());
//!     println!("random i64: {}", rng.rand_i64());
//!     println!("random u128: {}", rng.rand_u128());
//!     println!("random i128: {}", rng.rand_i128());
//!     println!("random bool: {:?}", rng.rand_bool());
//! }
//! ```
//! 
//! # LICENSE
//!
//! This project is distributed under MIT license.


// Big prime numbers
const PRIME_A: u64 = 1442695040888963407;
const PRIME_B: u64 = 6364136223846793005;

/// Rand Struct
pub struct Rand {
    seed: u64,
}

/// Default Rand implementation
impl Default for Rand {
    fn default() -> Self {
        Self::new()
    }
}

/// Rand implementation
impl Rand {
    /// Create new Rand object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// ```
    pub fn new() -> Rand {
        let rand_ptr: *const i32 = &123;
        Rand {
            // Seed the RNG with a pointer address multiplied by UNIX_EPOCH
            seed: (rand_ptr as u64).wrapping_mul(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        }
    }

    /// Generate random value (usize)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_number: usize = rng.rand();
    /// ```
    pub fn rand(&mut self) -> usize {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32).wrapping_mul(PRIME_A) as usize
    }

    /// Generate random bool value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_bool: bool = rng.rand_bool();
    /// ```
    pub fn rand_bool(&mut self) -> bool {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) % 2 == 0
    }

    /// Generate random u8 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_u8: u8 = rng.rand_u8();
    /// ```
    pub fn rand_u8(&mut self) -> u8 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u8
    }

    /// Generate random u16 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_u16: u16 = rng.rand_u16();
    /// ```
    pub fn rand_u16(&mut self) -> u16 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u16
    }

    /// Generate random u32 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_u32: u32 = rng.rand_u32();
    /// ```
    pub fn rand_u32(&mut self) -> u32 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u32
    }

    /// Generate random u64 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_u64: u64 = rng.rand_u64();
    /// ```
    pub fn rand_u64(&mut self) -> u64 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32).wrapping_mul(PRIME_A) as u64
    }

    /// Generate random u128 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_u128: u128 = rng.rand_u128();
    /// ```
    pub fn rand_u128(&mut self) -> u128 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        ((self.seed >> 32) as u128)
            .wrapping_mul(PRIME_A as u128)
            .wrapping_mul(PRIME_B as u128)
    }

    /// Generate random i8 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_i8: i8 = rng.rand_i8();
    /// ```
    pub fn rand_i8(&mut self) -> i8 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i8
    }

    /// Generate random i16 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_i16: i16 = rng.rand_i16();
    /// ```
    pub fn rand_i16(&mut self) -> i16 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i16
    }

    /// Generate random i32 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_i32: i32 = rng.rand_i32();
    /// ```
    pub fn rand_i32(&mut self) -> i32 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i32
    }

    /// Generate random i64 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_i64: i64 = rng.rand_i64();
    /// ```
    pub fn rand_i64(&mut self) -> i64 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32).wrapping_mul(PRIME_A) as i64
    }

    /// Generate random i128 value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use randlib::Rand;
    ///
    /// let mut rng = Rand::new();
    /// let random_i128: i128 = rng.rand_i128();
    /// ```
    pub fn rand_i128(&mut self) -> i128 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        ((self.seed >> 32) as i128)
            .wrapping_mul(PRIME_A as i128)
            .wrapping_mul(PRIME_B as i128)
    }
}
