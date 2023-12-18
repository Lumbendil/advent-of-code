use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use priority_queue::{DoublePriorityQueue, PriorityQueue};

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


/*
 1  function Dijkstra(Graph, source):
 2
 3      for each vertex v in Graph.Vertices:
 4          dist[v] ← INFINITY
 5          prev[v] ← UNDEFINED
 6          add v to Q
 7      dist[source] ← 0
 8
 9      while Q is not empty:
10          u ← vertex in Q with min dist[u]

if u = target
1  S ← empty sequence
2  u ← target
3  if prev[u] is defined or u = source:          // Do something only if the vertex is reachable
4      while u is defined:                       // Construct the shortest path with a stack S
5          insert u at the beginning of S        // Push the vertex onto the stack
6          u ← prev[u]                           // Traverse from target to source


11          remove u from Q
12
13          for each neighbor v of u still in Q:
14              alt ← dist[u] + Graph.Edges(u, v)
15              if alt < dist[v]:
16                  dist[v] ← alt
17                  prev[v] ← u
18
19      return dist[], prev[]
 */
fn search_path_cost(map: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    let map_size = (map.len(), map[0].len());

    let mut open_set = DoublePriorityQueue::new();



























    open_set.push(start, estimate_distance(start, goal));
    // dbg!(&open_set);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some((current, distance)) = open_set.pop_min() {
        if current == goal {
            return distance
        }

        dbg!(calculate_path(&came_from, current));
        for neighbour in get_neighbours(current, map_size, &came_from) {
            dbg!(&neighbour);
            let tentative_g_score = g_score.get(&current).unwrap() + map[neighbour.0][neighbour.1];
            let current_g_score = g_score.get(&neighbour).unwrap_or(&usize::MAX);

            if tentative_g_score < *current_g_score {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                open_set.push(neighbour, tentative_g_score + estimate_distance(neighbour, goal));
            }
        }

        // dbg!(&open_set);
    }

    panic!("no path found")
}

fn calculate_path_cost(came_from: HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let mut value = 0;

    while let Some(new_current) = came_from.get(&current) {
        // dbg!(current);
        value += map[current.0][current.1];
        current = *new_current;
    }

    value
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

    let &(x, y) = path.last().unwrap();
    let (x2, y2) = path[path.len() - NODES_TO_CHECK];

    return if x2 + NODES_TO_CHECK - 1 == x {
        debug_assert_eq!(y, y2);
        Some(Up)
    } else if x2 == x + NODES_TO_CHECK - 1 {
        debug_assert_eq!(y, y2);
        Some(Down)
    } else if y2 + NODES_TO_CHECK - 1 == y {
        debug_assert_eq!(x, x2);
        Some(Right)
    } else if y2 == y + NODES_TO_CHECK - 1 {
        debug_assert_eq!(x, x2);
        Some(Left)
    } else {
        None
    }
}


fn get_neighbours((x, y): (usize, usize), (height, width): (usize, usize), came_from: &HashMap<(usize, usize), (usize, usize)>) -> Vec<(usize, usize)> {
    use Direction::*;
    let mut neighbours = Vec::new();

    let restricted_direction = get_restricted_direction(&calculate_path(came_from, (x, y)));

    if x < height - 1 && restricted_direction != Some(Up) {
        neighbours.push((x + 1, y));
    }

    if x > 0 && restricted_direction != Some(Down) {
        neighbours.push((x - 1, y));
    }

    if y < width - 1 && restricted_direction != Some(Left) {
        neighbours.push((x, y + 1));
    }

    if y > 0 && restricted_direction != Some(Right) {
        neighbours.push((x, y - 1));
    }

    if let Some(previous) = came_from.get(&(x, y)) {
        if let Some(index) = neighbours.iter().position(|x| x == previous) {
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
    fn test_estimate_distance() {
        assert_eq!(estimate_distance((0, 0), (10, 10)), 20);
        assert_eq!(estimate_distance((1, 0), (10, 10)), 19);
        assert_eq!(estimate_distance((0, 1), (10, 10)), 19);
        assert_eq!(estimate_distance((9, 9), (10, 10)), 2);
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