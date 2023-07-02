#![no_std]

mod const_buffer;

mod xxhash64;
pub use xxhash64::Xx64;
pub use xxhash64::xx_hash64;
pub use xxhash64::xx_hash64_seed;

mod xxhash32;
pub use xxhash32::xx_hash32;
pub use xxhash32::xx_hash32_seed;
