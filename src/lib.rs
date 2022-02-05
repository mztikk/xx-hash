use std::mem;

const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;

const CHUNK_SIZE: usize = mem::size_of::<u32>() * 4;

#[inline(always)]
const fn get_32bits(input: &[u8], cursor: usize) -> u32 {
    input[cursor] as u32
        | (input[cursor + 1] as u32) << 8
        | (input[cursor + 2] as u32) << 16
        | (input[cursor + 3] as u32) << 24
}

#[inline(always)]
const fn round(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME32_2))
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

#[inline(always)]
const fn avalanche(mut input: u32) -> u32 {
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
            v1 = round(v1, get_32bits(input, cursor));
            cursor += mem::size_of::<u32>();
            v2 = round(v2, get_32bits(input, cursor));
            cursor += mem::size_of::<u32>();
            v3 = round(v3, get_32bits(input, cursor));
            cursor += mem::size_of::<u32>();
            v4 = round(v4, get_32bits(input, cursor));
            cursor += mem::size_of::<u32>();

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
        cursor += mem::size_of::<u32>();
        len -= mem::size_of::<u32>();
        result = result.rotate_left(17).wrapping_mul(PRIME32_4);
    }

    while len > 0 {
        result = result.wrapping_add((input[cursor] as u32).wrapping_mul(PRIME32_5));
        cursor += mem::size_of::<u8>();
        len -= mem::size_of::<u8>();
        result = result.rotate_left(11).wrapping_mul(PRIME32_1);
    }

    avalanche(result)
}
