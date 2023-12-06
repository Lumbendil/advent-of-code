use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn line_to_number(line: String) -> f64 {
    line.trim()
        .replace(" ", "")
        .split(':')
        .map(|x| x.trim().parse::<f64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .next()
        .unwrap()
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut line = String::new();

    _ = input.read_line(&mut line);
    let time = line_to_number(line.clone());
    line.clear();
    _ = input.read_line(&mut line);
    let distance = line_to_number(line);

    let sol_1 = ((time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0 - 1.0).ceil() as i64;
    let sol_2 = ((time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0 + 1.0).floor() as i64;

    let value = sol_1 - sol_2 + 1;

    output.write(value.to_string().as_bytes()).unwrap();
}