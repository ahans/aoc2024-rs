use std::simd::prelude::*;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { solve(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512f"))]
unsafe fn solve(input: &str) -> u32 {
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let s = input.as_bytes();
    let mut i = 0;
    unsafe {
        while i < input.len() {
            let is_lock = s[i] == b'#';
            let values = u8x32::from_slice(&s[i + 6..i + 6 + 32]);
            i += 6 + 36 + 1;
            let mask = values.simd_eq(u8x32::splat(b'#'));
            if is_lock {
                locks.push(mask.to_bitmask() as u32);
            } else {
                keys.push(mask.to_bitmask() as u32);
            }
        }
    }
    for _ in 0..(keys.len() / 16 + 1) * 16 - keys.len() {
        keys.push(0xffffffff);
    }

    locks
        .into_iter()
        .map(|lock| {
            let lock_vec = u32x16::splat(lock);
            keys.chunks_exact(16)
                .map(|chunk| {
                    let key_vec = u32x16::from_slice(chunk);
                    let overlap = lock_vec & key_vec;
                    overlap.simd_eq(u32x16::splat(0)).to_bitmask().count_ones()
                })
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> u64 {
    0
}
