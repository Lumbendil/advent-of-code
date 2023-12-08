use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(mut input: impl BufRead, mut output: impl Write) {
    let mut lines = input.lines();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let instruction_line = lines.next().unwrap().unwrap();
    let instructions = instruction_line.as_bytes();

    lines.next();

    let regex = Regex::new(r"(?<origin>[A-Z]+) = \((?<left>[A-Z]+), (?<right>[A-Z]+)\)").unwrap();

    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = regex.captures(line.as_str()) {
            map.insert(captures["origin"].to_string(), (captures["left"].to_string(), captures["right"].to_string()));
        }
    }

    let mut steps = 0;
    let total_instructions = instructions.len();
    let mut current_instruction = 0;

    let mut current_node = "AAA".to_string();
    let target_node = "ZZZ".to_string();

    while current_node != target_node {
        if current_instruction >= total_instructions {
            current_instruction = 0;
        }

        let action = instructions[current_instruction];
        let current_node_options = map.get(&current_node).unwrap();

        current_node = match action {
            b'L' => (*current_node_options).0.clone(),
            b'R' => (*current_node_options).1.clone(),
            _ => panic!("unexpected action {}", action as char),
        };
        current_instruction += 1;
        steps += 1;
    }

    output.write(steps.to_string().as_bytes()).unwrap();

}