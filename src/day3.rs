use memchr::{memchr2, memmem};

#[aoc(day3, part1)]
pub fn part1(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut p1 = 0;
    let mut it = memmem::find_iter(s, b"mul(");
    while let Some(offset) = it.next() {
        let mut i = offset + 4;
        let mut a = 0;
        while s[i].is_ascii_digit() {
            a = a * 10 + (s[i] as u8 - b'0') as i32;
            i += 1;
        }
        if s[i] != b',' {
            continue;
        }
        i += 1;
        let mut b = 0;
        while s[i].is_ascii_digit() {
            b = b * 10 + (s[i] as u8 - b'0') as i32;
            i += 1;
        }
        if s[i] != b')' {
            continue;
        }
        p1 += a * b;
    }
    p1
}

#[aoc(day3, part2)]
pub fn part2(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut p2 = 0;
    let mut on = true;
    let mut offset = 0;
    let do_finder = memmem::Finder::new(b"do");
    loop {
        if on {
            if let Some(i) = memchr2(b'm', b'd', &s[offset..]) {
                offset += i;
                if &s[offset..offset + 5] == b"don't" {
                    on = false;
                    offset += 5;
                } else if &s[offset..offset + 4] == b"mul(" {
                    offset += 4;
                    let mut a = 0;
                    while s[offset].is_ascii_digit() {
                        a = a * 10 + (s[offset] as u8 - b'0') as i32;
                        offset += 1;
                    }
                    if s[offset] != b',' {
                        continue;
                    }
                    offset += 1;
                    let mut b = 0;
                    while s[offset].is_ascii_digit() {
                        b = b * 10 + (s[offset] as u8 - b'0') as i32;
                        offset += 1;
                    }
                    if s[offset] != b')' {
                        continue;
                    }
                    offset += 1;
                    p2 += a * b;
                } else {
                    offset += 1;
                }
            } else {
                break;
            }
        } else {
            if let Some(i) = do_finder.find(&s[offset..]) {
                offset += i + 2;
                if &s[offset..offset + 3] != b"n't" {
                    on = true;
                } else {
                    offset += 3;
                }
            } else {
                break;
            }
        }
    }
    p2
}
