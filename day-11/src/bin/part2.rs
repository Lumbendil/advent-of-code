use std::cmp::{max, min};
use std::io::{self, BufRead};
use itertools::{Itertools, multiunzip};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    const DISTANCE_PER_SPACE: i64 = 1000000 - 1;

    print!("{}", process(handle, DISTANCE_PER_SPACE));
}

fn process(input: impl BufRead, distance_per_space: i64) -> i64 {
    let galaxies: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, x)| x == &'#')
                .map(|(x, _)| x as i64)
                .collect::<Vec<i64>>()
        })
        .enumerate()
        .flat_map(|(x, list)| {
            list.into_iter().map(move |y| (x as i64, y))
        })
        .collect();


    let (mut rows_not_to_expand, mut columns_not_to_expand): (Vec<i64>, Vec<i64>) = multiunzip(galaxies.clone());

    rows_not_to_expand.sort();
    rows_not_to_expand.dedup();

    columns_not_to_expand.sort();
    columns_not_to_expand.dedup();

    galaxies
        .into_iter()
        .combinations(2)
        .map(|x| permutation_distance(x, &columns_not_to_expand, &rows_not_to_expand, distance_per_space))
        .sum()
}

fn permutation_distance(values: Vec<(i64, i64)>, column_non_expansions: &Vec<i64>, row_non_expansions: &Vec<i64>, distance_per_space: i64) -> i64 {
    assert_eq!(values.len(), 2);

    let (source_x, source_y) = values[0];
    let (target_x, target_y) = values[1];

    let distance = (source_x - target_x).abs() + (source_y - target_y).abs();

    let mut empty_spaces_x = 0;
    for a in min(source_x, target_x) .. max(source_x, target_x) {
        if !row_non_expansions.contains(&a) {
            empty_spaces_x += distance_per_space;
        }
    }

    let mut empty_spaces_y = 0;
    for a in min(source_y, target_y) .. max(source_y, target_y) {
        if !column_non_expansions.contains(&a) {
            empty_spaces_y += distance_per_space;
        }
    }

    distance + empty_spaces_x + empty_spaces_y
}
