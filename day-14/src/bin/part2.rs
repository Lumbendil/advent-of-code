use std::collections::HashSet;
use std::io::{self, BufRead};

const CYCLES: usize = 1000000000;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    todo!("This solution is still not working");

    println!("{}", process(handle, CYCLES));
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RockType {
    Moving,
    Static,
}

fn process(input: impl BufRead, _cycles: usize) -> String {
    let (moving_rocks, static_rocks): (Vec<_>, Vec<_>) = input.lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.unwrap()
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &char)| {
                    char != b'.'
                })
                .map(move |(y, &z)| {
                    (
                        (x, y),
                        match z {
                            b'#' => RockType::Static,
                            b'O' => RockType::Moving,
                            x => panic!("unexpected rock type '{}'", x as char)
                        }
                    )
                })
                .collect::<Vec<_>>()
        })
        .partition(|(_, t)| *t == RockType::Moving);

    let moving_rocks = convert_to_hashset(moving_rocks);
    let static_rocks = convert_to_hashset(static_rocks);

    dbg!(&moving_rocks, &static_rocks);

    format!("{}", 0)
}

fn convert_to_hashset(vec: Vec<((usize, usize), RockType)>) -> HashSet<(usize, usize)> {
    vec.into_iter()
        .map(|(x, _)| x)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "87")]
    #[case(2, "69")]
    #[case(3, "69")]
    fn example_after_n_cycles(#[case] cycles: usize, #[case] expected: &str) {
        let data = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(process(data.as_bytes(), cycles), expected);
    }

    #[test]
    fn example_works() {
        let data = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(process(data.as_bytes(), CYCLES), "64");
    }
}