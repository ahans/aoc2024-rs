use memchr::memchr_iter;
use std::collections::VecDeque;

const W: i32 = 57;
const H: i32 = 57;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    solve::<true>(input)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    solve::<false>(input)
}

fn solve<const WITH_SEEN_SET: bool>(input: &str) -> i32 {
    unsafe {
        #[inline]
        fn index<const WIDTH: i32>(row: i32, col: i32) -> usize {
            return (row * WIDTH + col) as usize;
        }
        let grid = input.as_bytes();
        let mut count = 0;
        let mut q = VecDeque::with_capacity((W * H) as usize);
        let mut it = memchr_iter(b'9', grid);
        while let Some(offset) = it.next() {
            let start_row = (offset / (W + 1) as usize) as i32;
            let start_col = (offset % (W + 1) as usize) as i32;
            if *grid.get_unchecked(index::<{ W + 1 }>(start_row, start_col)) == b'9' {
                q.clear();
                q.push_back((start_row, start_col));
                let mut seen = [false; (W * H) as usize];
                while !q.is_empty() {
                    let (row, col) = q.pop_front().unwrap();
                    if WITH_SEEN_SET && seen[index::<W>(row, col)] {
                        continue;
                    }
                    let cur = grid.get_unchecked(index::<{ W + 1 }>(row, col));
                    if *cur == b'0' {
                        count += 1;
                    }
                    if WITH_SEEN_SET {
                        *seen.get_unchecked_mut(index::<W>(row, col)) = true;
                    }
                    for (dr, dc) in DIRECTIONS {
                        let nr = row + dr;
                        let nc = col + dc;
                        if (!(0 <= nr && nr < H && 0 <= nc && nc < W))
                            || *grid.get_unchecked(index::<{ W + 1 }>(nr, nc)) != cur - 1
                            || (WITH_SEEN_SET && *seen.get_unchecked(index::<W>(nr, nc)))
                        {
                            continue;
                        }
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        count
    }
}
