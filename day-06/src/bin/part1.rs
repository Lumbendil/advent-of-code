use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn line_to_numbers(line: String) -> Vec<f64> {
    line.trim().split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<f64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut line = String::new();

    _ = input.read_line(&mut line);
    let times = line_to_numbers(line.clone());
    line.clear();
    _ = input.read_line(&mut line);
    let distances = line_to_numbers(line);

    let value: i64 = times.iter().zip(distances.iter())
        .map(|(time, distance)| {

            // speed = time_pressed;
            // 0 > time_pressed.powi(2) - time_pressed * time - distance;

            let sol_1 = ((time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0 - 1.0).ceil() as i64;
            let sol_2 = ((time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0 + 1.0).floor() as i64;

            sol_1 - sol_2 + 1
        })
        .product();

    output.write(value.to_string().as_bytes()).unwrap();
}