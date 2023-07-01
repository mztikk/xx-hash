#![no_std]

const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;

const XXH_PRIME64_1: u64 = 0x9E3779B185EBCA87;
const XXH_PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const XXH_PRIME64_3: u64 = 0x165667B19E3779F9;
const XXH_PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const XXH_PRIME64_5: u64 = 0x27D4EB2F165667C5;

const CHUNK_SIZE: usize = core::mem::size_of::<u32>() * 4;
const CHUNK_SIZE64: usize = core::mem::size_of::<u64>() * 4;

#[inline(always)]
const fn get_32bits(input: &[u8], cursor: usize) -> u32 {
    input[cursor] as u32
        | (input[cursor + 1] as u32) << 8
        | (input[cursor + 2] as u32) << 16
        | (input[cursor + 3] as u32) << 24
}

#[inline(always)]
const fn round32(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME32_2))
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

#[inline(always)]
const fn avalanche32(mut input: u32) -> u32 {
    input ^= input >> 15;
    input = input.wrapping_mul(PRIME32_2);
    input ^= input >> 13;
    input = input.wrapping_mul(PRIME32_3);
    input ^= input >> 16;
    input
}

#[inline(always)]
pub const fn xx_hash32(input: &[u8]) -> u32 {
    xx_hash32_seed(input, 0)
}

pub const fn xx_hash32_seed(input: &[u8], seed: u32) -> u32 {
    let mut result = input.len() as u32;
    let mut cursor = 0;

    if input.len() >= CHUNK_SIZE {
        let mut v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2 = seed.wrapping_add(PRIME32_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME32_1);

        loop {
            v1 = round32(v1, get_32bits(input, cursor));
            cursor += core::mem::size_of::<u32>();
            v2 = round32(v2, get_32bits(input, cursor));
            cursor += core::mem::size_of::<u32>();
            v3 = round32(v3, get_32bits(input, cursor));
            cursor += core::mem::size_of::<u32>();
            v4 = round32(v4, get_32bits(input, cursor));
            cursor += core::mem::size_of::<u32>();

            if (input.len() - cursor) < CHUNK_SIZE {
                break;
            }
        }

        result = result.wrapping_add(
            v1.rotate_left(1).wrapping_add(
                v2.rotate_left(7)
                    .wrapping_add(v3.rotate_left(12).wrapping_add(v4.rotate_left(18))),
            ),
        );
    } else {
        result = result.wrapping_add(seed.wrapping_add(PRIME32_5));
    }

    let mut len = input.len() - cursor;

    while len >= 4 {
        result = result.wrapping_add(get_32bits(input, cursor).wrapping_mul(PRIME32_3));
        cursor += core::mem::size_of::<u32>();
        len -= core::mem::size_of::<u32>();
        result = result.rotate_left(17).wrapping_mul(PRIME32_4);
    }

    while len > 0 {
        result = result.wrapping_add((input[cursor] as u32).wrapping_mul(PRIME32_5));
        cursor += core::mem::size_of::<u8>();
        len -= core::mem::size_of::<u8>();
        result = result.rotate_left(11).wrapping_mul(PRIME32_1);
    }

    avalanche32(result)
}

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
            cursor += core::mem::size_of::<u64>();
            v2 = round64(v2, get_64bits(input, cursor));
            cursor += core::mem::size_of::<u64>();
            v3 = round64(v3, get_64bits(input, cursor));
            cursor += core::mem::size_of::<u64>();
            v4 = round64(v4, get_64bits(input, cursor));
            cursor += core::mem::size_of::<u64>();

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
        let k1 = round64(0, get_64bits(input, cursor));
        cursor += core::mem::size_of::<u64>();
        result ^= k1;
        result = result
            .rotate_left(27)
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4);
        len -= core::mem::size_of::<u64>();
    }

    while len >= 4 {
        result ^= (get_32bits(input, cursor) as u64).wrapping_mul(XXH_PRIME64_1);
        cursor += core::mem::size_of::<u32>();
        result = result
            .rotate_left(23)
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3);
        len -= core::mem::size_of::<u32>();
    }

    while len > 0 {
        result ^= (input[cursor] as u64).wrapping_mul(XXH_PRIME64_5);
        result = result.rotate_left(11).wrapping_mul(XXH_PRIME64_1);
        cursor += core::mem::size_of::<u8>();
        len -= core::mem::size_of::<u8>();
    }

    avalanche64(result)
}
