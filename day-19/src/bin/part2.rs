use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::Range;
use itertools::Itertools;
use regex::Regex;
use crate::Target::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug, PartialEq, Clone)]
enum Target {
    Reject,
    Accept,
    Workflow(String),
}

impl Target {
    fn new(s: String) -> Target {
        use Target::*;

        if s == "R" {
            Reject
        } else if s == "A" {
            Accept
        } else {
            Workflow(s)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Action {
    ApplyTarget(Target),
    GreaterThanThen(String, i32, Target),
    LessThanThen(String, i32, Target),
}

fn process(input: impl BufRead) -> String {
    let elements = input
        .lines()
        .map(|x| x.unwrap())
        .group_by(|x| x == "");
    let mut elements_iter = elements.into_iter();

    let operations_iter = elements_iter.next().unwrap().1;

    let operations_line = Regex::new(r"(?<name>[a-z]+)\{(?<operations>.*)}").unwrap();
    let operation = Regex::new(r"(?<property>[a-z]+)(?<operation>[<>])(?<value>\d+):(?<target>.*)").unwrap();

    let operations_map: HashMap<_, _> = operations_iter
        .map(|x| {
            let result = operations_line.captures(x.as_str()).unwrap();

            let operations = result["operations"]
                .split(",")
                .map(|o| {
                    if let Some(caps) = operation.captures(o) {
                        let value = caps["value"].parse::<i32>().unwrap();
                        let target = Target::new(caps["target"].to_owned());
                        let property = caps["property"].to_owned();

                        match caps["operation"].to_owned().as_str() {
                            ">" => Action::GreaterThanThen(property, value, target),
                            "<" => Action::LessThanThen(property, value, target),
                            a => panic!("unexpected operation {}", a)
                        }
                    } else {
                        Action::ApplyTarget(Target::new(o.to_owned()))
                    }
                })
                .collect_vec();

            (result["name"].to_owned(), operations)
        })
        .collect();

    let mut base_hash_map = HashMap::new();

    base_hash_map.insert("x".to_string(), 1..4001);
    base_hash_map.insert("m".to_string(), 1..4001);
    base_hash_map.insert("a".to_string(), 1..4001);
    base_hash_map.insert("s".to_string(), 1..4001);

    format!("{}", get_value(base_hash_map, "in".to_string(), &operations_map))
}

fn get_value(mut element: HashMap<String, Range<i32>>, workflow: String, operations: &HashMap<String, Vec<Action>>) -> usize {
    use Action::*;

    let mut accepted = 0;

    let operation_list = operations.get(&workflow).unwrap();

    for operation in operation_list {
        match operation {
            ApplyTarget(ref target) => {
                return accepted + match_target(target, element, operations);
            },
            GreaterThanThen(ref property, value, ref target) => {
                let current_range = element.get(property).unwrap();

                if current_range.start > *value {
                    return accepted + match_target(target, element, operations);
                } else if current_range.contains(&(*value + 1)) {
                    let range_outside = current_range.start..*value + 1; // Outside condition
                    let range_inside = *value + 1..current_range.end;

                    element.insert(property.to_string(), range_outside);

                    let mut new_range = element.clone();
                    new_range.insert(property.to_string(), range_inside);
                    accepted += match_target(target, new_range, operations);
                }
            }
            LessThanThen(ref property, value, ref target) => {
                let current_range = element.get(property).unwrap();

                if current_range.end < *value {
                    return accepted + match_target(target, element, operations);
                } else if current_range.contains(&(value - 1)) {
                    let range_inside = current_range.start..*value; // Outside condition
                    let range_outside = *value..current_range.end;

                    element.insert(property.to_string(), range_outside);

                    let mut new_range = element.clone();
                    new_range.insert(property.to_string(), range_inside);
                    accepted += match_target(target, new_range, operations);
                }
            }
        }
    }

    panic!("no matching operation found: {:?} {}", element, workflow)
}

fn match_target(target: &Target, element: HashMap<String, Range<i32>>, operations: &HashMap<String, Vec<Action>>) -> usize {
    match target {
        Accept => {
            element.iter().map(|(_, x)| x.len()).fold(1, |acc, x| acc * x)
        },
        Reject => 0,
        Workflow(s) => get_value(element, s.to_string(), operations),
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(process(data.as_bytes()), "167409079868000");
    }
}