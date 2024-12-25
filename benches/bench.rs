#![feature(portable_simd)]

use std::{fs::read_to_string, hint::black_box};

use aoc2024_rs::day25::part1;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day25(c: &mut Criterion) {
    let s = read_to_string("./input/2024/day25.txt").unwrap();
    let s = s.as_str();

    c.bench_function("day25 part1", |b| b.iter(|| part1(black_box(s))));

    // assert_eq!(part1(s).to_string(), read_to_string("./outputs/20p1.txt").unwrap());
    // assert_eq!(part2(s).to_string(), read_to_string("./outputs/20p2.txt").unwrap());
}

criterion_group!(benches, day25);
criterion_main!(benches);
