use std::iter::zip;

pub fn process_part1(input: &str) -> i32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    input
        .lines()
        .for_each(|row| {
            let mut splited = row.split_whitespace();
            first.push(
                splited.next().unwrap().parse::<i32>().unwrap()
            );
            second.push(
                splited.next().unwrap().parse::<i32>().unwrap()
            );
        });
    first.sort();
    second.sort();

    zip(first, second)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pt1() {
        let input = include_str!("../input.txt");
        let result = process_part1(input);
        assert_eq!(2367773, result);
    }
}
