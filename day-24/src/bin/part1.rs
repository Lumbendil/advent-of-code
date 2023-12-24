use std::io::{self, BufRead};
use itertools::Itertools;
use tuple::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug, Copy, Clone)]
struct ThreeDCoordinate {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Copy, Clone)]
struct Hailstorm {
    position: ThreeDCoordinate,
    speed: ThreeDCoordinate,
}

impl Hailstorm {
    fn intersection(&self, other: &Hailstorm) -> Intersection {
        let (a, c) = self.line_definition();
        let (b, d) = other.line_definition();

        if a == b {
            Intersection::Parallel
        } else {
            // Formula: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
            let px = (d - c) / (a - b);
            let py = a * px + c;

            let point = (px, py);

            if self.is_past(point) || other.is_past(point) {
                Intersection::Past
            } else {
                Intersection::Cross(ThreeDCoordinate{
                    x: px,
                    y: py,
                    z: 0.0
                })
            }
        }
    }

    fn is_past(&self, (x, y): (f64, f64)) -> bool {
        self.speed.x > 0.0 && x < self.position.x
            || self.speed.x < 0.0 && x > self.position.x
            || self.speed.y < 0.0 && y > self.position.y
            || self.speed.y < 0.0 && y > self.position.y
    }

    // y = return.0 * x + return.1
    fn line_definition(&self) -> (f64, f64) {
        (
            self.speed.y / self.speed.x,
            self.position.y - self.speed.y * self.position.x / self.speed.x
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Intersection {
    Past,
    // TODO: This case is not being handled: Overlap(Hailstorm),
    Parallel,
    Cross(ThreeDCoordinate)
}

impl Intersection {
    fn within(&self, (x0, x1): (f64, f64), (y0, y1): (f64, f64)) -> bool {
        match self {
            Intersection::Past => false,
            Intersection::Parallel => false,
            Intersection::Cross(coordinates) => coordinates.x >= x0 && coordinates.x <= x1 && coordinates.y >= y0 && coordinates.y <= y1,
        }
    }
}

const MINIMUM_COORDINATE: f64 = 200000000000000f64;
const MAXIMUM_COORDINATE: f64 = 400000000000000f64;
fn process(input: impl BufRead) -> String {
    let coordinate = (MINIMUM_COORDINATE, MAXIMUM_COORDINATE);

    format!("{}", process_bounded(input, coordinate, coordinate))
}

fn process_bounded(input: impl BufRead, x_limits: (f64, f64), y_limits: (f64, f64)) -> usize {
    let data = input.lines()
        .map(|x| x.unwrap())
        .map(parse_line)
        .collect_vec()
    ;

    data.into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            a.intersection(&b)
        })
        .filter(|x| {
            x.within(x_limits, y_limits)
        })
        .count()
}

fn parse_line(line: String) -> Hailstorm {
    let (position, speed) = line.split_once("@")
        .unwrap()
        .map(|x| x.trim())
        .map(|x| {
            let numbers = x.split(",")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect_vec();

            ThreeDCoordinate{
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
    ;

    Hailstorm{
        position,
        speed,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(process_bounded(data.as_bytes(), (7.0, 27.0), (7.0, 27.0)), 2);
    }
}