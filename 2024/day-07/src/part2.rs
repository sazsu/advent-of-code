pub fn solve(input: &str) -> u64 {
    let mut res = 0;
    for row in input.lines() {
        let split: Vec<&str> = row.split_whitespace().collect();
        let target: u64 = split[0].strip_suffix(":").unwrap().parse().expect("not a number");
        let nums: Vec<u64> = split[1..]
            .iter()
            .map(|x| x.parse().expect("not a number"))
            .collect();

        if check_possible(nums[0], target, &nums, nums.len() - 1) {
            res += target
        }
    }
    res
}

fn check_possible(target: u64, curr: u64, nums: &[u64], i: usize) -> bool {
    if i == 0 {
        return target == curr
    }
    let mul = match curr % nums[i] == 0 {
        true => check_possible(target, curr / nums[i], nums, i - 1),
        false => false
    };
    let sum = match !mul && curr > nums[i] {
        true => check_possible(target, curr - nums[i], nums, i - 1),
        false => false
    };
    let concat = match !mul && !sum && curr.to_string().ends_with(&nums[i].to_string()) {
        true => check_possible(target, curr / 10u64.pow(nums[i].to_string().len() as u32), nums, i - 1),
        false => false
    };
    mul || sum || concat
}

mod tests {
    use super::*;

    #[test]
    fn test_pt2() {
        let input = include_str!("../input.txt");
        let result = solve(input);
        assert_eq!(38322057216320, result);
    }
}