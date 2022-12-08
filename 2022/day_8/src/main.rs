use itertools::{iproduct, Product};
use std::fs;
use take_until::TakeUntilExt;

struct Map {
    map: Vec<Vec<i32>>,
    width: usize,
}

impl Map {
    fn new(input_stream: &str) -> Self {
        let width = input_stream.lines().next().unwrap().len();
        Self {
            map: input_stream
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|x| x.to_digit(10).unwrap() as i32)
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(),
            width: width,
        }
    }
    fn forest(&self) -> Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        iproduct!(0..self.map.len(), 0..self.width)
    }
    fn is_visible(&self, row_num: usize, col_num: usize) -> bool {
        self.is_top_visible(row_num, col_num)
            || self.is_left_visible(row_num, col_num)
            || self.is_bottom_visible(row_num, col_num)
            || self.is_right_visible(row_num, col_num)
    }
    fn is_top_visible(&self, row_num: usize, col_num: usize) -> bool {
        self.map[..row_num]
            .iter()
            .map(|row| row[col_num])
            .collect::<Vec<i32>>()
            .iter()
            .max()
            < Some(&self.map[row_num][col_num])
    }
    fn is_bottom_visible(&self, row_num: usize, col_num: usize) -> bool {
        self.map[row_num + 1..]
            .iter()
            .map(|row| row[col_num])
            .collect::<Vec<i32>>()
            .iter()
            .max()
            < Some(&self.map[row_num][col_num])
    }
    fn is_left_visible(&self, row_num: usize, col_num: usize) -> bool {
        self.map[row_num][..col_num].iter().max() < Some(&self.map[row_num][col_num])
    }
    fn is_right_visible(&self, row_num: usize, col_num: usize) -> bool {
        self.map[row_num][col_num + 1..].iter().max() < Some(&self.map[row_num][col_num])
    }
    fn get_scenic_score(&self, row_num: usize, col_num: usize) -> i32 {
        self.get_top_scenic_score(row_num, col_num)
            + self.get_bottom_scenic_score(row_num, col_num)
            + self.get_left_scenic_score(row_num, col_num)
            + self.get_right_scenic_score(row_num, col_num)
    }
    fn get_top_scenic_score(&self, row_num: usize, col_num: usize) -> i32 {
        self.map[..row_num]
            .iter()
            .map(|row| row[col_num])
            .take_until(|x| x >= &&self.map[row_num][col_num])
            .count() as i32
    }
    fn get_bottom_scenic_score(&self, row_num: usize, col_num: usize) -> i32 {
        self.map[row_num + 1..]
            .iter()
            .map(|row| row[col_num])
            .take_until(|x| x >= &&self.map[row_num][col_num])
            .count() as i32
    }
    fn get_left_scenic_score(&self, row_num: usize, col_num: usize) -> i32 {
        self.map[row_num][..col_num].iter().rev().take_until(|x| x >= &&self.map[row_num][col_num]).count() as i32
    }
    fn get_right_scenic_score(&self, row_num: usize, col_num: usize) -> i32 {
        self.map[row_num][col_num + 1..].iter().take_until(|x| x >= &&self.map[row_num][col_num]).count() as i32
    }
}

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    let map = Map::new(&input_stream);
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &Map) -> i32 {
    map.forest().filter(|(row_num, col_num)| map.is_visible(*row_num, *col_num))
        .count() as i32
}

fn part2(map: &Map) -> i32 {
    map.forest().map(|(row_num, col_num)| map.get_scenic_score(row_num, col_num)).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input_stream = fs::read_to_string("./inputs/demo.txt").unwrap();
        let map = Map::new(&input_stream);
        assert_eq!(part1(&map), 21);

        let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
        let map = Map::new(&input_stream);
        assert_eq!(part1(&map), 1827);
    }

    #[test]
    fn test_part2() {
        let input_stream = fs::read_to_string("./inputs/demo.txt").unwrap();
        let map = Map::new(&input_stream);
        assert_eq!(part2(&map), 8);
    }
}
