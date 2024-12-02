use itertools::Itertools;

enum Direction {
    Incr,
    Decr
}

pub fn process(input: &str) -> i32 {
    let mut res = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        match check_report(&report) {
            true => res += 1,
            false => {
                let is_safe = (0..report.len()).any(|idx| {
                    let modified: &Vec<i32> = &report.iter().enumerate()
                        .filter(|&(i, _)| i != idx)
                        .map(|(_, &value)| value)
                        .collect();
                    check_report(modified)
                });
                res += i32::from(is_safe)
            }
        }
    }
    res
}

fn check_report(report: &[i32]) -> bool {
    let mut direction: Option<Direction> = None;

    for (a, b) in  report.iter().tuple_windows() {
        let diff = b - a;
        if !(1..=3).contains(&diff.abs()) {
            return false
        }
        match diff.signum() {
            -1 => match direction {
                None => { direction = Some(Direction::Decr) }
                Some(Direction::Incr) => { return false }
                Some(Direction::Decr) => continue
            }
            1 => match direction {
                None => { direction = Some(Direction::Incr) }
                Some(Direction::Decr) => { return false }
                Some(Direction::Incr) => continue
            }
            _ => return false
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pt2() {
        let input = include_str!("../input.txt");
        let result = process(input);
        assert_eq!(465, result);
    }
}
