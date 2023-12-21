use std::collections::HashSet;
use std::io::{self, BufRead};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

const NUMBER_OF_STEPS: usize = 64;

fn process(input: impl BufRead) -> String {
    format!("{}", process_steps(input, NUMBER_OF_STEPS))
}

fn process_steps(input: impl BufRead, steps: usize) -> usize {
    let mut height = 0;
    let mut width = 0;

    let elements: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {

            height = x;

            line.unwrap()
                .char_indices()
                .filter_map(|(y, char)| {
                    if width < y {
                        width = y;
                    }

                    match char {
                        '#' => Some(((x, y), true)),
                        'S' => Some(((x, y), false)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()

        })
        .collect()
    ;

    let start_element = elements.iter().find(|(_, val)| *val == false).unwrap().0;
    let rocks: HashSet<_> = elements
        .into_iter()
        .filter_map(|(pos, x)| {
            if x {
                Some(pos)
            } else {
                None
            }
        })
        .collect();

    let mut current_steps = vec![start_element];

    for _ in 0..steps {
        current_steps = process_step(current_steps, height, width, &rocks);
    }

    current_steps.len()
}

fn process_step(current_positions: Vec<(usize, usize)>, height: usize, width: usize, rocks: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    current_positions
        .into_iter()
        .flat_map(|pos| get_positions(pos, height, width))
        .filter(|pos| !rocks.contains(pos))
        .sorted()
        .dedup()
        .collect()
}

fn get_positions((x, y): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if x > 0 {
        result.push((x - 1, y));
    }

    if y > 0 {
        result.push((x, y - 1));
    }

    if x < height {
        result.push((x + 1, y));
    }

    if y < width {
        result.push((x, y + 1));
    }

    result
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(process_steps(data.as_bytes(), 6), 16);
    }
}