use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::str::FromStr;
use regex::Regex;
use strum_macros::EnumString;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

#[derive(Clone, strum_macros::Display,Eq, Hash, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum CubeColor {
    Red,
    Green,
    Blue,
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut buffer = String::new();
    let mut value = 0;

    let mut game_state = HashMap::new();

    game_state.insert(CubeColor::Red, 12);
    game_state.insert(CubeColor::Green, 13);
    game_state.insert(CubeColor::Blue, 14);


    while let Ok(a) =  input.read_line(&mut buffer) {
        if a == 0 {
            break;
        }

        let result = possible_game(buffer.clone(), game_state.clone());
        buffer.clear();
        value += result;
    }

    output.write(value.to_string().as_bytes()).unwrap();
}

fn possible_game(line: String, state: HashMap<CubeColor, i32>) -> i32 {
    let header_regex = Regex::new(r"Game (\d+): ").unwrap();

    let game_id = header_regex.captures(line.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
        .parse::<i32>()
        .unwrap();

    let game_regex = Regex::new(r"(?<amount>\d+) (?<color>[a-z]+)").unwrap();

    let stripped_line = header_regex.replace(line.as_str(), "");
    let split_result = stripped_line.split(";");

    for part in split_result {
        for (_, [amount, color]) in game_regex.captures_iter(part).map(|c| c.extract()) {
            let amount = amount.to_string().parse::<i32>().unwrap();
            let color = CubeColor::from_str(color).unwrap();

            if *state.get(&color).unwrap() < amount {
                return 0
            }
        }
    }

    game_id
}
