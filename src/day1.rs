use std::collections::HashMap;

fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    
    for line in s.lines() {
        left.push(line[0..5].parse().unwrap());
        right.push(line[8..13].parse().unwrap());
    }
    
    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();
    
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);
    
    let right_count: HashMap<i32, i32> = right
        .iter()
        .fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    
    left.iter()
        .map(|&l| l * right_count.get(&l).copied().unwrap_or(0))
        .sum()
}
