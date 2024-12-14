use std::simd::prelude::*;

const W: i16 = 101;
const H: i16 = 103;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    let mut i = 0;
    let s = input.as_bytes();
    while i < s.len() {
        i += 2;
        let mut x = (s[i] - b'0') as i32;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            x = 10 * x + (s[i] - b'0') as i32;
            i += 1;
        }
        i += 1;
        let mut y = (s[i] - b'0') as i32;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            y = 10 * y + (s[i] - b'0') as i32;
            i += 1;
        }
        i += 3;
        let mut f = 1;
        if s[i] == b'-' {
            f = -1;
            i += 1;
        }
        let mut dx = (s[i] - b'0') as i32;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            dx = 10 * dx + (s[i] - b'0') as i32;
            i += 1;
        }
        dx = f * dx;
        i += 1;
        if s[i] == b'-' {
            f = -1;
            i += 1;
        } else {
            f = 1;
        }
        let mut dy = (s[i] - b'0') as i32;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            dy = 10 * dy + (s[i] - b'0') as i32;
            i += 1;
        }
        dy = f * dy;
        i += 1;

        let x = (x + 100 * dx).rem_euclid(W as i32);
        let y = (y + 100 * dy).rem_euclid(H as i32);

        if (x == (W as i32) / 2) || (y == (H as i32) / 2) {
            continue;
        }
        let left = x < (W as i32 / 2);
        let top = y < (H as i32 / 2);
        if top {
            if left {
                tl += 1;
            } else {
                tr += 1;
            }
        } else {
            if left {
                bl += 1;
            } else {
                br += 1;
            }
        }
    }

    tl * tr * bl * br
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let (x, dx, y, dy) = parse(input);

    let col_step = 'outer: {
        for step in 0..W {
            let mut col_sums = [0i16; W as usize];

            let step = step as i16;

            for i in 0..32 {
                let x = i16x16::from_slice(&x[i * 16..(i + 1) * 16]);
                let dx = i16x16::from_slice(&dx[i * 16..(i + 1) * 16]);
                let nx = (x + i16x16::splat(step) * dx) % i16x16::splat(W as i16);
                for lane in 0..16 {
                    col_sums[nx[lane] as usize] += 1;
                }
            }

            if col_sums.iter().filter(|&cnt| *cnt >= 31).count() >= 2 {
                break 'outer step;
            }
        }
        panic!("no matching step for columns found");
    };
    let row_step = 'outer: {
        for step in 0..H {
            let mut row_sums = [0i16; H as usize];

            let step = step as i16;

            for i in 0..32 {
                let y = i16x16::from_slice(&y[i * 16..(i + 1) * 16]);
                let dy = i16x16::from_slice(&dy[i * 16..(i + 1) * 16]);
                let ny = (y + i16x16::splat(step) * dy) % i16x16::splat(H as i16);
                for lane in 0..16 {
                    row_sums[ny[lane] as usize] += 1;
                }
            }

            if row_sums.iter().filter(|&cnt| *cnt >= 31).count() >= 2 {
                break 'outer step;
            }
        }
        panic!("no matching step for rows found");
    };

    (H as i32 * 51 * col_step as i32 + W as i32 * 51 * row_step as i32) % (W * H) as i32
}

fn parse(input: &str) -> (Vec<i16>, Vec<i16>, Vec<i16>, Vec<i16>) {
    let mut xs = Vec::with_capacity(512);
    let mut dxs = Vec::with_capacity(512);
    let mut ys = Vec::with_capacity(512);
    let mut dys = Vec::with_capacity(512);
    let mut i = 0;
    let s = input.as_bytes();
    while i < input.len() {
        i += 2;
        let mut x = (s[i] - b'0') as i16;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            x = 10 * x + (s[i] - b'0') as i16;
            i += 1;
        }
        i += 1;
        let mut y = (s[i] - b'0') as i16;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            y = 10 * y + (s[i] - b'0') as i16;
            i += 1;
        }
        i += 3;
        let mut f = 1;
        if s[i] == b'-' {
            f = -1;
            i += 1;
        }
        let mut dx = (s[i] - b'0') as i16;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            dx = 10 * dx + (s[i] - b'0') as i16;
            i += 1;
        }
        dx = f * dx;
        i += 1;
        if s[i] == b'-' {
            f = -1;
            i += 1;
        } else {
            f = 1;
        }
        let mut dy = (s[i] - b'0') as i16;
        i += 1;
        while i < s.len() && s[i].is_ascii_digit() {
            dy = 10 * dy + (s[i] - b'0') as i16;
            i += 1;
        }
        dy = f * dy;
        i += 1;

        xs.push(x);
        ys.push(y);
        dxs.push(dx.rem_euclid(W));
        dys.push(dy.rem_euclid(H));
    }
    xs.extend([0; 12]);
    dxs.extend([0; 12]);
    ys.extend([0; 12]);
    dys.extend([0; 12]);

    (xs, dxs, ys, dys)
}
