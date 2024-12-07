use bit_set::BitSet;

fn parse(input: &str) -> (BitSet, usize) {
    let s = input.as_bytes();
    let mut pairs = BitSet::with_capacity(100 * 100);
    let mut i: usize = 0;
    while s[i] != b'\n' {
        let a = (s[i] - b'0') * 10 + (s[i + 1] - b'0');
        let b = (s[i + 3] - b'0') * 10 + (s[i + 4] - b'0');
        pairs.insert(a as usize * 100 + b as usize);
        i += 6;
    }
    i += 1;
    (pairs, i)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let (pairs, offset) = parse(&input);
    let mut p1 = 0;
    let mut i = offset;
    let s = input.as_bytes();
    while i < s.len() {
        let mut a = (s[i] - b'0') * 10 + (s[i + 1] - b'0');
        i += 2;
        let mut valid = true;
        let mut nums = Vec::with_capacity(32);
        nums.push(a);
        while i < s.len() && s[i] == b',' {
            if !valid {
                i += 3;
            } else {
                i += 1; // skip ,
                let b = (s[i] - b'0') * 10 + (s[i + 1] - b'0');
                i += 2;
                if !pairs.contains(a as usize * 100 + b as usize) {
                    valid = false;
                }
                a = b;
                nums.push(b);
            }
        }
        if valid {
            p1 += nums[nums.len() / 2] as i32;
        }
        i += 1;
    }
    p1
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let (pairs, offset) = parse(&input);
    let mut p2 = 0;
    let mut i = offset;
    let s = input.as_bytes();
    while i < s.len() {
        let a = (s[i] - b'0') * 10 + (s[i + 1] - b'0');
        i += 2;
        let mut nums = Vec::with_capacity(32);
        nums.push(a);
        while i < s.len() && s[i] == b',' {
            i += 1; // skip ,
            let b = (s[i] - b'0') * 10 + (s[i + 1] - b'0');
            i += 2;
            nums.push(b);
        }
        i += 1;

        if !nums
            .windows(2)
            .all(|uv| pairs.contains(uv[0].clone() as usize * 100 + uv[1].clone() as usize))
        {
            nums.sort_unstable_by(|u, v| {
                if pairs.contains(u.clone() as usize * 100 + v.clone() as usize) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            p2 += nums[nums.len() / 2] as i32;
        }
    }
    p2
}
