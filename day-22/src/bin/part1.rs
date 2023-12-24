use std::io::{self, BufRead};
use tuple::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
    z: usize
}
fn process(input: impl BufRead) -> String {
    let data: Vec<_> = input
        .lines()
        .map(|x| {
            x.unwrap()
                .split_once("~")
                .unwrap()
                .map(convert_into_coordinates)
        })
        .collect()
    ;

    dbg!(data);

    format!("{}", 0)
}

fn convert_into_coordinates(text: &str) -> Coordinates {
    let mut source = text.split(",").map(|x| x.parse::<usize>().unwrap());

    Coordinates {
        x: source.next().unwrap(),
        y: source.next().unwrap(),
        z: source.next().unwrap(),
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(process(data.as_bytes()), "5");
    }
}