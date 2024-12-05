use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn solve(input: &str) -> u32 {
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(input[0]);
    let updates = input[1];
    parse_updates(&rules, updates)
}

fn parse_rules(input: &str) -> HashMap<u8, HashSet<u8>> {
    let mut hm: HashMap<u8, HashSet<u8>> = HashMap::new();
    
    for row in input.lines() {
        let mut split = row.split("|");
        let a = split.next().unwrap().parse::<u8>().unwrap();
        let b = split.next().unwrap().parse::<u8>().unwrap();

        hm.entry(a)
            .or_default()
            .insert(b);
    }
    
    hm
}

fn parse_updates(rules: &HashMap<u8, HashSet<u8>>, updates: &str) -> u32 {
    return updates
        .lines()
        .map(|line| {
            let iter = line.split(',').map(|x| x.parse::<u8>().unwrap());
            let mut nums = vec![];
            for num in iter {
                nums.push(num);
            }
            if is_sorted(&nums, rules) {
                0
            } else {
                nums.sort_by(|a, b| custom_sort(*a, *b, rules));
                nums[nums.len() / 2] as u32
            }
        })
        .sum::<u32>();
}

fn custom_sort(a: u8, b: u8, rules: &HashMap<u8, HashSet<u8>>) -> Ordering {
    if a == b {
        return Ordering::Equal;
    } else if rules.get(&a).unwrap().contains(&b) {
        return Ordering::Less;
    }
    Ordering::Greater
}

fn is_sorted(nums: &[u8], rules: &HashMap<u8, HashSet<u8>>) -> bool {
    return nums
        .windows(2)
        .all(|pair| custom_sort(pair[0], pair[1], rules) != Ordering::Greater);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let input = include_str!("../input.txt");
        let result = solve(input);
        assert_eq!(4030, result);
    }
}