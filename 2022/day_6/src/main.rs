use std::fs;

fn main() {
    let input_stream = fs::read_to_string("./inputs/test.txt").unwrap();
    println!("Part 1: {}", get_marker_index(&input_stream, 4));
    println!("Part 2: {}", get_marker_index(&input_stream, 14));
}

fn get_marker_index(input_stream: &str, length: usize) -> usize {
    let mut marker = vec![];

    for (index, character) in input_stream.chars().enumerate() {
        let char_exists_at = marker.iter().position(|&x| x == character);

        if char_exists_at.is_some() {
            for _ in 0..=char_exists_at.unwrap() {
                marker.remove(0);
            }
        }

        if marker.len() + 1 == length {
            return index + 1;
        }

        marker.push(character);
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::get_marker_index;

    #[test]
    fn demo_part_1() {
        assert_eq!(get_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn demo_part_2() {
        assert_eq!(get_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(get_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(get_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            get_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(get_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
