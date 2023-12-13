use std::cmp::min;
use std::io::{self, BufRead};
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;

const TIMES_DUPLICATED: usize = 5;

fn main() {
    todo!("This solution is not working");
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let lines: Vec<String> = input.lines()
        .map(|x| x.unwrap())
        .collect();

    let value: usize = lines
        .par_iter()
        // .progress_count(lines.len() as u64) progress bar
        .map(|x| line_value(x, TIMES_DUPLICATED))
        .sum();

    format!("{}", value)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum GearStatus {
    Unknown,
    Working,
    Faulty,
}

fn line_value(line: &String, times_duplicated: usize) -> usize {
    let (gear_values, numbers) = line.split_once(' ').unwrap();

    let values: Vec<usize> = numbers.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut gears: Vec<GearStatus> = gear_values
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

    gears.push(GearStatus::Unknown);
    gears = gears.repeat(times_duplicated);
    gears.pop();

    let values = values.repeat(times_duplicated);

    possible_solutions(values.first(), &values[1..], &gears[..])
}


fn new_line_value(line: &String, times_duplicated: usize) -> usize {
    let (gear_values, numbers) = line.split_once(' ').unwrap();

    let values: Vec<usize> = numbers.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut gears: Vec<GearStatus> = gear_values
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

    let mut rev_values: Vec<usize> = values.clone().into_iter().rev().collect();
    let mut rev_gears: Vec<GearStatus> = gears.clone().into_iter().rev().collect();

    let find_min_solution_len = min_solution_length(values.first(), &values[1..], &gears[..], 0).unwrap();
    let find_min_solution_len_rev = min_solution_length(rev_values.first(), &rev_values[..], &rev_gears[..], 0).unwrap();

    let actual_len = gears.len();

    dbg!(find_min_solution_len, find_min_solution_len_rev, actual_len);

    0
    //
}


fn min_solution_length(current_value: Option<&usize>, values_tail: &[usize], gears: &[GearStatus], accumulated_length: usize) -> Option<usize> {
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
                return None
            }

            if gears.len() < value + first_non_working_gear + values_tail.iter().sum::<usize>() {
                return None
            }

            let gear = current_gear.unwrap();

            if gear == &GearStatus::Faulty {
                for i in 1..value {
                    if let None | Some(&GearStatus::Working) = gears.get(first_non_working_gear + i) {
                        return None
                    }
                }

                return match gears.get(first_non_working_gear + value) {
                    None => Some(accumulated_length + first_non_working_gear + value),
                    Some(GearStatus::Faulty) => None,
                    Some(_) => min_solution_length(values_tail.first(), get_tail(values_tail, 1), &gears[(first_non_working_gear + value + 1)..], accumulated_length + first_non_working_gear + value + 1),
                }
            } else {
                let no_faulty_solutions = min_solution_length(current_value, values_tail, &gears[(first_non_working_gear + 1)..], accumulated_length + first_non_working_gear + 1);

                for i in 1..value {
                    if let None|Some(&GearStatus::Working) = gears.get(first_non_working_gear + i) {
                        return no_faulty_solutions
                    }
                }

                if Some(&GearStatus::Faulty) == gears.get(first_non_working_gear + value) {
                    return no_faulty_solutions
                } else {
                    let faulty_solutions = min_solution_length(values_tail.first(), get_tail(values_tail, 1), get_tail(gears, first_non_working_gear + value + 1), accumulated_length + first_non_working_gear + value);

                    return match (no_faulty_solutions, faulty_solutions) {
                        (Some(A), Some(B)) => Some(min(A, B)),
                        (Some(A), None) => Some(A),
                        (None, Some(B)) => Some(B),
                        (None, None) => None,
                    }
                }
            }
        },
        None => if gears.contains(&GearStatus::Faulty) {
            return None
        } else {
            return Some(accumulated_length)
        }
    }
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

            if gears.len() < value + first_non_working_gear + values_tail.iter().sum::<usize>() {
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
    use rstest::rstest;
    use crate::*;
    #[rstest]
    #[case("?###???????? 3,2,1", 1, 10)]
    #[case("???????????? 2,2,1,1", 1, 35)]
    #[case("???????????? 2,2,1,1", 2, 3003)]
    #[case("???.### 1,1,3", 5, 1)]
    #[case("????.######..#####. 1,6,5", 5, 2500)]
    #[case("?###???????? 3,2,1", 1, 10)]
    #[case("?###???????? 3,2,1", 2, 150)]
    #[case("?###???????? 3,2,1", 3, 2250)]
    #[case("?###???????? 3,2,1", 4, 33750)]
    #[case("?###???????? 3,2,1", 5, 506250)]
    fn line_value_returns_expected_value(#[case] input: &str, #[case] times_duplicated: usize, #[case] expected: usize) {
        assert_eq!(line_value(&input.to_string(), times_duplicated), expected);
    }

    #[test]
    fn example_works() {
        let data = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(data.as_bytes()), "525152");
    }
}