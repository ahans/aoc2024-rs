#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    solve::<0>(input)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    solve::<10_000_000_000_000>(input)
}

fn solve<const OFFSET: i64>(input: &str) -> i64 {
    let mut i = 0;
    let s = input.as_bytes();
    let mut result = 0;
    unsafe {
        for _ in 0..320 {
            i += 12;
            let xa: i64 =
                ((*s.get_unchecked(i) - b'0') * 10 + *s.get_unchecked(i + 1) - b'0') as i64;
            i += 2 + 4;
            let ya: i64 =
                ((*s.get_unchecked(i) - b'0') * 10 + *s.get_unchecked(i + 1) - b'0') as i64;
            i += 2 + 1 + 12;
            let xb: i64 =
                ((*s.get_unchecked(i) - b'0') * 10 + *s.get_unchecked(i + 1) - b'0') as i64;
            i += 2 + 4;
            let yb: i64 =
                ((*s.get_unchecked(i) - b'0') * 10 + *s.get_unchecked(i + 1) - b'0') as i64;
            i += 2 + 1 + 9;
            let mut xt: i64 = 0;
            while s.get_unchecked(i).is_ascii_digit() {
                xt = 10 * xt + (*s.get_unchecked(i) - b'0') as i64;
                i += 1;
            }
            i += 4;
            let mut yt: i64 = 0;
            while i < s.len() && s.get_unchecked(i).is_ascii_digit() {
                yt = 10 * yt + (*s.get_unchecked(i) - b'0') as i64;
                i += 1;
            }
            i += 2;

            xt += OFFSET;
            yt += OFFSET;

            let denominator = yt * xb - yb * xt;
            let divisor = xb * ya - xa * yb;
            if denominator % divisor == 0 {
                let a = denominator / divisor;
                let b = (xt - xa * a) / xb;
                result += 3 * a + b;
            }
        }
    }
    result
}
