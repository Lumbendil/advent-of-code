use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use core::cmp::{max, min};
use Direction::*;

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
    let mut input: Vec<Vec<char>> = input.lines().map(|x| x.unwrap().chars().collect()).collect();

    let dimension_x = input.len();

    let initial_x = input.iter().position(|x| x.contains(&'S')).unwrap();
    let initial_y = input[initial_x].iter().position(|y| y == &'S').unwrap();

    let initial_position = (initial_x, initial_y);

    let (mut direction,mut position) = find_direction(&input, initial_position, dimension_x);
    let first_direction = direction;

    let mut path_map = HashMap::new();

    let mut min_x: usize = usize::MAX;
    let mut min_y: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    while position != initial_position {
        let (x, y) = position;

        // Define range of positions
        min_x = min(min_x, x);
        max_x = max(max_x, x);
        min_y = min(min_y, y);
        max_y = max(max_y, y);

        path_map.insert(position, input[x][y]);
        (direction, position) = do_move(&input, direction, position);
    }



    let first_position_value = match (first_direction, direction) {
        (Left|Right, Left|Right) => '-',
        (Up|Down, Up|Down) => '|',
        (Up, Left) => 'L',
        (Up, Right) => 'J',
        (Down, Left) => 'F',
        (Down, Right) => '7',
        (_, _) => panic!("Should never happen")
    };

    path_map.insert(initial_position, first_position_value);
    input[initial_position.0][initial_position.1] = first_position_value;

    let value = calculate_inside_nodes(&input, &path_map, (min_x, min_y), (max_x, max_y));

    /*
        let mut new_map = vec![vec!['.'; input[0].len()]; dimension_x];

        for ((x, y), value) in &path_map {
            new_map[*x][*y] = *value;
        }

        for (x, y) in &value {
            new_map[*x][*y] = 'I';
        }


        new_map
            .into_iter()
            .map(|x| x.into_iter().collect())
            .for_each(|x: String| {
                output.write(x.as_bytes()).unwrap();
                output.write("\n".as_bytes()).unwrap();
            });

     */

    output.write(value.len().to_string().as_bytes()).unwrap();
}

fn elements_until_bump(path: &HashMap<(usize, usize), char>, direction: Direction, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut nodes = vec!();
    let (mut x, mut y) = position;

    loop {
        match direction {
            Up => x -= 1,
            Down => x += 1,
            Left => y -= 1,
            Right => y += 1,
        }

        if path.contains_key(&(x, y)) {
            break;
        } else {
            nodes.push((x, y));
        }
    }

    nodes
}
fn calculate_inside_nodes(map: &Vec<Vec<char>>, path: &HashMap<(usize, usize), char>, (min_x, min_y): (usize, usize), (max_x, max_y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut nodes = vec!();

    let (mut inside_direction,initial_position) = find_line_in_border(map, path, (min_x, min_y), (max_x, max_y));
    let mut position = initial_position;

    let mut direction = match inside_direction {
        Left|Right => Up,
        Up|Down => Left,
    };

    loop {
        (direction, position) = do_move(map, direction, position);

        match (inside_direction, map[position.0][position.1]) {
            (Up|Down|Left|Right, '-'|'|') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }

            (Right, 'J') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Down;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Left, 'J') => {
                inside_direction = Up;
            }
            (Down, 'J') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Right;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Up, 'J') => {
                inside_direction = Left;
            }

            (Right, 'L') => {
                inside_direction = Up;
            }
            (Left, 'L') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Down;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Down, 'L') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Left;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Up, 'L') => {
                inside_direction = Right;
            }

            (Up, 'F') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Left;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Down, 'F') => {
                inside_direction = Right;
            }
            (Left, 'F') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Up;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Right, 'F') => {
                inside_direction = Down;
            }


            (Up, '7') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Right;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }
            (Down, '7') => {
                inside_direction = Left;
            }
            (Left, '7') => {
                inside_direction = Down;
            }
            (Right, '7') => {
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
                inside_direction = Up;
                nodes.append(&mut elements_until_bump(path, inside_direction, position));
            }

            (a, b) => panic!("Unexpected case! {:?} {:?}", a, b)
        }

        if position == initial_position {
            break;
        }
    }

    nodes.sort();
    nodes.dedup();
    nodes
}


fn find_line_in_border(map: &Vec<Vec<char>>, path: &HashMap<(usize, usize), char>, (min_x, min_y): (usize, usize), (max_x, max_y): (usize, usize)) -> (Direction, (usize, usize)) {
    for x in min_x..max_x {
        if let Some('|') = path.get(&(x, min_y)) {
            return (Right, (x, min_y));
        }

        if let Some('|') = path.get(&(x, max_y)) {
            return (Left, (x, max_y));
        }
    }

    for y in min_y..max_y {
        if let Some('-') = path.get(&(min_x, y)) {
            return (Up, (min_x, y));
        }

        if let Some('-') = path.get(&(max_x, y)) {
            return (Down, (max_x, y));
        }
    }

    find_line_in_border(map, path, (min_x + 1, min_y + 1), (max_x - 1, max_y - 1))
}

fn do_move(map: &Vec<Vec<char>>, direction: Direction, position: (usize, usize)) -> (Direction, (usize, usize)) {
    try_move(map, direction, position).unwrap()
}
fn try_move(map: &Vec<Vec<char>>, direction: Direction, (x, y): (usize, usize)) -> Option<(Direction, (usize, usize))> {
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
    if x > 0 {
        if try_move(map, Up, (x - 1, y)).is_some() {
            return (Up, (x - 1, y))
        }
    }

    if x < dimension_x - 1 {
        if try_move(map, Down, (x + 1, y)).is_some() {
            return (Down, (x + 1, y))
        }
    }

    if y > 0 {
        if try_move(map, Left, (x, y - 1)).is_some() {
            return (Left, (x, y - 1))
        }
    }

    if try_move(map, Right, (x, y + 1)).is_some() {
        return (Right, (x, y + 1))
    }

    panic!("no direction found for find direction function")
}


