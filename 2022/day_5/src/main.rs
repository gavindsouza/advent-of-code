use std::{collections::HashMap, fs, ops::Deref};

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    let crates_diagram_end = input_stream.find("1").unwrap();
    let instructions_start = input_stream.find("move").unwrap();

    let crates_stack = get_crates(&input_stream[..crates_diagram_end]);
    let instructions = get_instructions(&input_stream[instructions_start..]);

    part1(crates_stack.to_owned(), &instructions);
    part2(crates_stack.to_owned(), &instructions);
}

fn part1(crates_stack: HashMap<usize, Vec<String>>, instructions: &Vec<Move>) {
    let mut stack = Stack {
        stack: crates_stack,
    };

    for step in instructions {
        stack.apply(*step);
    }
    println!("{:?}", stack.head());
}

fn part2(crates_stack: HashMap<usize, Vec<String>>, instructions: &Vec<Move>) {
    let mut stack = Stack {
        stack: crates_stack,
    };
    for step in instructions {
        stack.apply_batch(*step);
    }
    println!("{:?}", stack.head());
}

fn get_instructions(input_stream: &str) -> Vec<Move> {
    let mut instructions = vec![];
    for instruction in input_stream.lines() {
        let instr_tokens = instruction.split_whitespace().collect::<Vec<&str>>();
        let (size, source, target) = (
            to_usize(instr_tokens[1]),
            to_usize(instr_tokens[3]),
            to_usize(instr_tokens[5]),
        );
        instructions.push(Move {
            source: source,
            target: target,
            size: size,
        });
    }
    instructions
}

#[derive(Debug, Clone, Copy)]
struct Move {
    source: usize,
    target: usize,
    size: usize,
}

fn get_crates(stack: &str) -> HashMap<usize, Vec<String>> {
    let stack_tokens: Vec<Vec<String>> = stack
        .lines()
        .map(|line| get_tokens(line))
        .collect::<Vec<Vec<String>>>();
    let mut realized_stack: HashMap<usize, Vec<String>> = HashMap::new();

    for line in stack_tokens {
        for (i, token) in line.iter().enumerate() {
            let mut _token = token.deref();
            if _token == "   " {
                continue;
            } else {
                _token = unbox(_token);
            }
            if realized_stack.contains_key(&i) {
                let key = realized_stack.get_mut(&i).unwrap();
                key.push(_token.to_string());
            } else {
                realized_stack.insert(i, vec![_token.to_string()]);
            }
        }
    }

    for (_, stack) in realized_stack.iter_mut() {
        stack.reverse();
    }

    realized_stack
}

#[derive(Debug)]
struct Stack {
    stack: HashMap<usize, Vec<String>>,
}

impl Stack {
    fn push(&mut self, key: usize, value: String) {
        self.stack.entry(key).or_insert(Vec::new()).push(value);
    }

    fn pop(&mut self, key: usize) -> Option<String> {
        self.stack.get_mut(&key).and_then(|v| v.pop())
    }

    fn apply(&mut self, _move: Move) {
        for _ in 0.._move.size {
            let stacked_crate = self.pop(_move.source - 1);
            if stacked_crate.is_none() {
                continue;
            }
            self.push(_move.target - 1, stacked_crate.unwrap());
        }
    }

    fn apply_batch(&mut self, _move: Move) {
        let mut cached_crates = vec![];

        for _ in 0.._move.size {
            let stacked_crate = self.pop(_move.source - 1);
            if stacked_crate.is_none() {
                continue;
            }
            cached_crates.push(stacked_crate.unwrap());
        }
        for cached_crate in cached_crates.iter().rev() {
            self.push(_move.target - 1, cached_crate.to_owned());
        }
    }

    fn head(&mut self) -> String {
        let mut head_str = String::new();
        for i in 0..self.stack.len() {
            head_str.push_str(&self.top(i));
        }
        return head_str;
    }

    fn top(&mut self, key: usize) -> String {
        let vector = self.stack.get(&key);
        let empty = "   ".to_string();

        if vector.is_none() {
            return empty;
        }

        for element in vector.into_iter().rev() {
            let last_element = element.last();
            if last_element.is_none() {
                return empty;
            }
            return last_element.unwrap().to_string();
        }
        empty
    }
}

fn unbox(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn to_usize(s: &str) -> usize {
    s.parse().unwrap()
}

fn get_tokens(input: &str) -> Vec<String> {
    let mut tokens = vec![];
    let token_size = 3;
    let line_size = input.len();
    let slices_num = line_size / token_size;
    let mut buffer = 0;

    for i in 0..slices_num {
        let start = i * token_size + buffer;
        let end = start + token_size;
        if end > input.len() {
            break;
        }
        let token = &input[start..end];
        tokens.push(token.to_string());
        buffer += 1;
    }

    tokens
}
