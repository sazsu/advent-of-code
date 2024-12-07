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
    if curr < target {
        return false;
    }
    if i == 0 {
        return target == curr
    }
    let mul = curr % nums[i] == 0 && check_possible(target, curr / nums[i], nums, i - 1);
    let sum = curr > nums[i] && !mul && check_possible(target, curr - nums[i], nums, i - 1);
    
    sum || mul
}

mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let input = include_str!("../input.txt");
        let result = solve(input);
        assert_eq!(4364915411363, result);
    }
}