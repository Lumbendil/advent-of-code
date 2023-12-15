use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug)]
struct Element {
    label: String,
    focus_power: Option<usize>,
}

fn process(input: impl BufRead) -> String {
    const VAL: Vec<Element> = vec![];

    let mut boxes = [VAL; 256];

    let lenses = input.lines()
        .map(|x| {
            x.unwrap().clone().split(',').map(|x| x.to_owned()).collect::<Vec<String>>()
        })
        .flatten()
        .map(|mut lens: String| {
            let last_char = lens.pop().unwrap();

            match last_char {
                '-' => {
                    Element {
                        label: lens,
                        focus_power: None,
                    }
                },
                x =>  {
                    let focus_power = x.to_digit(10).unwrap() as usize;
                    lens.pop();

                    Element {
                        label: lens,
                        focus_power: Some(focus_power),
                    }
                }
            }
        });

    for lens in lenses {
        let box_index = apply_hash(lens.label.as_bytes());

        match lens.focus_power {
            Some(_) => {
                match boxes[box_index].iter().position(|x| x.label == lens.label) {
                    None => boxes[box_index].push(lens),
                    Some(i) => {
                        boxes[box_index][i] = lens;
                    },
                }

            },
            None => {
                match boxes[box_index].iter().position(|x| x.label == lens.label) {
                    None => (),
                    Some(i) => {
                        boxes[box_index].remove(i);
                    },
                }
            }
        }
    }

    let value: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            (box_index + 1) * lenses
                .into_iter()
                .enumerate()
                .map(|(slot_index, element)| {
                    (slot_index + 1) * element.focus_power.unwrap()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    format!("{}", value)
}

fn apply_hash(chars: &[u8]) -> usize {
    chars.into_iter()
        .fold(0, |acc, &ascii| {
            ((acc + ascii as usize) * 17) % 256
        })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(process(data.as_bytes()), "145");
    }
}