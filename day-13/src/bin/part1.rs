use std::io::{self, BufRead};
use std::ops::Range;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let value: usize = input.lines()
        .batching(|x| {
            let mut result = Vec::new();
            while let Some(Ok(line)) = x.next() {
                if line.is_empty() {
                    break;
                }

                result.push(line);
            }

            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        })
        .map(process_map)
        .sum();

    format!("{}", value)
}

fn process_map(map: Vec<String>) -> usize {
    if let Some(x) = check_map_horizontal(&map) {
        x * 100
    } else {
        check_map_vertical(&map).unwrap()
    }
}

fn check_map_horizontal(map: &Vec<String>) -> Option<usize> {
    let map_len = map.len();

    (1..map_len)
        .filter(|&option| {
        let (range_before, range_after) = get_ranges(option, map_len);

        range_before.zip(range_after.rev()).all(|(x, y)| map[x] == map[y])
    }).next()
}

fn check_map_vertical(map: &Vec<String>) -> Option<usize> {
    let row_len = map[0].len();

    (1..row_len)
        .filter(|&option| {
            let (range_before, range_after) = get_ranges(option, row_len);

            map.iter().all(|line| {
                let line = line.as_bytes();
                // Maybe there is a more optimal way to compare reversed strings, but I don't know it
                range_before.clone().zip(range_after.clone().rev()).all(|(x, y)| line[x] == line[y])
            })
        }).next()
}

fn get_ranges(current_index: usize, map_length: usize) -> (Range<usize>, Range<usize>) {
    let half_map_length = map_length / 2;
    if current_index <= half_map_length {
        (0..current_index, current_index..current_index*2)
    } else {
        ((2*current_index - map_length)..current_index, current_index..map_length)
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 8, (0..1, 1..2))]
    #[case(7, 8, (6..7, 7..8))]
    #[case(6, 8, (4..6, 6..8))]
    #[case(4, 8, (0..4, 4..8))]
    #[case(4, 9, (0..4, 4..8))]
    #[case(5, 9, (1..5, 5..9))]
    fn test_get_ranges(#[case] current_index: usize, #[case] map_length: usize, #[case] expected_ranges: (Range<usize>, Range<usize>)) {
        assert_eq!(get_ranges(current_index, map_length), expected_ranges)
    }

    #[test]
    fn example_works() {
        let data = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(process(data.as_bytes()), "405");
    }
}