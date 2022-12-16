use std::collections::HashMap;

use pathfinding::directed::dijkstra::dijkstra;

pub struct HeightMap {
    points: HashMap<(i32, i32), u8>,
    width: i32,
    height: i32,

    start: (i32, i32),
    end: (i32, i32),
}

impl HeightMap {
    fn new() -> Self {
        HeightMap {
            points: HashMap::new(),
            width: 0,
            height: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

fn neighbors(from: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (from.0 - 1, from.1),
        (from.0 + 1, from.1),
        (from.0, from.1 - 1),
        (from.0, from.1 + 1),
    ]
}

// Returns a list of the points in the grid you could step to from `from`.
// To make working with the dijkstra implementation easier, it returns
// a pair (point, cost), where cost is always equal to 1 for this problem.
fn next_steps(map: &HeightMap, from: (i32, i32)) -> Vec<((i32, i32), i32)> {
    let start_height = map.points[&from];

    neighbors(from)
        .iter()
        .filter(|to| {
            map.points
                .get(to)
                .filter(|height| **height <= start_height + 1)
                .is_some()
        })
        .map(|p| (*p, 1))
        .collect()
}

// Returns a list of the points in the grid from which you could step to `to`.
fn prev_steps(map: &HeightMap, to: (i32, i32)) -> Vec<((i32, i32), i32)> {
    let end_height = map.points.get(&to).unwrap();

    neighbors(to)
        .iter()
        .filter(|from| {
            map.points
                .get(from)
                .filter(|height| **height >= end_height - 1)
                .is_some()
        })
        .map(|p| (*p, 1))
        .collect()
}

#[aoc_generator(day12)]
fn generator(input: &[u8]) -> HeightMap {
    let mut row: i32 = 0;
    let mut col: i32 = 0;

    let mut result = HeightMap::new();

    for c in input {
        match c {
            b'\n' => {
                result.width = col;
                row += 1;
                col = 0;
            }
            b'S' => {
                result.points.insert((row, col), 0);
                result.start = (row, col);
                col += 1;
            }
            b'E' => {
                result.points.insert((row, col), 25);
                result.end = (row, col);
                col += 1;
            }
            _ => {
                result.points.insert((row, col), c - b'a');
                col += 1;
            }
        }
    }
    result.height = row;

    result
}

#[aoc(day12, part1)]
pub fn part1(input: &HeightMap) -> i32 {
    let (_, length) =
        dijkstra(&input.start, |p| next_steps(input, *p), |p| *p == input.end).unwrap();

    length
}

#[aoc(day12, part2)]
pub fn part2(input: &HeightMap) -> i32 {
    let (_, length) = dijkstra(
        &input.end,
        |p| prev_steps(input, *p),
        |p| input.points[p] == 0,
    )
    .unwrap();

    length
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "Sabqponm\n\
                           abcryxxl\n\
                           accszExk\n\
                           acctuvwj\n\
                           abdefghi\n";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE.as_bytes());
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE.as_bytes());
        assert_eq!(part2(&input), 29);
    }
}
