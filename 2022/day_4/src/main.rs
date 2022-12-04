use std::{fs, ops::RangeInclusive};

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    println!("Part 1: {}", part1(&input_stream));
    println!("Part 2: {}", part2(&input_stream));
}

fn part1(input_stream: &str) -> u32 {
    let mut total = 0;

    for line in input_stream.lines() {
        let (large_range, small_range) = generate_ranges(line);

        if large_range.swallows(small_range, true) {
            total += 1;
        }
    }
    total
}

fn part2(input_stream: &str) -> u32 {
    let mut total = 0;

    for line in input_stream.lines() {
        let (large_range, small_range) = generate_ranges(line);

        if large_range.swallows(small_range, false) {
            total += 1;
        }
    }
    total
}

fn generate_ranges(assignment_pair: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let ranges: Vec<&str> = assignment_pair.split(",").collect();
    let first_range = get_range_object(ranges[0]);
    let second_range = get_range_object(ranges[1]);

    match (first_range.end() - first_range.start()) > (second_range.end() - second_range.start()) {
        true => (first_range, second_range),
        false => (second_range, first_range),
    }
}

fn get_range_object(range_expr: &str) -> RangeInclusive<u32> {
    let range = range_expr
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    range[0]..=range[1]
}

pub trait RangeUtils {
    fn swallows(&self, other_range: RangeInclusive<u32>, strict: bool) -> bool;
}

impl RangeUtils for RangeInclusive<u32> {
    fn swallows(&self, other_range: RangeInclusive<u32>, strict: bool) -> bool {
        let contains_start = self.contains(&other_range.start());
        let contains_end = self.contains(&other_range.end());

        match strict {
            true => contains_start && contains_end,
            false => contains_start || contains_end,
        }
    }
}
