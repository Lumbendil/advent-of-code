use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let mut reversed_lines = input.lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    let first_line = reversed_lines.next().unwrap();

    let rock_counter: Vec<_> = first_line.chars()
        .into_iter()
        .map(|x| if x == 'O' {1} else {0})
        .collect();

    let (mut total_value, row_number, rock_counter) = reversed_lines
        .enumerate()
        .map(|(x, y)| (x + 2, y))
        .fold((0, 0, rock_counter), |(mut x, _, rock_counter), (row_number, row)| {
            let new_rock_counter: Vec<usize> =
                row.chars()
                    .zip(rock_counter.into_iter())
                    .map(|(char, count)| match char {
                        'O' => count + 1,
                        '#' => {
                            x +=  calculate_rocks_value(count, row_number);

                            0
                        },
                        _ => count
                    })
                    .collect()
            ;
            (x, row_number, new_rock_counter)
        });

    total_value += rock_counter.into_iter().map(|x| calculate_rocks_value(x, row_number + 1)).sum::<usize>();

    format!("{}", total_value)
}

fn calculate_rocks_value(rock_count: usize, current_row_value: usize) -> usize {
    (2 * (current_row_value - rock_count) + rock_count - 1)*rock_count/2
}


#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 5, 7)]
    #[case(1, 5, 4)]
    #[case(0, 5, 0)]
    fn test_calculate_rocks_value(#[case] rock_count: usize, #[case] current_row_value: usize, #[case] expected: usize) {
        assert_eq!(calculate_rocks_value(rock_count, current_row_value), expected);
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

        assert_eq!(process(data.as_bytes()), "136");
    }
}