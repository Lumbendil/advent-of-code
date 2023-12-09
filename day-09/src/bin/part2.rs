use std::io::{self, BufRead, Write};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(input: impl BufRead, mut output: impl Write) {
    let value: i64 = input.lines()
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(process_line)
        .sum();

    output.write(value.to_string().as_bytes()).unwrap();
}

fn process_line (line: String) -> i64 {
    let parsed: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();

    process_iterator(parsed)
}

fn process_iterator(values: Vec<i64>) -> i64 {
    let new_values: Vec<i64> = values
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect();

    if new_values.iter().all_equal() {
        *values.first().unwrap() - *new_values.first().unwrap()
    } else {
        let first_value = *values.first().unwrap();
        let next_value = process_iterator(new_values);

        first_value - next_value
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn process_line_works() {
        let data = vec![
            ("10 13 16 21 30 45", 5),
        ];

        for (input, output) in data {
            assert_eq!(process_line(input.to_string()), output);
        }
    }
}