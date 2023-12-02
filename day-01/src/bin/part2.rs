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

    let number_names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].map(|x| x.as_bytes());
    let mut number_positions = [0; 9];

    for c in line.chars() {
        if let Ok(number) = c.to_string().parse::<i32>() {
            if first_number == 0 {
                first_number = number;
            }
            last_number = number;
        } else {
            for i in 0..9 {
                let val = number_names[i][number_positions[i]] as char;
                if val == c {
                    number_positions[i] += 1;

                    if number_positions[i] == number_names[i].len() {
                        number_positions[i] = 0;
                        let number = (i as i32) + 1;
                        if first_number == 0 {
                            first_number = number;
                        }
                        last_number = number;
                    }
                } else {
                    if (number_names[i][0] as char) == c {
                        number_positions[i] = 1;
                    } else {
                        number_positions[i] = 0;
                    }
                }
                
            }
        }
    }

    (first_number * 10) + last_number
}
