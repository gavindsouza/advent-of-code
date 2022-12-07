use std::fs;

fn main() {
    let input_stream = fs::read_to_string("./inputs/demo.txt").unwrap();
    println!("{}", part1(&input_stream));
}

trait Reduce {
    fn reduce(&self) -> i32;
}
impl Reduce for Vec<i32> {
    fn reduce(&self) -> i32 {
        self.iter().sum()
    }
}

fn part1(input_stream: &str) -> i32 {
    let mut filesystem: Vec<Vec<i32>> = vec![];
    let mut filesystem_sums: Vec<i32> = vec![];

    for line in input_stream.lines() {
        if line.starts_with("$ cd") {
            if line == "$ cd .." {
                filesystem_sums.push(filesystem.last().unwrap().reduce());
                filesystem.pop();
            }
            filesystem.push(vec![]);
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            if line.starts_with("dir ") {
                filesystem.push(vec![]);
                continue;
            }
            let (file_size, _) = line.split_at(line.find(" ").unwrap());
            filesystem.last_mut().unwrap().push(file_size.parse().unwrap());
        }
    }

    let mut total = 0;
    for sum in &filesystem_sums {
        if sum < &100000 {
            total += sum;
        }
    }

    println!("{:?}", filesystem);
    println!("{:?}", filesystem_sums);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_part1() {
        let input_stream = fs::read_to_string("./inputs/demo.txt").unwrap();
        assert_eq!(part1(&input_stream), 95437);
    }
}