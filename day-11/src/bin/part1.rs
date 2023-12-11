use std::cmp::{max, min};
use std::io::{self, BufRead};
use itertools::{Itertools, multiunzip};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle);
}

fn process(input: impl BufRead) {
    let galaxies: Vec<(i32, i32)> = input.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, x)| x == &'#')
                .map(|(x, _)| x as i32)
                .collect::<Vec<i32>>()
        })
        .enumerate()
        .flat_map(|(x, list)| {
            list.into_iter().map(move |y| (x as i32, y))
        })
        .collect();


    let (mut rows_not_to_expand, mut columns_not_to_expand): (Vec<i32>, Vec<i32>) = multiunzip(galaxies.clone());

    rows_not_to_expand.sort();
    rows_not_to_expand.dedup();

    columns_not_to_expand.sort();
    columns_not_to_expand.dedup();

    let value: i32 = galaxies
        .into_iter()
        .combinations(2)
        .map(|x| permutation_distance(x, &columns_not_to_expand, &rows_not_to_expand))
        .sum();

    print!("{}", value);
}

fn permutation_distance(values: Vec<(i32, i32)>, column_non_expansions: &Vec<i32>, row_non_expansions: &Vec<i32>) -> i32 {
    assert_eq!(values.len(), 2);

    let (source_x, source_y) = values[0];
    let (target_x, target_y) = values[1];

    let mut distance = (source_x - target_x).abs() + (source_y - target_y).abs();

    for a in min(source_x, target_x) .. max(source_x, target_x) {
        if !row_non_expansions.contains(&a) {
            distance += 1;
        }
    }


    for a in min(source_y, target_y) .. max(source_y, target_y) {
        if !column_non_expansions.contains(&a) {
            distance += 1;
        }
    }

    distance
}