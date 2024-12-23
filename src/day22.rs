use rayon::prelude::*;
use std::simd::prelude::*;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    let numbers: Vec<u32> = input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    numbers
        .par_chunks(4)
        .map(|chunk| {
            if chunk.len() == 4 {
                let mut v4 = u32x4::from_slice(chunk);
                for _ in 0..2000 {
                    v4 = (v4 ^ (v4 << u32x4::splat(6))) & u32x4::splat(0xffffff);
                    v4 = (v4 ^ (v4 >> u32x4::splat(5))) & u32x4::splat(0xffffff);
                    v4 = (v4 ^ (v4 << u32x4::splat(11))) & u32x4::splat(0xffffff);
                }
                v4.reduce_sum() as u64
            } else {
                chunk
                    .iter()
                    .map(|value| {
                        let mut v: u32 = *value;
                        for _ in 0..2000 {
                            v = hash(v);
                        }
                        v as u64
                    })
                    .sum()
            }
        })
        .sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u64 {
    let s = input.as_bytes();
    let mut i = 0;
    let mut p2: u64 = 0;
    let mut bananas = vec![0u32; 130321];
    let mut seen = vec![0u16; 130321];
    let mut count = 0;
    while i < s.len() {
        let mut v: u32 = 0;
        while i < s.len() && s[i].is_ascii_digit() {
            v = 10 * v + (s[i] - b'0') as u32;
            i += 1;
        }

        let mut last4 = [0i32; 4];

        let mut v_last = v % 10;
        let mut index: usize = 0;
        for j in 0..2000 {
            let w = hash(v);
            let w_last = w % 10;

            let diff = w_last as i32 - v_last as i32;

            if j >= 3 {
                if j > 3 {
                    index = index - 19 * 19 * 19 * (last4[j % 4] + 9) as usize;
                }
                index = index * 19 + (diff + 9) as usize;
                if seen[index] < count + 1 {
                    seen[index] = count + 1;
                    bananas[index] += w_last;
                    if bananas[index] as u64 > p2 {
                        p2 = bananas[index] as u64;
                    }
                }
            } else {
                index = 19 * index + (diff + 9) as usize;
            }

            last4[j % 4] = diff;

            v = w;
            v_last = w_last;
        }
        i += 1;

        count += 1;
    }
    p2
}

#[inline(always)]
fn hash(v: u32) -> u32 {
    let v = (v ^ (v << 6)) & 0xffffff;
    let v = (v ^ (v >> 5)) & 0xffffff;
    (v ^ (v << 11)) & 0xffffff
}
