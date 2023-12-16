use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use itertools::Itertools;
use Direction::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("{}", process(handle));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum MapItem {
    SplitterHorizontal,
    SplitterVertical,
    MirrorLeftDown,
    MirrorLeftUp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct BeamDirection {
    x: usize,
    y: usize,
    direction: Direction,
}

impl BeamDirection {
    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn apply_direction(&self, direction: Direction, map_height: usize, map_width: usize) -> Option<BeamDirection> {
        match (direction, self.get_position()) {
            (Left, (x, y)) if y < map_width - 1 => Some(BeamDirection{x, y: y + 1, direction: Left}),
            (Right, (x, y)) if y > 0 => Some(BeamDirection{x, y: y - 1, direction: Right}),
            (Up, (x, y)) if x > 0 => Some(BeamDirection{x: x - 1, y, direction: Up}),
            (Down, (x, y)) if x < map_height - 1 => Some(BeamDirection{x: x + 1, y, direction: Down}),
            _ => None,
        }
    }
}


fn process(input: impl BufRead) -> String {
    let mut map_height = 0;
    let mut map_width = 0;

    let map: HashMap<(usize, usize), MapItem> = input.lines()
        .enumerate()
        .map(|(x, line)| {
            let line = line.unwrap();

            map_width = line.len();
            map_height = x + 1;

            line.char_indices().filter_map(|(y, char)| {
                match char {
                    '-' => Some(((x, y), MapItem::SplitterHorizontal)),
                    '|' => Some(((x, y), MapItem::SplitterVertical)),
                    '\\' => Some(((x, y), MapItem::MirrorLeftDown)),
                    '/' => Some(((x, y),  MapItem::MirrorLeftUp)),

                    _ => None,
                }
            })
            .collect::<HashMap<(usize, usize), MapItem>>()
        })
        .flatten()
        .collect();

    let all_directions =
        (0..map_height).map(|x| {
            BeamDirection{x, y: 0, direction: Left}
        }).chain(
            (0..map_height).map(|x| {
                BeamDirection{x, y: map_width - 1, direction: Right}
            })
        ).chain(
            (0..map_width).map(|y| {
                BeamDirection{x: 0, y, direction: Down}
            })
        ).chain(
            (0..map_width).map(|y| {
                BeamDirection{x: map_height - 1, y, direction: Down}
            })
        );

    let value: usize = all_directions.map(|initial_direction| {
        let mut pending_beams = vec![initial_direction];
        let mut beam_paths: HashSet<BeamDirection> = Default::default();

        while let Some(beam_direction) = pending_beams.pop() {
            if !beam_paths.insert(beam_direction) {
                continue;
            }
            match map.get(&beam_direction.get_position()) {
                None => {
                    if let Some(new_direction) = beam_direction.apply_direction(beam_direction.direction, map_height, map_width) {
                        pending_beams.push(new_direction);
                    }
                },
                Some(MapItem::MirrorLeftDown) => {
                    let new_direction = match beam_direction.direction {
                        Left => Down,
                        Right => Up,
                        Down => Left,
                        Up => Right,
                    };

                    if let Some(new_direction) = beam_direction.apply_direction(new_direction, map_height, map_width) {
                        pending_beams.push(new_direction);
                    }
                },
                Some(MapItem::MirrorLeftUp) => {
                    let new_direction = match beam_direction.direction {
                        Left => Up,
                        Right => Down,
                        Down => Right,
                        Up => Left,
                    };

                    if let Some(new_direction) = beam_direction.apply_direction(new_direction, map_height, map_width) {
                        pending_beams.push(new_direction);
                    }
                },
                Some(MapItem::SplitterHorizontal) => {
                    match beam_direction.direction {
                        Right|Left => {
                            if let Some(new_direction) = beam_direction.apply_direction(beam_direction.direction, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                        },
                        Up|Down => {
                            if let Some(new_direction) = beam_direction.apply_direction(Right, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                            if let Some(new_direction) = beam_direction.apply_direction(Left, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                        }
                    }
                },
                Some(MapItem::SplitterVertical) => {
                    match beam_direction.direction {
                        Up|Down => {
                            if let Some(new_direction) = beam_direction.apply_direction(beam_direction.direction, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                        },
                        Right|Left => {
                            if let Some(new_direction) = beam_direction.apply_direction(Up, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                            if let Some(new_direction) = beam_direction.apply_direction(Down, map_height, map_width) {
                                pending_beams.push(new_direction);
                            }
                        }
                    }
                },
            }
        }


        beam_paths
            .into_iter()
            .map(|beam_direction: BeamDirection| {
                (beam_direction.x, beam_direction.y)
            })
            .sorted()
            .dedup()
            .count()
    })
    .max()
    .unwrap();

    format!("{}", value)
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let data = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        assert_eq!(process(data.as_bytes()), "51");
    }
}