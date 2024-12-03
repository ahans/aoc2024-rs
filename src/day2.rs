fn safe(levels: &[u8; 8], num_levels: usize) -> bool {
    let mut valid = true;
    let mut i = 0;
    let mut j = 1;

    while j < num_levels  {
        let diff = levels[j] as i32 - levels[i] as i32;
        if diff < 1 || diff > 3 {
            valid = false;
        }
        i += 1;
        j += 1;
    }

    let n = num_levels;


    if valid {
        return true;
    }

    valid = true;
    i = n - 2;
    j = n - 1;

    loop {
        let diff = levels[i] as i32 - levels[j] as i32;
        if diff < 1 || diff > 3 {
            valid = false;
            break;
        }
        if i == 0 { break; }
        i -= 1;
        j -= 1;
    }

    valid
}

#[aoc(day2, part1)]
pub fn part1(s: &str) -> i32 {
    let mut n = 0;
    let mut levels = [0u8; 8];
    let mut num_levels = 0;

    for ch in s.chars() {
        match ch {
            ' ' => num_levels += 1,
            '\n' => {
                if safe(&levels, num_levels + 1) {
                    n += 1;
                }
                num_levels = 0;
                levels = [0; 8];
            }
            _ => {
                levels[num_levels] = levels[num_levels] * 10 + (ch as u8 - b'0');
            }
        }
    }

    if num_levels > 0 && safe(&levels, num_levels + 1) {
        n += 1;
    }

    n
}

#[aoc(day2, part2)]
pub fn part2(s: &str) -> i32 {
    let mut n = 0;
    let mut levels = [0u8; 8];
    let mut num_levels = 0;

    for ch in s.chars() {
        match ch {
            ' ' => num_levels += 1,
            '\n' => {
                if safe(&levels, num_levels + 1) {
                    n += 1;
                } else {
                    for i in 0..8 {
                        if levels[i] == 0 { break; }
                        
                        let mut l2 = [0u8; 8];
                        let mut wt = 0;
                        
                        for j in 0..8 {
                            if levels[j] == 0 { break; }
                            if j != i {
                                l2[wt] = levels[j];
                                wt += 1;
                            }
                        }
                        
                        if safe(&l2, wt) {
                            n += 1;
                            break;
                        }
                    }
                }
                
                num_levels = 0;
                levels = [0; 8];
            }
            _ => {
                levels[num_levels] = levels[num_levels] * 10 + (ch as u8 - b'0');
            }
        }
    }

    if num_levels > 0 {
        if safe(&levels, num_levels + 1) {
            n += 1;
        } else {
            for i in 0..8 {
                if levels[i] == 0 { break; }
                
                let mut l2 = [0u8; 8];
                let mut wt = 0;
                
                for j in 0..8 {
                    if levels[j] == 0 { break; }
                    if j != i {
                        l2[wt] = levels[j];
                        wt += 1;
                    }
                }
                
                if safe(&l2, wt) {
                    n += 1;
                    break;
                }
            }
        }
    }

    n
}
