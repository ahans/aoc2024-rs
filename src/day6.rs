use bit_set::BitSet;

// fn gen_positions(input: &str) -> BitSet {
//     let mut positions = BitSet::with_capacity(130 * 130);
// }

fn preprocess(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, (usize, usize)) {
    let num_rows: usize = 130;
    let num_cols: usize = 130;

    let input = input.as_bytes();

    let mut row_obstacles: Vec<Vec<usize>> = vec![Vec::with_capacity(130); num_rows];
    let mut col_obstacles: Vec<Vec<usize>> = vec![Vec::with_capacity(130); num_cols];

    // Find start position and track obstacles
    let mut start_pos = (0, 0);
    for row in 0usize..num_rows {
        for col in 0usize..num_cols {
            match input[row * 131 + col] {
                b'^' => {
                    start_pos = (row, col);
                }
                b'#' => {
                    row_obstacles[row].push(col);
                    col_obstacles[col].push(row);
                }
                _ => {}
            }
        }
    }
    (row_obstacles, col_obstacles, start_pos)
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    let (row_obstacles, col_obstacles, start_pos) = preprocess(&input);

    // let start_pos = start_pos.unwrap();
    let (path, has_cycle) = simulate(start_pos, &row_obstacles, &col_obstacles);
    assert!(!has_cycle);

    let mut positions = BitSet::with_capacity(130 * 130);
    let mut p1 = 0;

    for window in path.windows(2) {
        let u = window[0];
        let v = window[1];
        let dir = if u.0 == v.0 {
            (0, if u.1 < v.1 { 1 } else { -1 })
        } else {
            (if u.0 < v.0 { 1 } else { -1 }, 0)
        };

        let mut p = u;
        loop {
            if !positions.contains(p.0 * 130 + p.1) {
                p1 += 1;
                positions.insert(p.0 * 130 + p.1);
            }
            if p == v {
                break;
            }
            p = ((p.0 as i32 + dir.0) as usize, (p.1 as i32 + dir.1) as usize);
        }
    }
    p1
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let (mut row_obstacles, mut col_obstacles, start_pos) = preprocess(&input);
    let (path, has_cycle) = simulate(start_pos, &row_obstacles, &col_obstacles);
    assert!(!has_cycle);

    let mut positions = BitSet::with_capacity(130 * 130);

    for window in path.windows(2) {
        let u = window[0];
        let v = window[1];
        let dir = if u.0 == v.0 {
            (0, if u.1 < v.1 { 1 } else { -1 })
        } else {
            (if u.0 < v.0 { 1 } else { -1 }, 0)
        };

        let mut p = u;
        loop {
            if !positions.contains(p.0 * 130 + p.1) {
                positions.insert(p.0 * 130 + p.1);
            }
            if p == v {
                break;
            }
            p = ((p.0 as i32 + dir.0) as usize, (p.1 as i32 + dir.1) as usize);
        }
    }

    positions.remove(start_pos.0 * 130 + start_pos.1);

    let mut p2 = 0;
    for obstacle in positions.iter() {
        // println!("{:?}", obstacle);
        let row = obstacle / 130;
        let col = obstacle % 130;

        if can_make_loop(
            (row, col),
            &mut row_obstacles,
            &mut col_obstacles,
            start_pos,
        ) {
            p2 += 1;
        }
    }

    p2
}

fn simulate(
    start_pos: (usize, usize),
    row_obstacles: &Vec<Vec<usize>>,
    col_obstacles: &Vec<Vec<usize>>,
) -> (Vec<(usize, usize)>, bool) {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut pos = start_pos;
    let mut path = Vec::with_capacity(1024);
    let mut seen = [false; 130 * 130 * 4];

    loop {
        let dir = directions[dir_index];

        if seen[(130 * 130) * dir_index + pos.0 * 130 + pos.1] {
            return (path, true);
        }

        path.push(pos);
        seen[(130 * 130) * dir_index + pos.0 * 130 + pos.1] = true;

        if dir.0 == 0 {
            // Horizontal movement
            let row = pos.0;
            let col = pos.1;
            let dx = dir.1;

            let obstacle_index = row_obstacles[row].binary_search(&col).unwrap_or_else(|x| x);

            if dx == -1 {
                // Move left
                let mut i = obstacle_index;
                while i > 0 && row_obstacles[row][i - 1] > col {
                    i -= 1;
                }

                if i == 0 {
                    path.push((row, 0));
                    return (path, false);
                }

                pos = (row, row_obstacles[row][i - 1] + 1);
                dir_index = (dir_index + 1) % 4;
            } else {
                // Move right
                let mut i = obstacle_index;
                while i < row_obstacles[row].len() && row_obstacles[row][i] <= col {
                    i += 1;
                }

                if i == row_obstacles[row].len() {
                    path.push((row, 130 - 1));
                    return (path, false);
                }

                pos = (row, row_obstacles[row][i] - 1);
                dir_index = (dir_index + 1) % 4;
            }
        } else {
            // Vertical movement
            let row = pos.0;
            let col = pos.1;
            let dy = dir.0;

            let obstacle_index = col_obstacles[col].binary_search(&row).unwrap_or_else(|x| x);

            if dy == -1 {
                // Move up
                let mut i = obstacle_index;
                while i > 0 && col_obstacles[col][i - 1] > row {
                    i -= 1;
                }

                if i == 0 {
                    path.push((0, col));
                    return (path, false);
                }

                pos = (col_obstacles[col][i - 1] + 1, col);
                dir_index = (dir_index + 1) % 4;
            } else {
                // Move down
                let mut i = obstacle_index;
                while i < col_obstacles[col].len() && col_obstacles[col][i] <= row {
                    i += 1;
                }

                if i == col_obstacles[col].len() {
                    path.push((130 - 1, col));
                    return (path, false);
                }

                pos = (col_obstacles[col][i] - 1, col);
                dir_index = (dir_index + 1) % 4;
            }
        }
    }
}

fn can_make_loop(
    pos: (usize, usize),
    row_obstacles: &mut Vec<Vec<usize>>,
    col_obstacles: &mut Vec<Vec<usize>>,
    start_pos: (usize, usize),
) -> bool {
    let row = pos.0;
    let col = pos.1;

    let row_index = row_obstacles[row].binary_search(&col).unwrap_or_else(|x| x);
    row_obstacles[row].insert(row_index, col);

    let col_index = col_obstacles[col].binary_search(&row).unwrap_or_else(|x| x);
    col_obstacles[col].insert(col_index, row);

    let (_, has_cycle) = simulate(start_pos, row_obstacles, col_obstacles);

    row_obstacles[row].remove(row_index);
    col_obstacles[col].remove(col_index);

    has_cycle
}
