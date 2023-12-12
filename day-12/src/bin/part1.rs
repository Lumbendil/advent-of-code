use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let value: usize = input.lines()
        .map(|x| x.unwrap())
        .map(line_value)
        .sum();

    format!("{}", value)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum GearStatus {
    Unknown,
    Working,
    Faulty,
}

fn line_value(line: String) -> usize {
    let (gear_values, numbers) = line.split_once(' ').unwrap();

    let values: Vec<usize> = numbers.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let gears: Vec<GearStatus> = gear_values
        .chars()
        .map(|x| {
            match x {
                '?' => GearStatus::Unknown,
                '.' => GearStatus::Working,
                '#' => GearStatus::Faulty,
                _ => panic!("unexpecter char")
            }
        })
        .collect();

    possible_solutions(values.first(), &values[1..], &gears[..])
}

fn possible_solutions(current_value: Option<&usize>, values_tail: &[usize], gears: &[GearStatus]) -> usize {
    match current_value {
        Some(&value) => {
            let mut first_non_working_gear = 0;

            while let Some(x) = gears.get(first_non_working_gear) {
                if x != &GearStatus::Working {
                    break;
                }
                first_non_working_gear += 1;
            }

            let current_gear = gears.get(first_non_working_gear);
            if current_gear.is_none() {
                return 0
            }

            if gears.len() < value + first_non_working_gear {
                return 0
            }

            let gear = current_gear.unwrap();

            if gear == &GearStatus::Faulty {
                for i in 1..value {
                    if let None|Some(&GearStatus::Working) = gears.get(first_non_working_gear + i) {
                        return 0
                    }
                }

                return match gears.get(first_non_working_gear + value) {
                    None => if values_tail.is_empty() { 1 } else { 0 },
                    Some(GearStatus::Faulty) => 0,
                    Some(_) => possible_solutions(values_tail.first(), get_tail(values_tail, 1), &gears[(first_non_working_gear + value + 1)..]),
                }
            } else {
                let no_faulty_solutions = possible_solutions(current_value, values_tail, &gears[(first_non_working_gear + 1)..]);

                for i in 1..value {
                    if let None|Some(&GearStatus::Working) = gears.get(first_non_working_gear + i) {
                        return no_faulty_solutions
                    }
                }

                if Some(&GearStatus::Faulty) == gears.get(first_non_working_gear + value) {
                    return no_faulty_solutions
                } else {
                    let faulty_solutions = possible_solutions(values_tail.first(), get_tail(values_tail, 1), get_tail(gears, first_non_working_gear + value + 1));

                    return no_faulty_solutions + faulty_solutions
                }
            }
        },
        None => if gears.contains(&GearStatus::Faulty) {
            return 0
        } else {
            return 1
        }
    }
}

fn get_tail<T>(elements: &[T], total: usize) -> &[T] {
    if elements.len() <= total {
        &[]
    } else {
        &elements[total..]
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn my_test() {
        assert_eq!(line_value("?###???????? 3,2,1".to_string()), 10);
    }

    #[test]
    fn line_value_returns_expected_value() {
        let data = vec![
            ("??.### 1,1,3", 0),
            ("???.### 1,1,3", 1),
            ("?###???????? 3,2,1", 10),
        ];

        for (input, expected) in data {
            let output = line_value(input.to_string());
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn example_works() {
        let data = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(data.as_bytes()), "21");
    }
}