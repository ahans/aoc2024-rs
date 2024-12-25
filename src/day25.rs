use std::simd::prelude::*;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let s = input.as_bytes();
    let mut cur_key = u32x8::splat(b'#' as u32);
    let mut cur_key_count = 0;
    let mut i = 0;
    while i < s.len() {
        let is_lock = s[i] == b'#';
        i += 5 + 1;
        let values = u8x32::from_slice(&s[i..i + 32]);
        i += 36;
        i += 1;
        let mask = values.simd_eq(u8x32::splat(b'#'));
        if is_lock {
            locks.push(mask.to_bitmask() as u32);
        } else {
            cur_key[cur_key_count] = mask.to_bitmask() as u32;
            cur_key_count += 1;
            if cur_key_count == 8 {
                keys.push(cur_key);
                cur_key = u32x8::splat(b'#' as u32);
                cur_key_count = 0;
            }
        }
    }
    if cur_key_count > 0 {
        keys.push(cur_key);
    }

    let mut p1 = 0;
    for lock in locks {
        let lock_vec = u32x8::splat(lock);
        for key_vec in &keys {
            let overlap = lock_vec & key_vec;
            p1 += overlap.simd_eq(u32x8::splat(0)).to_bitmask().count_ones();
        }
    }
    // for key_vec in &keys {
    //     for lock in &locks {
    //         let overlap = u32x4::splat(*lock) & key_vec;
    //         p1 += overlap.simd_eq(u32x4::splat(0)).to_bitmask().count_ones();
    //     }
    // }
    p1
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> u64 {
    0
}
