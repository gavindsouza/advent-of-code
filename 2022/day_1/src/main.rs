fn main() {
    let lines: Vec<Option<u32>> = include_str!("../inputs/test.txt")
        .lines()
        .map(|s| s.parse::<u32>().ok())
        .collect();

    let groups_sum = lines
        .split(|x| x.is_none())
        .map(|group| group.iter().filter_map(|x| *x).sum::<u32>());

    let p1 = groups_sum.clone().max().unwrap();

    let mut sorted_groups_sum = groups_sum.clone().collect::<Vec<_>>();
    sorted_groups_sum.sort_by(|a, b| b.cmp(a));
    let p2: u32 = sorted_groups_sum.iter().take(3).sum();

    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
}
