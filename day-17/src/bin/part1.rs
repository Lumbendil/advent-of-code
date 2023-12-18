use std::cmp::{max, min};
use std::collections::{HashMap};
use std::io::{self, BufRead};
use priority_queue::{DoublePriorityQueue};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

fn process(input: impl BufRead) -> String {
    let map: Vec<Vec<usize>> = input.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let start = (0, 0);
    let goal = (map.len() - 1, map[0].len() - 1);

    let value = search_path_cost(&map, start, goal);

    format!("{}", value)
}

fn search_path_cost(map: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    let map_size = (map.len(), map[0].len());

    let mut open_set = DoublePriorityQueue::new();
    open_set.push(start, estimate_distance(start, goal));

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some((current, distance)) = open_set.pop_min() {
        if current == goal {
            return distance
        }

        for neighbour in get_neighbours(current, map_size, &calculate_path(&came_from, current)) {
            let tentative_g_score = g_score.get(&current).unwrap() + map[neighbour.0][neighbour.1];
            let current_g_score = g_score.get(&neighbour).unwrap_or(&usize::MAX);

            if tentative_g_score < *current_g_score {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                open_set.push(neighbour, tentative_g_score + estimate_distance(neighbour, goal));
            }
        }
    }

    panic!("no path found")
}

const NODES_TO_CHECK: usize = 3;

// TODO: Only backtrack n steps
fn calculate_path(came_from: &HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = Vec::new();

    path.push(current);
    while let Some(new_current) = came_from.get(&current) {
        current = *new_current;
        path.push(current);
    }

    path
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn get_restricted_direction(path: &Vec<(usize, usize)>) -> Option<Direction> {
    use Direction::*;

    if path.len() < NODES_TO_CHECK {
        return None
    }

    let nodes_back = NODES_TO_CHECK - if path.len() == NODES_TO_CHECK {1} else {0};

    let (x, y) = path[0];
    let (x2, y2) = path[nodes_back];

    return if x2 + nodes_back == x {
        debug_assert_eq!(y, y2);
        Some(Down)
    } else if x2 == x + nodes_back {
        debug_assert_eq!(y, y2);
        Some(Up)
    } else if y2 + nodes_back == y {
        debug_assert_eq!(x, x2);
        Some(Left)
    } else if y2 == y + nodes_back {
        debug_assert_eq!(x, x2);
        Some(Right)
    } else {
        None
    }
}


fn get_neighbours((x, y): (usize, usize), (height, width): (usize, usize), path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    use Direction::*;
    let mut neighbours = Vec::new();

    let restricted_direction = get_restricted_direction(path);

    if x < height - 1 && restricted_direction != Some(Down) {
        neighbours.push((x + 1, y));
    }

    if x > 0 && restricted_direction != Some(Up) {
        neighbours.push((x - 1, y));
    }

    if y < width - 1 && restricted_direction != Some(Left) {
        neighbours.push((x, y + 1));
    }

    if y > 0 && restricted_direction != Some(Right) {
        neighbours.push((x, y - 1));
    }

    if path.len() > 1 {
        let previous = path[1];
        if let Some(index) = neighbours.iter().position(|x| *x == previous) {
            neighbours.remove(index);
        }
    }

    neighbours
}

fn estimate_distance((x, y): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let a = max(x, x2) - min(x, x2) + max(y, y2) - min(y, y2);

    if a > 1 {
        a - 1
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_neighbours_restricts_paths() {
        assert_eq!(get_neighbours((0, 2), (5, 5), &vec![(0, 2),(0, 1),(0, 0)]), vec![(1,2)]);
        assert_eq!(get_neighbours((0, 0), (5, 5), &vec![(0, 0),(0, 1),(0, 2)]), vec![(1,0)]);
        assert_eq!(get_neighbours((2, 0), (5, 5), &vec![(2, 0),(1, 0),(0, 0)]), vec![(2,1)]);
        assert_eq!(get_neighbours((1, 0), (5, 5), &vec![(1, 0),(2, 0),(3, 0)]), vec![(1,1)]);

        assert_eq!(get_neighbours((0, 2), (5, 5), &vec![(0, 2),(0, 1),(0, 0),(1, 0)]), vec![(1,2), (0, 3)]);
    }

    #[test]
    fn example_works() {
        let data = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        assert_eq!(process(data.as_bytes()), "102");
    }
}