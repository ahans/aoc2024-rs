#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    solve(&input, false)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    solve(&input, true)
}

fn solve(input: &str, part2: bool) -> i64 {
    let mut count = 0;

    let s = input.as_bytes();
    let mut i = 0;

    let mut operands = Vec::with_capacity(16);
    let mut results = Vec::with_capacity(256);
    let mut new_results = Vec::with_capacity(256);

    while i < s.len() && s[i].is_ascii_digit() {
        let mut result = 0i64;
        while s[i].is_ascii_digit() {
            result = result * 10 + (s[i] - b'0') as i64;
            i += 1;
        }
        i += 2;
        operands.clear();
        while s[i].is_ascii_digit() {
            let mut op = 0i64;
            while i < s.len() && s[i].is_ascii_digit() {
                op = op * 10 + (s[i] - b'0') as i64;
                i += 1;
            }
            operands.push(op);
            if i >= s.len() || s[i] != b' ' {
                break;
            }
            i += 1;
        }
        i += 1;

        results.clear();
        results.push(result);
        for op in operands[1..].iter().rev() {
            new_results.clear();
            for r in &results {
                if r - op >= 0 {
                    new_results.push(r - op);
                }
                if r % op == 0 {
                    new_results.push(r / op);
                }
                if part2 {
                    let npo10 = next_power_of_ten(*op);
                    if r % npo10 == *op {
                        new_results.push(r / npo10);
                    }
                }
            }
            std::mem::swap(&mut results, &mut new_results);
        }
        let op = operands[0];
        let npo10 = next_power_of_ten(op);
        for r in &results {
            if r - op == 0 || (r % op == 0 && r / op == 0) || (part2 && r % npo10 == op && r / npo10 == 0) {
                count += result;
                break;
            }
        }
    }

    count
}

#[inline]
fn next_power_of_ten(n: i64) -> i64 {
    let mut power = 10;
    while power <= n {
        power *= 10;
    }
    power
}
