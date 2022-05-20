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
    pub fn new() -> Rand {
        let rand_ptr: *const i32 = &123;
        Rand {
            seed: (rand_ptr as u64).wrapping_mul(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        }
    }

    /// Generate random value (u32)
    pub fn rand(&mut self) -> u32 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u32
    }

    /// Generate random bool value
    pub fn rand_bool(&mut self) -> bool {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) % 2 == 0
    }

    /// Generate random u8 value
    pub fn rand_u8(&mut self) -> u8 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u8
    }

    /// Generate random u16 value
    pub fn rand_u16(&mut self) -> u16 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u16
    }

    /// Generate random u32 value
    pub fn rand_u32(&mut self) -> u32 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as u32
    }

    /// Generate random u64 value
    pub fn rand_u64(&mut self) -> u64 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32).wrapping_mul(PRIME_A) as u64
    }

    /// Generate random u128 value
    pub fn rand_u128(&mut self) -> u128 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        ((self.seed >> 32) as u128)
            .wrapping_mul(PRIME_A as u128)
            .wrapping_mul(PRIME_B as u128)
    }

    /// Generate random i8 value
    pub fn rand_i8(&mut self) -> i8 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i8
    }

    /// Generate random i16 value
    pub fn rand_i16(&mut self) -> i16 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i16
    }

    /// Generate random i32 value
    pub fn rand_i32(&mut self) -> i32 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32) as i32
    }

    /// Generate random i64 value
    pub fn rand_i64(&mut self) -> i64 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        (self.seed >> 32).wrapping_mul(PRIME_A) as i64
    }

    /// Generate random i128 value
    pub fn rand_i128(&mut self) -> i128 {
        self.seed = PRIME_A.wrapping_mul(self.seed).wrapping_add(PRIME_B);
        ((self.seed >> 32) as i128)
            .wrapping_mul(PRIME_A as i128)
            .wrapping_mul(PRIME_B as i128)
    }
}
