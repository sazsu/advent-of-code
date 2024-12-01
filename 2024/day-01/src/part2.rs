use std::collections::HashMap;

pub fn process_part2(input: &str) -> i32 {
    let mut first: Vec<i32> = vec![];
    let mut hm: HashMap<i32, i32> = HashMap::new();

    input.lines().for_each(|row| {
        let mut splited = row.split_whitespace();

        first.push(splited.next().unwrap().parse::<i32>().unwrap());
        *hm.entry(splited.next().unwrap().parse::<i32>().unwrap())
            .or_insert(0) += 1;
    });
    let mut sum = 0;
    first
        .iter()
        .for_each(
            |&x| sum += x * *hm.entry(x).or_insert(0)
        );
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pt2() {
        let input = include_str!("../input.txt");
        let result = process_part2(input);
        assert_eq!(21271939, result);
    }
}
