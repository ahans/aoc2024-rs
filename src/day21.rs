#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    solve::<LUT2>(input)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    solve::<LUT25>(input)
}

#[inline(always)]
fn solve<const LUT: [u64; 11 * 16]>(input: &str) -> u64 {
    let s = input.as_bytes();
    let mut result = 0;
    let mut i = 0;
    unsafe {
        for _ in 0..5 {
            let mut value = 0;
            let mut len = 0;

            let mut v = (s.get_unchecked(i) - b'0') as u64;
            i += 1;
            let mut prev = 10;
            value += v;
            len += LUT.get_unchecked((prev * 16 + v) as usize);
            prev = v;

            v = (s.get_unchecked(i) - b'0') as u64;
            i += 1;
            value = value * 10 + v;
            len += LUT.get_unchecked((prev * 16 + v) as usize);
            prev = v;

            v = (s.get_unchecked(i) - b'0') as u64;
            i += 1;
            value = value * 10 + v;
            len += LUT.get_unchecked((prev * 16 + v) as usize);

            len += LUT.get_unchecked((v * 16 + 10) as usize);

            i += 2;
            result += value * len;
        }
    }
    result
}

const LUT2: [u64; 11 * 16] = [
    1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 10, 0, 0, 0, 0, 0, 21, 1, 10, 11, 12, 19, 20, 13, 20,
    21, 22, 0, 0, 0, 0, 0, 16, 18, 1, 10, 21, 12, 19, 22, 13, 20, 17, 0, 0, 0, 0, 0, 21, 19, 18, 1,
    22, 21, 12, 23, 22, 13, 16, 0, 0, 0, 0, 0, 22, 16, 17, 18, 1, 10, 11, 12, 19, 20, 23, 0, 0, 0,
    0, 0, 17, 21, 16, 17, 18, 1, 10, 21, 12, 19, 18, 0, 0, 0, 0, 0, 22, 22, 21, 16, 19, 18, 1, 22,
    21, 12, 17, 0, 0, 0, 0, 0, 23, 17, 18, 19, 16, 17, 18, 1, 10, 11, 24, 0, 0, 0, 0, 0, 18, 22,
    17, 18, 21, 16, 17, 18, 1, 10, 19, 0, 0, 0, 0, 0, 23, 23, 22, 17, 22, 21, 16, 19, 18, 1, 18, 0,
    0, 0, 0, 0, 18, 26, 21, 12, 27, 22, 13, 28, 23, 14, 1, 0, 0, 0, 0, 0,
];

const LUT25: [u64; 11 * 16] = [
    1,
    31420065369,
    14752615084,
    24095973437,
    31420065370,
    14752615085,
    24095973438,
    31420065371,
    14752615086,
    24095973439,
    14287938116,
    0,
    0,
    0,
    0,
    0,
    27052881363,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    14752615085,
    24095973438,
    24095973439,
    27052881364,
    0,
    0,
    0,
    0,
    0,
    20790420654,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    28154654778,
    14752615085,
    24095973438,
    22778092491,
    0,
    0,
    0,
    0,
    0,
    27622800565,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    28154654779,
    28154654778,
    14752615085,
    20790420654,
    0,
    0,
    0,
    0,
    0,
    27052881364,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    27052881365,
    0,
    0,
    0,
    0,
    0,
    20790420655,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    22778092492,
    0,
    0,
    0,
    0,
    0,
    27622800566,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    20790420655,
    0,
    0,
    0,
    0,
    0,
    27052881365,
    20790420655,
    22778092492,
    22778092493,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    27052881366,
    0,
    0,
    0,
    0,
    0,
    20790420656,
    27622800566,
    20790420655,
    22778092492,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    22778092493,
    0,
    0,
    0,
    0,
    0,
    27622800567,
    27622800567,
    27622800566,
    20790420655,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    20790420656,
    0,
    0,
    0,
    0,
    0,
    22411052532,
    31420065370,
    28154654777,
    14752615084,
    31420065371,
    28154654778,
    14752615085,
    31420065372,
    28154654779,
    14752615086,
    1,
    0,
    0,
    0,
    0,
    0,
];