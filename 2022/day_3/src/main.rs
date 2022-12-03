use std::{collections::HashSet, fs};

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    let sacks = generate_rucksacks(&input_stream);

    part1(&sacks);
    part2(&sacks);
}

fn part1(sacks: &Vec<Rucksack>) {
    let mut sacks_with_extra_supplies = Vec::new();

    for sack in sacks {
        for supply in sack.common_supplies() {
            sacks_with_extra_supplies.push(ItemType { name: supply }.priority())
        }
    }

    println!(
        "Part 1: {:?}",
        sacks_with_extra_supplies.iter().sum::<u32>()
    );
}

fn part2(sacks: &Vec<Rucksack>) {
    let mut group_priority = Vec::new();

    for rucksacks in sacks.chunks(3) {
        let badge_priority = get_common_item(rucksacks).priority();
        group_priority.push(badge_priority);
    }

    println!("Part 2: {:?}", group_priority.iter().sum::<u32>());
}

#[derive(Copy, Clone)]
struct Rucksack<'a> {
    supplies: &'a str,
}

impl Rucksack<'_> {
    fn items(&self) -> Vec<ItemType> {
        self.supplies
            .chars()
            .map(|item| ItemType { name: item })
            .into_iter()
            .collect()
    }
    fn common_supplies(&self) -> HashSet<char> {
        let (_top, _bottom) = self.supplies.split_at(self.supplies.len() / 2);
        let top = _top.chars().collect::<Vec<char>>();
        let bottom = _bottom.chars().collect::<Vec<char>>();
        let mut _common_supplies = HashSet::new();

        for supply in top {
            if bottom.contains(&supply) {
                _common_supplies.insert(supply);
            }
        }
        _common_supplies
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ItemType {
    name: char,
}

impl ItemType {
    fn priority(&self) -> u32 {
        let ord = self.name as u32;

        if self.name >= 'a' && self.name <= 'z' {
            return ord - 97 + 1;
        } else if self.name >= 'A' && self.name <= 'Z' {
            return 26 + ord - 65 + 1;
        }
        panic!("Invalid item type: {}", self.name);
    }
}

fn generate_rucksacks(input_stream: &str) -> Vec<Rucksack> {
    input_stream
        .lines()
        .map(|x| Rucksack { supplies: x })
        .collect()
}

fn get_common_item<'a>(rucksacks: &'a [Rucksack]) -> ItemType {
    let mut common_item_types: Vec<ItemType> = Vec::new();

    for chunk in rucksacks.chunks(2) {
        let rucksack_1 = &chunk[0].items();

        if chunk.len() < 2 {
            for item_type in rucksack_1 {
                if common_item_types.contains(&item_type) {
                    return item_type.clone();
                }
            }
        } else {
            let rucksack_2 = &chunk[1].items();
            common_item_types.extend(get_common_items(rucksack_1, rucksack_2));
        }
    }
    panic!("No common item types found");
}

fn get_common_items(a: &Vec<ItemType>, b: &Vec<ItemType>) -> Vec<ItemType> {
    let (shorter, longer) = match a.len() > b.len() {
        true => (b, a),
        false => (a, b),
    };
    let shorter_set: HashSet<ItemType> = HashSet::from_iter(shorter.to_owned());
    let longer_set: HashSet<ItemType> = HashSet::from_iter(longer.to_owned());

    return shorter_set
        .intersection(&longer_set)
        .map(|i| *i)
        .collect::<Vec<_>>();
}
