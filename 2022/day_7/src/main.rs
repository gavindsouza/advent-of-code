use std::fs;

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    part1(&input_stream);
}

#[derive(Debug, Clone, PartialEq)]
struct FSNode {
    name: &str,
    size: i32,
    children: Vec<&FSNode>,
    parent: Option<Box<FSNode>>,
    is_file: bool,
}

impl FSNode {
    fn get_size(&self) -> i32 {
        if self.is_file {
            self.size
        } else {
            self.children.iter().map(|child| child.get_size()).sum()
        }
    }
}

fn part1(input_stream: &str) -> i32 {
    let root_node = FSNode {
        name: "/",
        children: vec![],
        parent: None,
        is_file: false,
        size: 0,
    };
    let mut current_node = root_node;
    let mut push_to_current_node = false;

    for line in input_stream.lines() {
        if line.starts_with("$ cd") {
            if line == "$ cd /" {
                push_to_current_node = false;
                continue;

            } else if line == "$ cd .." {
                push_to_current_node = false;
                continue;
            }

            let node = FSNode {
                name: line[4..].trim(),
                children: vec![],
                parent: Some(Box::new(current_node)),
                is_file: false,
                size: 0,
            };
            current_node.children.push(&node);
            push_to_current_node = false;
            continue;

        } else if line.starts_with("$ ls") {
            push_to_current_node = true;
            continue;

        } else if push_to_current_node {
            let (file_size, file_name) = line.split_at(line.find(" ").unwrap());

            if file_size == "dir" {
                let node = FSNode {
                    name: file_name,
                    children: vec![],
                    parent: Some(Box::new(current_node)),
                    is_file: false,
                    size: 0,
                };
                current_node.children.push(&node);

            } else {
                let node = FSNode {
                    name: file_name.trim(),
                    size: file_size.parse::<i32>().unwrap(),
                    children: vec![],
                    parent: Some(Box::new(current_node)),
                    is_file: true,
                };
                current_node.children.push(&node);
            }
        }
    }
    0
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