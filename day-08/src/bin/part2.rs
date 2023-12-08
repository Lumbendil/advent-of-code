use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(input: impl BufRead, mut output: impl Write) {
    let mut lines = input.lines();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut current_nodes: Vec<String> = Vec::new();

    let instruction_line = lines.next().unwrap().unwrap();
    let instructions = instruction_line.as_bytes();

    let mut times_finished: Vec<Vec<usize>> = Vec::new();

    lines.next();

    let regex = Regex::new(r"(?<origin>[0-9A-Z]+) = \((?<left>[0-9A-Z]+), (?<right>[0-9A-Z]+)\)").unwrap();

    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = regex.captures(line.as_str()) {
            let origin = captures["origin"].to_string();

            map.insert(origin.clone(), (captures["left"].to_string(), captures["right"].to_string()));
            if origin.ends_with("A") {
                current_nodes.push(origin.clone());
                times_finished.push(Vec::new());
            }
        }
    }

    let mut steps = 0;
    let total_instructions = instructions.len();
    let mut current_instruction = 0;

    let mut finished = false;

    while !finished {
        steps += 1;
        if current_instruction >= total_instructions {
            current_instruction = 0;
        }

        let action = instructions[current_instruction];

        let mut j: usize = 0;
        finished = true;

        for current_node in current_nodes.iter_mut() {
            let current_node_options = map.get(current_node).unwrap();

            *current_node = match action {
                b'L' => (*current_node_options).0.clone(),
                b'R' => (*current_node_options).1.clone(),
                _ => panic!("unexpected action {}", action as char),
            };

            if current_node.ends_with("Z") {
                times_finished[j].push(steps);
            }

            let solutions_found = &times_finished[j];

            if solutions_found.len() < 2 {
                finished = false;
            }
            j += 1;
        }

        current_instruction += 1;
    }

    let times = calculate_permutations(1, times_finished);
    let value = times.iter().min().unwrap();

    output.write(value.to_string().as_bytes()).unwrap();

}

fn calculate_permutations(number: usize, reminder: Vec<Vec<usize>>)-> Vec<usize> {
    if reminder.len() == 0 {
        return vec![number];
    }

    // let mut result = Vec::new();
    let (head, tail) = reminder.split_at(1);

    head[0].iter().map(|x| {
        calculate_permutations(num::integer::lcm(*x, number), tail.to_vec())
    }).flatten().collect()
}