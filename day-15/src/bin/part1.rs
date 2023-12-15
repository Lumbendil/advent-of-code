use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let value: u64 = input.lines()
        .map(|x| {
            x.unwrap().clone().split(',').map(|x| x.to_owned()).collect::<Vec<String>>()
        })
        .flatten()
        .map(|x: String| {
            apply_hash(x.as_bytes())
        })
        .sum();


    format!("{}", value)
}

fn apply_hash(chars: &[u8]) -> u64 {
    chars.into_iter()
        .fold(0, |acc, &ascii| {
            ((acc + ascii as u64) * 17) % 256
        })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(process(data.as_bytes()), "1320");
    }
}