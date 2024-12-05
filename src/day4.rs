#[aoc(day4, part1)]
pub fn part1(s: &str) -> i32 {
    const XMAS: u32 = (b'X' as u32) << 24 | (b'M' as u32) << 16 | (b'A' as u32) << 8 | b'S' as u32;
    const SAMX: u32 = (b'S' as u32) << 24 | (b'A' as u32) << 16 | (b'M' as u32) << 8 | b'X' as u32;

    let s = s.as_bytes();
    let mut count = 0;

    unsafe {
        for row in 0..137 {
            for col in 0..140 {
                let idx = row * 141 + col;
                let c = s[idx];

                if c == b'X' || c == b'S' {
                    if col + 3 < 140 {
                        let horizontal = u32::from_ne_bytes([
                            *s.get_unchecked(idx),
                            *s.get_unchecked(idx + 1),
                            *s.get_unchecked(idx + 2),
                            *s.get_unchecked(idx + 3),
                        ]);
                        count += (horizontal == XMAS || horizontal == SAMX) as i32;

                        let diagonal1 = (*s.get_unchecked(idx) as u32) << 24
                            | (*s.get_unchecked(idx + 141 + 1) as u32) << 16
                            | (*s.get_unchecked(idx + 2 * 141 + 2) as u32) << 8
                            | (*s.get_unchecked(idx + 3 * 141 + 3) as u32);
                        count += (diagonal1 == XMAS || diagonal1 == SAMX) as i32;
                    }

                    let vertical = (*s.get_unchecked(idx) as u32) << 24
                        | (*s.get_unchecked(idx + 141) as u32) << 16
                        | (*s.get_unchecked(idx + 2 * 141) as u32) << 8
                        | (*s.get_unchecked(idx + 3 * 141) as u32);

                    count += (vertical == XMAS || vertical == SAMX) as i32;

                    if col >= 3 {
                        let diagonal2 = (s[idx] as u32) << 24
                            | (*s.get_unchecked(idx + 141 - 1) as u32) << 16
                            | (*s.get_unchecked(idx + 2 * 141 - 2) as u32) << 8
                            | (*s.get_unchecked(idx + 3 * 141 - 3) as u32);

                        count += (diagonal2 == XMAS || diagonal2 == SAMX) as i32;
                    }
                }
            }
        }

        for row in 137..140 {
            for col in 0..140 {
                let idx = row * 141 + col;
                let c = s[idx];

                if (c == b'X' || c == b'S') && col + 3 < 140 {
                    let horizontal = u32::from_ne_bytes([
                        *s.get_unchecked(idx),
                        *s.get_unchecked(idx + 1),
                        *s.get_unchecked(idx + 2),
                        *s.get_unchecked(idx + 3),
                    ]);
                    count += (horizontal == XMAS || horizontal == SAMX) as i32;
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(s: &str) -> i32 {
    const MS: u16 = (b'M' as u16) << 8 | (b'S' as u16);
    const SM: u16 = (b'S' as u16) << 8 | (b'M' as u16);

    let s = s.as_bytes();
    let mut count = 0;
    unsafe {
        for row in 1..139 {
            let offset = row * 141;
            let mut it = memchr::memchr_iter(b'A', &s[offset..offset + 141]);
            while let Some(i) = it.next() {
                if i >= 1 && i <= 138 {
                    let j = offset + i;
                    let diag1: u16 = ((*s.get_unchecked(j - 141 - 1) as u16) << 8)
                        | (*s.get_unchecked(j + 141 + 1) as u16);
                    let diag2: u16 = ((*s.get_unchecked(j - 141 + 1) as u16) << 8)
                        | (*s.get_unchecked(j + 141 - 1) as u16);
                    count += ((diag1 == SM || diag1 == MS) && (diag2 == SM || diag2 == MS)) as i32;
                }
            }
        }
    }
    count
}
