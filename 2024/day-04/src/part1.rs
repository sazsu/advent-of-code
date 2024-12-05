enum Direction {
    Right,
    Down,
    DownLeft,
    DownRight,
}

fn get_coords_change(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
        Direction::DownLeft => (1, -1),
        Direction::DownRight => (1, 1),
    }
}

pub fn parse(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect())
        .collect();
    let mut res = 0;
    let len = input.len();

    for i in 0..len {
        res += parse_line(&input, Direction::Right, i as i32, 0, len);
        res += parse_line(&input, Direction::Down, 0, i as i32, len);
    }
    // XMAS is 4 chars long
    for i in 0..len - 3 {
        res += parse_line(&input, Direction::DownRight, i as i32, 0, len - i);
        res += parse_line(&input, Direction::DownRight, 0, (i + 1) as i32, len - i - 1);
        res += parse_line(&input, Direction::DownLeft, 0, (len - i - 1) as i32,len - i);
        res += parse_line(&input, Direction::DownLeft, (i + 1) as i32, (len - 1) as i32, len - i - 1)
    }
    res
}

fn parse_line(input: &[Vec<char>], direction: Direction, mut r: i32, mut c: i32, size: usize) -> u32 {
    let mut cnt: u32 = 0;
    let (m, n) = get_coords_change(direction);
    let mut xmas = vec![];

    for _ in 0..size {
        let len = xmas.len();
        let ch = input[r as usize][c as usize];
        if len >= 3 && (
            (ch == 'S' && xmas[len - 1] == 'A' && xmas[len - 2] == 'M' && xmas[len - 3] == 'X') ||
            (ch == 'X' && xmas[len - 1] == 'M' && xmas[len - 2] == 'A' && xmas[len - 3] == 'S')) {
                xmas.clear();
                cnt += 1;
            }
        xmas.push(ch);
        r += m;
        c += n;
    }
    cnt
}