use std::io::{self, BufRead, Write};
use std::ops::Range;
use itertools::Itertools;
use rangemap::RangeMap;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut value: i64 = i64::MAX;
    let mut first_line = String::new();

    _ = input.read_line(&mut first_line);
    _ = input.read_line(&mut first_line);

    let transforms: Vec<RangeMap<i64, i64>> = input
        .split(b'\n')
        .batching(|it| {
            if let None = it.next() {
                return None
            }
            let mut group = RangeMap::new();
            while let Some(x) = it.next() {
                let row = String::from_utf8(x.unwrap()).unwrap().trim().to_string();
                if row == "" {
                    break;
                }

                let data: Vec<i64> = row.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect();

                group.insert(data[1]..(data[1] + data[2]), data[0] - data[1]);
            }
            Some(group)
        })
        .collect();

    let values = first_line.trim().split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .tuples()
        .map(|(start, range)| start..(start + range))
    ;

    for ran in values {
        let mut offsets_range_map: Vec<Range<i64>> = Vec::new();
        offsets_range_map.push(ran.clone());

        for transform in &transforms {
            let mut applied_transformations = RangeMap::new();
            for range in &offsets_range_map {
                applied_transformations.insert(range.clone(), 0);
            }

            for range in &offsets_range_map {
                for (second_range, offset2) in transform.overlapping(&range) {
                    let overlap_range = std::cmp::max(range.start, second_range.start)..std::cmp::min(range.end, second_range.end);

                    applied_transformations.insert(overlap_range.clone(), *offset2);
                }
            }

            offsets_range_map.clear();

            for (range, offset) in applied_transformations {
                let new_range = range.start+offset .. range.end + offset;
                offsets_range_map.push(new_range);
            }
        }

        for range in offsets_range_map {
            if range.start < value {
                value = range.start
            }
        }
    }

    output.write(value.to_string().as_bytes()).unwrap();
}
