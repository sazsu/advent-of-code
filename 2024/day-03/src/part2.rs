use std::cmp::Ordering;

enum Preset {
    Enabled,
    Disabled,
}

pub fn parse_corrupted_memory(input: &str) -> i32 {
    let mut curr_preset = Preset::Enabled;
    let input = input.replace("\n", "");
    let operations = input.split("mul(");
    let sum: i32 = operations
        .map(|x| {
            let curr_mult = match curr_preset {
                Preset::Enabled => {
                    match x.find(")") {
                        Some(i) => {
                            let expr = &x[..i];
                            let (a, b) = parse_expr(expr);
                            a * b
                        }
                        None => 0
                    }
                },
                Preset::Disabled => 0
            };
            if let Some(p) = get_new_preset(x) {
                curr_preset = p
            }
            curr_mult
            
    }).sum();
    sum
}

fn get_new_preset(s: &str) -> Option<Preset> {
    let do_idx = s.rfind("do()").unwrap_or(0);
    let dont_idx = s.rfind("don't()").unwrap_or(0);

    match do_idx.cmp(&dont_idx) {
        Ordering::Greater => Some(Preset::Enabled),
        Ordering::Less => Some(Preset::Disabled),
        Ordering::Equal => None
    }

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
    fn test_parsing_pt2() {
        let input = include_str!("../input.txt");
        let result = parse_corrupted_memory(input);
        assert_eq!(85508223, result);
    }
}
