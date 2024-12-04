pub fn parse(input: &str) -> u32{
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect())
        .collect();
    let max_r = input.len() - 1;
    let max_c = input[0].len() - 1;
    let mut res = 0;

    for (r, row) in input.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'A' {
                    if r == 0 || r >= max_r || c == 0 || c >= max_c {
                        continue
                    }
                    let first = [input[r - 1][c - 1], input[r + 1][c + 1]];
                    let second = [input[r + 1][c - 1], input[r - 1][c + 1]];
                    if string_matches(&first) && string_matches(&second) {
                        res += 1;
                    }
            }
        }
    }
    res
}

fn string_matches(s: &[char; 2]) -> bool {
    (s[0] == 'M' && s[1] == 'S') || (s[0] == 'S' && s[1] == 'M')
}