pub fn parse_corrupted_memory(input: &str) -> i32 {
    let input = input.replace("\n", "");
    let operations = input.split("mul(");
    let sum: i32 = operations
        .map(|x| {
            match x.find(")") {
                Some(i) => {
                    let expr = &x[..i];
                    let (a, b) = parse_expr(expr);
                    a * b
                }
                None => 0
            }
    }).sum();
    sum
}

fn parse_expr(expr: &str) -> (i32, i32) {
    let parsed = expr
        .split(",")
        .collect::<Vec<&str>>();
    match parsed.len() {
        2 => {
            let a = parsed[0].parse::<i32>().unwrap_or(0);
            let b = parsed[1].parse::<i32>().unwrap_or(0);
            (a, b)
        }
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let input = include_str!("../input.txt");
        let result = parse_corrupted_memory(input);
        assert_eq!(187825547, result);
    }
}