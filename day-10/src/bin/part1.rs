use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn process(input: impl BufRead, mut output: impl Write) {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.unwrap().chars().collect()).collect();

    let dimension_x = input.len();

    let initial_x = input.iter().position(|x| x.contains(&'S')).unwrap();
    let initial_y = input[initial_x].iter().position(|y| y == &'S').unwrap();

    let initial_position = (initial_x, initial_y);

    let (mut direction,mut position) = find_direction(&input, initial_position, dimension_x);
    let mut distance: f64 = 1.0;

    while position != initial_position {
        (direction, position) = do_move(&input, direction, position);
        distance += 1.0;
    }

    let value = (distance / 2.0).ceil() as u32;

    output.write(value.to_string().as_bytes()).unwrap();
}

fn do_move(map: &Vec<Vec<char>>, direction: Direction, position: (usize, usize)) -> (Direction, (usize, usize)) {
    try_move(map, direction, position).unwrap()
}
fn try_move(map: &Vec<Vec<char>>, direction: Direction, (x, y): (usize, usize)) -> Option<(Direction, (usize, usize))> {
    use Direction::*;

    match direction {
        Up => {
            match map[x][y] {
                '|' => Some((Up, (x - 1, y))),
                '7' => Some((Left, (x, y - 1))),
                'F' => Some((Right, (x, y + 1))),
                _ => None,
            }
        }
        Down => {
            match map[x][y] {
                '|' => Some((Down, (x + 1, y))),
                'L' => Some((Right, (x, y + 1))),
                'J' => Some((Left, (x, y - 1))),
                _ => None,
            }
        }
        Left => {
            match map[x][y] {
                '-' => Some((Left, (x, y - 1))),
                'F' => Some((Down, (x + 1, y))),
                'L' => Some((Up, (x - 1, y))),
                _ => None,
            }
        }
        Right => {
            match map[x][y] {
                '-' => Some((Right, (x, y + 1))),
                '7' => Some((Down, (x + 1, y))),
                'J' => Some((Up, (x - 1, y))),
                _ => None,
            }
        }
    }
}

fn find_direction(map: &Vec<Vec<char>>, (x, y): (usize, usize), dimension_x: usize)-> (Direction, (usize, usize)) {
    use Direction::*;

    if x > 0 {
        if let Some((dir, point)) = try_move(map, Up, (x - 1, y)) {
            return (dir, point)
        }
    }

    if x < dimension_x - 1 {
        if let Some((dir, point))  = try_move(map, Down, (x + 1, y)) {
            return (dir, point)
        }
    }

    if y > 0 {
        if let Some((dir, point))  = try_move(map, Left, (x, y - 1)) {
            return (dir, point)
        }
    }

    try_move(map, Right, (x, y + 1)).unwrap()
}


