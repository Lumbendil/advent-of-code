use std::collections::HashMap;
use std::io::{self, BufRead};
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
    elements_iter.next();
    let values = elements_iter.next().unwrap().1;

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

    let value: i32 = values
        .map(|x| {
            let slice = x.as_str();
            let slice2 = slice[1..x.len() - 1].to_owned();

            slice2.split(",")
                .map(|x| {
                    let (name, value) = x.split_once("=").unwrap();

                    (name.to_owned(), value.parse::<i32>().unwrap())
                })
                .collect::<HashMap<_,_>>()
        })
        .map(|x| get_value(x, &operations_map))
        .sum();


    format!("{}", value)
}

fn get_value(element: HashMap<String, i32>, operations: &HashMap<String, Vec<Action>>) -> i32 {
    let mut target = Workflow("in".to_string());

    while let Workflow(ref name) = target {
        target = apply_operations(&element, operations.get(name).unwrap())
    }

    match target {
        Reject => 0,
        Accept => element.iter().map(|(_, &x)| x).sum(),
        x => panic!("unexpected value {:?}", x)
    }
}

fn apply_operations(element: &HashMap<String, i32>, operations: &Vec<Action>) -> Target {
    use Action::*;

    for operation in operations {
        match operation {
            ApplyTarget(ref target) => return target.clone(),
            GreaterThanThen(ref property, ref value, ref target) => {
                if element.get(property).unwrap() > value {
                    return target.clone();
                }
            }
            LessThanThen(ref property, ref value, ref target) => {
                if element.get(property).unwrap() < value {
                    return target.clone();
                }
            }
        }
    }

    panic!("no matching operation found")
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

        assert_eq!(process(data.as_bytes()), "19114");
    }
}