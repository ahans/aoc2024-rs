use fxhash::FxHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let numbers = parse(input);
    solve(&numbers, 25, 512)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let numbers = parse(input);
    solve(&numbers, 75, 3800)
}

fn parse(s: &str) -> Vec<u64> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn solve(initial: &[u64], blinks: u64, capacity: usize) -> u64 {
    let mut front =
        HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::<FxHasher>::new());
    let mut back =
        HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::<FxHasher>::new());

    for &stone in initial {
        *front.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        for (&stone, &count) in &front {
            if stone == 0 {
                *back.entry(1).or_insert(0) += count;
                continue;
            }

            let n = stone.ilog10() + 1;
            if n % 2 == 0 {
                let mask = 10u64.pow(n / 2);
                *back.entry(stone / mask).or_insert(0) += count;
                *back.entry(stone % mask).or_insert(0) += count;
                continue;
            }

            *back.entry(stone * 2024).or_insert(0) += count;
        }

        std::mem::swap(&mut front, &mut back);
        back.clear();
    }

    // Calculate sum of counts
    front.values().sum()
}
