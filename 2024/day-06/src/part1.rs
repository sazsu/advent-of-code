use std::collections::HashSet;

enum Direction {
    Right,
    Down,
    Left,
    Up
}

impl Direction {
    fn turn(&mut self) {
        *self = match *self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right
        }
    }
    
    fn get_dir_change(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0)
        }
    }
}

fn parse(rows: &[&str]) -> (HashSet<(usize, usize)>, (usize, usize)) {
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut guard: Option<(usize, usize)> = None;
    for (i, row) in rows.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            match ch {
                '#' => { obstacles.insert((i, j)); },
                '^' => guard = Some((i, j)),
                _ => ()
            }
        }
    }
    (obstacles, guard.expect("no guard found"))
}

fn in_bounds(r: i32, c: i32, mx_r: i32, mx_c: i32) -> bool {
    0 <= r && r <= mx_r && 0 <= c && c <= mx_c
}

pub fn solve(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().collect();
    let mx_r = rows.len() as i32 - 1;
    let mx_c= rows[0].len() as i32 - 1;

    let (obstacles, (mut r, mut c)) = parse(&rows);
    let mut visited = HashSet::new();

    let mut direction = Direction::Up;

    loop {
        visited.insert((r, c));
        let (x_change, y_change) = direction.get_dir_change();

        let (new_x, new_y) = (r as i32 + x_change, c as i32 + y_change);
        if !in_bounds(new_x, new_y, mx_r, mx_c) {
            break
        } else if obstacles.contains(&(new_x as usize, new_y as usize)) {
            direction.turn();
        } else {
            (r, c) = (new_x as usize, new_y as usize);
        }
    }
    
    visited.len()
}
