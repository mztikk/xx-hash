use crate::{const_buffer::Buffer, xxhash32::get_32bits};

const XXH_PRIME64_1: u64 = 0x9E3779B185EBCA87;
const XXH_PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const XXH_PRIME64_3: u64 = 0x165667B19E3779F9;
const XXH_PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const XXH_PRIME64_5: u64 = 0x27D4EB2F165667C5;

const CHUNK_SIZE64: usize = core::mem::size_of::<u64>() * 4;

#[inline(always)]
const fn get_64bits(input: &[u8], cursor: usize) -> u64 {
    input[cursor] as u64
        | (input[cursor + 1] as u64) << 8
        | (input[cursor + 2] as u64) << 16
        | (input[cursor + 3] as u64) << 24
        | (input[cursor + 4] as u64) << 32
        | (input[cursor + 5] as u64) << 40
        | (input[cursor + 6] as u64) << 48
        | (input[cursor + 7] as u64) << 56
}

#[inline(always)]
const fn round64(acc: u64, input: u64) -> u64 {
    acc.wrapping_add(input.wrapping_mul(XXH_PRIME64_2))
        .rotate_left(31)
        .wrapping_mul(XXH_PRIME64_1)
}

#[inline(always)]
const fn merge_round64(mut acc: u64, mut val: u64) -> u64 {
    val = round64(0, val);
    acc ^= val;
    acc = acc.wrapping_mul(XXH_PRIME64_1).wrapping_add(XXH_PRIME64_4);
    acc
}

#[inline(always)]
const fn avalanche64(mut input: u64) -> u64 {
    input ^= input >> 33;
    input = input.wrapping_mul(XXH_PRIME64_2);
    input ^= input >> 29;
    input = input.wrapping_mul(XXH_PRIME64_3);
    input ^= input >> 32;
    input
}

type Buffer64 = Buffer<CHUNK_SIZE64>;

pub struct Xx64 {
    seed: u64,
    v1: u64,
    v2: u64,
    v3: u64,
    v4: u64,
    buffer: Buffer64,
    total_len: u64,
}

impl Xx64 {
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            seed,
            v1: seed.wrapping_add(XXH_PRIME64_1).wrapping_add(XXH_PRIME64_2),
            v2: seed.wrapping_add(XXH_PRIME64_2),
            v3: seed,
            v4: seed.wrapping_sub(XXH_PRIME64_1),
            buffer: Buffer64::default(),
            total_len: 0,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn read_chunk(&mut self, input: &[u8]) {
        let mut remaining = input;
        while remaining.len() > 0 {
            remaining = self.buffer.consume(remaining);
            self.total_len += self.buffer.len as u64;
            if self.buffer.is_full() {
                let mut cursor = 0;
                let data = self.buffer.data();
                self.v1 = round64(self.v1, get_64bits(data, cursor));
                cursor += 8;
                self.v2 = round64(self.v2, get_64bits(data, cursor));
                cursor += 8;
                self.v3 = round64(self.v3, get_64bits(data, cursor));
                cursor += 8;
                self.v4 = round64(self.v4, get_64bits(data, cursor));
                self.buffer.len = 0;
            }
        }
    }

    pub const fn finalize(&self) -> u64 {
        let mut result: u64;
        if self.total_len >= (CHUNK_SIZE64 as u64) {
            result = self.v1.rotate_left(1).wrapping_add(
                self.v2.rotate_left(7).wrapping_add(
                    self.v3
                        .rotate_left(12)
                        .wrapping_add(self.v4.rotate_left(18)),
                ),
            );

            result = merge_round64(result, self.v1);
            result = merge_round64(result, self.v2);
            result = merge_round64(result, self.v3);
            result = merge_round64(result, self.v4);
        } else {
            result = self.seed.wrapping_add(XXH_PRIME64_5);
        }

        result = result.wrapping_add(self.total_len);

        let mut len = self.buffer.len & 31;
        let mut cursor = 0;

        let data = self.buffer.data();

        while len >= 8 {
            result = (result ^ round64(0, get_64bits(data, cursor)))
                .rotate_left(27)
                .wrapping_mul(XXH_PRIME64_1)
                .wrapping_add(XXH_PRIME64_4);

            cursor += 8;
            len -= 8;
        }

        while len >= 4 {
            result = (result ^ (get_32bits(data, cursor) as u64).wrapping_mul(XXH_PRIME64_1))
                .rotate_left(23)
                .wrapping_mul(XXH_PRIME64_2)
                .wrapping_add(XXH_PRIME64_3);

            cursor += 4;
            len -= 4;
        }

        while len > 0 {
            result = (result ^ (data[cursor] as u64).wrapping_mul(XXH_PRIME64_5))
                .rotate_left(11)
                .wrapping_mul(XXH_PRIME64_1);

            cursor += 1;
            len -= 1;
        }

        avalanche64(result)
    }
}

impl Default for Xx64 {
    fn default() -> Self {
        Self::new_with_seed(0)
    }
}

#[inline(always)]
pub const fn xx_hash64(input: &[u8]) -> u64 {
    xx_hash64_seed(input, 0)
}

pub const fn xx_hash64_seed(input: &[u8], seed: u64) -> u64 {
    let mut result: u64;
    let mut cursor = 0;

    if input.len() >= CHUNK_SIZE64 {
        let mut v1 = seed.wrapping_add(XXH_PRIME64_1).wrapping_add(XXH_PRIME64_2);
        let mut v2 = seed.wrapping_add(XXH_PRIME64_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(XXH_PRIME64_1);

        loop {
            v1 = round64(v1, get_64bits(input, cursor));
            cursor += 8;
            v2 = round64(v2, get_64bits(input, cursor));
            cursor += 8;
            v3 = round64(v3, get_64bits(input, cursor));
            cursor += 8;
            v4 = round64(v4, get_64bits(input, cursor));
            cursor += 8;

            if (input.len() - cursor) < CHUNK_SIZE64 {
                break;
            }
        }

        result = v1.rotate_left(1).wrapping_add(
            v2.rotate_left(7)
                .wrapping_add(v3.rotate_left(12).wrapping_add(v4.rotate_left(18))),
        );

        result = merge_round64(result, v1);
        result = merge_round64(result, v2);
        result = merge_round64(result, v3);
        result = merge_round64(result, v4);
    } else {
        result = seed.wrapping_add(XXH_PRIME64_5);
    }

    result = result.wrapping_add(input.len() as u64);

    let mut len = input.len() & 31;

    while len >= 8 {
        result = (result ^ round64(0, get_64bits(input, cursor)))
            .rotate_left(27)
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4);

        cursor += 8;
        len -= 8;
    }

    while len >= 4 {
        result = (result ^ (get_32bits(input, cursor) as u64).wrapping_mul(XXH_PRIME64_1))
            .rotate_left(23)
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3);

        cursor += 4;
        len -= 4;
    }

    while len > 0 {
        result = (result ^ (input[cursor] as u64).wrapping_mul(XXH_PRIME64_5))
            .rotate_left(11)
            .wrapping_mul(XXH_PRIME64_1);

        cursor += 1;
        len -= 1;
    }

    avalanche64(result)
}
