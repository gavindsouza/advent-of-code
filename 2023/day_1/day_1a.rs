fn main() {
    part_one();
}

fn part_one() {
    let calibration_values_sum: u32 = include_str!("./inputs/test.txt")
        .lines()
        .map(|line| {
            let digits = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum();
    println!("{}", calibration_values_sum);
}
