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
        value += result;
        buffer.clear();
    }

    output.write(value.to_string().as_bytes()).unwrap();
}

fn process_line(line: String) -> i32 {
    let (_, game) = line.split_once(':').unwrap();
    let (winning, owned) = game.split_once('|').unwrap();

    let owned_numbers: Vec<u32> = owned.trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.to_string().parse::<u32>().unwrap()).collect();

     winning
         .trim()
         .split(' ')
         .filter(|x| !x.is_empty())
         .map(|x| x.to_string().parse::<u32>().unwrap())
         .fold(0, |acc, x| if owned_numbers.contains(&x) {if acc == 0 { 1 } else { acc * 2 }} else {acc})
}
