use std::io::{self, BufRead, Write};
use std::ops::Range;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

#[derive(Debug)]
struct TransformData {
    origin: Range<i64>,
    offset: i64,
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut first_line = String::new();
    // let mut value = 0;

    _ = input.read_line(&mut first_line);
    _ = input.read_line(&mut first_line);

    let mut values: Vec<i64> = first_line.trim().split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    let transforms = input
        .split(b'\n')
        .batching(|it| {
            if let None = it.next() {
                return None
            }
            let mut group = Vec::new();
            while let Some(x) = it.next() {
                let row = String::from_utf8(x.unwrap()).unwrap().trim().to_string();
                if row == "" {
                    break;
                }

                let data: Vec<i64> = row.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect();

                group.push(TransformData {
                    origin: data[1]..(data[1] + data[2]),
                    offset: data[0] - data[1],
                });
            }
            Some(group)
        });


    for transform_options in transforms {
        for x in values.iter_mut() {
            for transform_option in &transform_options {
                if transform_option.origin.contains(x) {
                    *x += transform_option.offset;
                    break;
                }
            }
        }
    }

    let value = values.iter().min().unwrap();


    output.write(value.to_string().as_bytes()).unwrap();
}
