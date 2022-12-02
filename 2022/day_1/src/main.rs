use core::str::Lines;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("./inputs/test.txt").expect("ohno");
    let mut calorie_list = get_elven_calories(input_data.lines());

    calorie_list.sort();
    let top_three = calorie_list[calorie_list.len() - 3..calorie_list.len()].to_vec();

    println!("MAX (p1): {}", top_three.last().unwrap());
    println!("MAX3 (p2): {}", top_three.iter().sum::<i32>());
}

fn get_elven_calories(calorie_list: Lines) -> Vec<i32> {
    let mut elves_calories = vec![];
    let mut current_calorie_count = 0;

    for calorie in calorie_list {
        if calorie == "" {
            elves_calories.push(current_calorie_count);
            current_calorie_count = 0;
        } else {
            current_calorie_count += calorie.parse::<i32>().unwrap();
        }
    }
    elves_calories
}
