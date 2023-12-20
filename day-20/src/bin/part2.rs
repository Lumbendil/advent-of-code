use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug, PartialEq)]
enum OperationType {
    FlipFlop,
    Conjunction,
    None,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SignalLevel {
    High,
    Low,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct PulsesSent {
    high: usize,
    low: usize,
}

#[derive(Debug, Clone)]
struct CurrentState {
    flip_flops:  HashMap<String, bool>,
    conjunctions: HashMap<String, HashMap<String, SignalLevel>>,
}
#[derive(Debug, PartialEq)]
struct Operation {
    op_type: OperationType,
    targets: Vec<String>,
}

fn process(input: impl BufRead) -> String {
    let operations: HashMap<String, Operation> = input
        .lines()
        .map(|x| x.unwrap())
        .map(parse_line)
        .collect()
    ;

    let mut current_state = CurrentState{
        flip_flops: HashMap::new(),
        conjunctions: HashMap::new(),
    };

    current_state.flip_flops = operations.iter()
        .filter(|&(_, operation)| {
            operation.op_type == OperationType::FlipFlop
        })
        .map(|(x, _)| (x.to_owned(), false))
        .collect();

    current_state.conjunctions = operations.iter()
        .filter(|&(_, operation)| {
            operation.op_type == OperationType::Conjunction
        })
        .map(|(x, _)| (x.to_owned(), HashMap::new()))
        .collect();

    for (source, operation) in &operations {
        for target in &operation.targets {
            if let Some(value) = current_state.conjunctions.get_mut(target) {
                value.insert(source.to_owned(), SignalLevel::Low);
            }
        }
    }

    let mut button_presses = 0;

    loop {
        let success = press_button(&operations, &mut current_state);
        button_presses += 1;

        if success {
            return format!("{}", button_presses)
        }
    }
}

fn parse_line(line: String) -> (String, Operation) {
    let (operation, targets_string) = line.split_once(" -> ").unwrap();

    let op_type = match operation.chars().next().unwrap() {
        '%' => OperationType::FlipFlop,
        '&' => OperationType::Conjunction,
        _ => OperationType::None,
    };

    let name: String = if op_type == OperationType::None {
        operation.trim()
    } else {
        operation.get(1..).unwrap().trim()
    }.to_owned();

    let targets: Vec<String> = targets_string.split(",").map(|x| x.trim().to_owned()).collect();

    (
        name,
        Operation {
            op_type,
            targets,
        }
    )
}

fn press_button(operations: &HashMap<String, Operation>, state: &mut CurrentState) -> bool {
    let mut pending_operations: VecDeque<(SignalLevel, &String, &String)> = VecDeque::new();

    // Initial State
    let broadcaster = "broadcaster".to_string();
    for target in operations.get(&broadcaster).unwrap().targets.iter() {
        pending_operations.push_back((SignalLevel::Low, target, &broadcaster));
    }

    while let Some((level, node, source)) = pending_operations.pop_front() {
        if node == "rx" && level == SignalLevel::Low {
            return true
        }

        if let Some(operation) = operations.get(node) {
            let targets = operation.targets.iter();

            match operation.op_type {
                OperationType::FlipFlop => {
                    match level {
                        SignalLevel::High => (),
                        SignalLevel::Low => {
                            let current_state = state.flip_flops.get_mut(node).unwrap();
                            *current_state = ! *current_state;

                            let output_signal = if *current_state { SignalLevel::High } else { SignalLevel::Low };

                            for target in targets {
                                pending_operations.push_back((output_signal, target, node));
                            }
                        }
                    }
                },
                OperationType::Conjunction => {
                    let current_state = state.conjunctions.get_mut(node).unwrap();
                    let current_node_state = current_state.get_mut(source).unwrap();
                    *current_node_state = level;

                    let output_signal = if current_state.iter().all(|(_, &x)| x == SignalLevel::High) { SignalLevel::Low } else { SignalLevel::High };

                    for target in targets {
                        pending_operations.push_back((output_signal, target, node));
                    }
                },
                OperationType::None => panic!("unexpected operation")
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(process(data.as_bytes()), "32000000");
    }

    #[test]
    fn example_2_works() {
        let data = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(process(data.as_bytes()), "11687500");
    }
}