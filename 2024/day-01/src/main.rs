use day_01::part1::process_part1;
use day_01::part2::process_part2;

fn main() {
    let test_case = include_str!("../input.txt");
    println!("Answer to the first part: {}", process_part1(test_case));
    println!("Answer to the second part: {}", process_part2(test_case))
}
