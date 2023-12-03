use std::io::{self, BufRead, Write};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut buffer = String::new();
    let mut previous_line_numbers:  Vec<LocatedNumber> = Vec::new();
    let mut line_numbers:  Vec<LocatedNumber>;

    let mut previous_line_symbols: Vec<LocatedGears> = Vec::new();
    let mut line_symbols: Vec<LocatedGears>;


    let mut value = 0;

    while let Ok(a) =  input.read_line(&mut buffer) {
        if a == 0 {
            break;
        }

        line_symbols = search_symbols(buffer.clone());
        line_numbers = search_numbers(buffer.clone());
        buffer.clear();

        for pos in &line_numbers {
            let position = if pos.position > 0 { pos.position - 1} else {0};
            let position2 = pos.position + pos.len();

            for gear in line_symbols.iter_mut() {
                if gear.position == position || gear.position == position2 {
                    gear.numbers.push(pos.value)
                }
            }

            for i in position..=position2 {
                for gear in previous_line_symbols.iter_mut() {
                    if gear.position == i {
                        gear.numbers.push(pos.value)
                    }
                }
            }
        }

        for pos in &previous_line_numbers {
            let position = if pos.position > 0 { pos.position - 1} else {0};
            let position2 = pos.position + pos.len();

            for i in position..=position2 {
                for gear in line_symbols.iter_mut() {
                    if gear.position == i {
                        gear.numbers.push(pos.value)
                    }
                }
            }
        }

        for gear in previous_line_symbols {
            if gear.numbers.len() == 2 {
                value += gear.numbers.iter().fold(1, |acc, x| acc * x );
            }
        }

        previous_line_numbers = line_numbers.clone();
        previous_line_symbols = line_symbols.clone();
    }

    for gear in previous_line_symbols {
        if gear.numbers.len() == 2 {
            value += gear.numbers.iter().fold(1, |acc, x| acc * x );
        }
    }

    output.write(value.to_string().as_bytes()).unwrap();
}

fn search_symbols(line: String) -> Vec<LocatedGears> {
    let re = Regex::new(r"[*]").unwrap();

    re.captures_iter(line.trim()).map(|res| {
        let re_match = res.get(0).unwrap();
        LocatedGears {
            position: re_match.start(),
            numbers: Vec::new(),
        }
    }).collect()
}

#[derive(Debug, Clone)]
struct LocatedGears {
    position: usize,
    numbers: Vec<i32>
}

#[derive(Debug, Clone)]
struct LocatedNumber {
    position: usize,
    value: i32,
}

impl LocatedNumber {
    fn len(&self) -> usize {
        self.value.ilog10() as usize + 1
    }
}

fn search_numbers(line: String) -> Vec<LocatedNumber> {
    let re = Regex::new(r"\d+").unwrap();

    re.captures_iter(line.as_str()).map(|res| {
        let re_match = res.get(0).unwrap();
        LocatedNumber {
            position: re_match.start(),
            value: re_match.as_str().to_string().parse::<i32>().unwrap()
        }
    }).collect()
}