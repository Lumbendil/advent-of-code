use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut buffer = String::new();
    let mut value = 0;

    while let Ok(a) =  input.read_line(&mut buffer) {
        if a == 0 {
            break;
        }

        let result = process_line(buffer.clone());
        println!("{} {}", buffer.trim(), result);
        value += result;
        buffer.clear();
    }

    output.write(value.to_string().as_bytes()).unwrap();
}

fn process_line(line: String) -> i32 {
    let mut first_number = 0;
    let mut last_number = 0;

    for c in line.chars() {
        if let Ok(number) = c.to_string().parse::<i32>() {
            if first_number == 0 {
                first_number = number;
            }
            last_number = number;
        }
    }

    (first_number * 10) + last_number
}
