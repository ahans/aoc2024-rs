#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let s = input.as_bytes();
    let mut antennas = vec![
        [
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
        ];
        128
    ];
    let mut antinodes = vec![false; 50 * 50];
    let mut p1 = 0;
    unsafe {
        for row in 0..50 {
            for col in 0..50 {
                match s[row * 51 + col] {
                    b'.' => {}
                    c => {
                        let character_index = c as usize;
                        let mut index = 0;
                        loop {
                            let other =
                                antennas.get_unchecked(character_index).get_unchecked(index);
                            if other.0 == -1 {
                                break;
                            }
                            index += 1;
                            let dr = row as i32 - other.0 as i32;
                            let dc = col as i32 - other.1 as i32;
                            let p = (other.0 as i32 - dr as i32, other.1 as i32 - dc as i32);
                            if 0 <= p.0 && p.0 < 50 && 0 <= p.1 && p.1 < 50 {
                                let idx = p.0 as usize * 50 + p.1 as usize;
                                if !antinodes.get_unchecked(idx) {
                                    *antinodes.get_unchecked_mut(idx) = true;
                                    p1 += 1;
                                }
                            }
                            let p = (row as i32 + dr as i32, col as i32 + dc as i32);
                            if 0 <= p.0 && p.0 < 50 && 0 <= p.1 && p.1 < 50 {
                                let idx = p.0 as usize * 50 + p.1 as usize;
                                if !antinodes.get_unchecked(idx) {
                                    *antinodes.get_unchecked_mut(idx) = true;
                                    p1 += 1;
                                }
                            }
                        }
                        *antennas
                            .get_unchecked_mut(character_index)
                            .get_unchecked_mut(index) = (row as i8, col as i8);
                    }
                };
            }
        }
    }
    p1
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let s = input.as_bytes();
    let mut antennas = vec![
        [
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
            (-1i8, -1i8),
        ];
        128
    ];
    let mut antinodes = vec![false; 50 * 50];
    let mut p2 = 0;
    unsafe {
        for row in 0..50 {
            for col in 0..50 {
                match s[row * 51 + col] {
                    b'.' => {}
                    c => {
                        let character_index = c as usize;
                        let mut index = 0;
                        loop {
                            let other =
                                antennas.get_unchecked(character_index).get_unchecked(index);
                            if other.0 == -1 {
                                break;
                            }
                            index += 1;
                            let dr = row as i32 - other.0 as i32;
                            let dc = col as i32 - other.1 as i32;
                            let mut p = (other.0 as i32, other.1 as i32);
                            while 0 <= p.0 && p.0 < 50 && 0 <= p.1 && p.1 < 50 {
                                let idx = p.0 as usize * 50 + p.1 as usize;
                                if !antinodes.get_unchecked(idx) {
                                    *antinodes.get_unchecked_mut(idx) = true;
                                    p2 += 1;
                                }
                                p = (p.0 + dr, p.1 + dc);
                            }
                            p = (row as i32, col as i32);
                            while 0 <= p.0 && p.0 < 50 && 0 <= p.1 && p.1 < 50 {
                                let idx = p.0 as usize * 50 + p.1 as usize;
                                if !antinodes.get_unchecked(idx) {
                                    *antinodes.get_unchecked_mut(idx) = true;
                                    p2 += 1;
                                }
                                p = (p.0 - dr, p.1 - dc);
                            }
                        }
                        *antennas
                            .get_unchecked_mut(character_index)
                            .get_unchecked_mut(index) = (row as i8, col as i8);
                    }
                };
            }
        }
    }
    p2
}
