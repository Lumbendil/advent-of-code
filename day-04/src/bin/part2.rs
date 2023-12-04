use std::io::{self, BufRead, Write};
use std::collections::VecDeque;


fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut buffer = String::new();
    let mut value = 0;
    let mut cards: VecDeque<i32> = VecDeque::new();

    while let Ok(a) =  input.read_line(&mut buffer) {
        if a == 0 {
            break;
        }

        let wins = process_line(buffer.clone());
        let cards_solved = cards.pop_front().unwrap_or(0) + 1;

        value += cards_solved;
        for i in 0..(wins as usize) {
            match cards.get(i) {
                Some(x) => {
                    cards[i] = x + cards_solved;
                },
                None => {
                    cards.push_back(cards_solved);
                }
            }
        }

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
         .filter(|x| owned_numbers.contains(&x))
         .count() as i32
}
