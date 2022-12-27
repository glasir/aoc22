use std::collections::HashMap;

use nom::{
    character::complete::{one_of, u32},
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

/*
 * Day 22 asks us to move around a strangely-shaped map filled with
 * obstacles. In part 1, we wrap when we go off an edge; in part 2,
 * it turns out we're actually moving on a cube, so we have to handle
 * the edge transitions very differently.
 * 
 * I am pretty happy with my solution for part 1, and extremely unhappy
 * with my solution for part 2, which relies on hardcoding the edge
 * transitions for my specific input shape.
 * 
 * So, I've put much less effort into cleaning up and commenting the code
 * for this day's puzzle.
 */

#[derive(Debug, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            'U' => Self::Up,
            '\n' => Self::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Open,
    Solid,
}

#[derive(Debug, Clone)]
pub enum MapType {
    Torus,
    Cube,
}

#[derive(Clone)]
pub struct Map {
    points: HashMap<(usize, usize), Cell>,
    row_bounds: Vec<(usize, usize)>,
    col_bounds: Vec<(usize, usize)>,

    structure: MapType,
}

impl Map {
    fn walk(
        &self,
        start_row: usize,
        start_col: usize,
        count: usize,
        initial_direction: &Direction,
    ) -> (usize, usize, Direction) {
        let (mut row, mut col) = (start_row, start_col);
        let mut direction = initial_direction.to_owned();

        for _ in 0..count {
            // Figure which cell is the next in that direction (accounting for wrapping).
            let (next_row, next_col, next_direction) = match self.structure {
                MapType::Torus => self.neighbor_torus(row, col, &direction),
                MapType::Cube => self.neighbor_cube(row, col, &direction),
            };

            // If that cell is blocked, we won't be able to move any further in that direction.
            // So return early.
            if matches!(self.points[&(next_row, next_col)], Cell::Solid) {
                return (row, col, direction);
            }

            // Otherwise, that cell is empty; move into it.
            (row, col, direction) = (next_row, next_col, next_direction);
        }

        (row, col, direction)
    }

    /**
     * Returns the neighbor of a given cell in a given direction,
     * wrapping when we get to the edges (i.e., part 1).
     */
    fn neighbor_torus(
        &self,
        row: usize,
        col: usize,
        direction: &Direction,
    ) -> (usize, usize, Direction) {
        let dir = direction.to_owned();
        match direction {
            Direction::Right => {
                if col == self.row_bounds[row].1 {
                    (row, self.row_bounds[row].0, dir)
                } else {
                    (row, col + 1, dir)
                }
            }
            Direction::Down => {
                if row == self.col_bounds[col].1 {
                    (self.col_bounds[col].0, col, dir)
                } else {
                    (row + 1, col, dir)
                }
            }
            Direction::Left => {
                if col == self.row_bounds[row].0 {
                    (row, self.row_bounds[row].1, dir)
                } else {
                    (row, col - 1, dir)
                }
            }
            Direction::Up => {
                if row == self.col_bounds[col].0 {
                    (self.col_bounds[col].1, col, dir)
                } else {
                    (row - 1, col, dir)
                }
            }
        }
    }

    /**
     * My input is laid out like this:
     *               (0, 50)--F---(0, 100)--G---(0,150)
     *                  |            |             |
     *                  A            |             D
     *                  |            |             |
     *              (50, 50)------(50, 100)--C--(50,150)
     *                  |            |
     *                  B            C
     *                  |            |
     * (100, 0)--B-(100, 50)----(100, 100)
     *    |             |            |
     *    A             |            D
     *    |             |            |
     * (150, 0)----(150, 50)--E-(150, 100)
     *    |             |
     *    F             E
     *    |             |
     * (200, 0)-G--(200, 50)
     *
     * There is a *lot* of casework to handle moving across the edges.
     * It is probably the worst thing I have ever written.
     */
    fn neighbor_cube(
        &self,
        row: usize,
        col: usize,
        direction: &Direction,
    ) -> (usize, usize, Direction) {
        match direction {
            Direction::Right => {
                if row < 50 && col == 149 {
                    (149 - row, 99, Direction::Left)
                } else if (50..100).contains(&row) && col == 99 {
                    (49, 100 + (row - 50), Direction::Up)
                } else if (100..150).contains(&row) && col == 99 {
                    (49 - (row - 100), 149, Direction::Left)
                } else if 150 <= row && col == 49 {
                    (149, 50 + (row - 150), Direction::Up)
                } else {
                    (row, col + 1, direction.clone())
                }
            }
            Direction::Down => {
                if row == 199 && col < 50 {
                    (0, col + 100, Direction::Down)
                } else if row == 149 && (50..100).contains(&col) {
                    (150 + (col - 50), 49, Direction::Left)
                } else if row == 49 && (100..150).contains(&col) {
                    (50 + (col - 100), 99, Direction::Left)
                } else {
                    (row + 1, col, direction.clone())
                }
            }
            Direction::Left => {
                if row < 50 && col == 50 {
                    (149 - row, 0, Direction::Right)
                } else if (50..100).contains(&row) && col == 50 {
                    (100, row - 50, Direction::Down)
                } else if (100..150).contains(&row) && col == 0 {
                    (49 - (row - 100), 50, Direction::Right)
                } else if 150 <= row && col == 0 {
                    (0, 50 + (row - 150), Direction::Down)
                } else {
                    (row, col - 1, direction.clone())
                }
            }
            Direction::Up => {
                if row == 100 && col < 50 {
                    (50 + col, 50, Direction::Right)
                } else if row == 0 && (50..100).contains(&col) {
                    (150 + (col - 50), 0, Direction::Right)
                } else if row == 0 && (100..150).contains(&col) {
                    (199, col - 100, Direction::Up)
                } else {
                    (row - 1, col, direction.clone())
                }
            }
        }
    }
}

type Path = Vec<(usize, Direction)>;

struct You {
    row: usize,
    col: usize,
    facing: Direction,
}

impl You {
    fn password(&self) -> usize {
        let facing_value = match self.facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        1000 * (1 + self.row) + 4 * (1 + self.col) + facing_value
    }
}

impl You {
    fn turn(&mut self, direction: &Direction) {
        match *direction {
            Direction::Right => {
                self.facing = match self.facing {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
            Direction::Left => {
                self.facing = match self.facing {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                }
            }
            _ => {}
        }
    }
}

fn parse_map(input: &str) -> Map {
    let mut points = HashMap::new();
    let mut row_bounds = Vec::new();
    let mut num_cols = 0;

    for (row, line) in input.lines().enumerate() {
        let row_start = line.find(|c| c != ' ').unwrap();
        let row_end = line.rfind(|c| c != ' ').unwrap();
        row_bounds.push((row_start, row_end));
        num_cols = num_cols.max(row_end);

        for (col, c) in line
            .as_bytes()
            .iter()
            .enumerate()
            .take(row_end + 1)
            .skip(row_start)
        {
            let cell = match c {
                b'.' => Cell::Open,
                b'#' => Cell::Solid,
                c => panic!("unexpected map character {}", *c as char),
            };
            points.insert((row, col), cell);
        }
    }

    // Figure out the points at which each column wraps.
    let col_bounds = (0..=num_cols)
        .map(|col| {
            points
                .keys()
                .filter(|(_, point_col)| col == *point_col)
                .fold((usize::MAX, 0), |bound, point| {
                    (bound.0.min(point.0), bound.1.max(point.0))
                })
        })
        .collect();

    Map {
        points,
        row_bounds,
        col_bounds,
        structure: MapType::Torus,
    }
}

fn parse_path(input: &str) -> Path {
    let parsed: IResult<&str, Vec<(usize, Direction)>> = many1(map(
        tuple((u32, opt(one_of("RDLU")))),
        |(count, maybe_dir)| {
            maybe_dir.map_or((count as usize, Direction::Up), |dir| {
                (count as usize, Direction::from(dir))
            })
        },
    ))(input);

    parsed.expect("error parsing path").1
}

#[aoc_generator(day22)]
fn generator(input: &str) -> (Map, Path) {
    let (map_str, path_str) = input.split_once("\n\n").unwrap().to_owned();

    (parse_map(map_str), parse_path(path_str))
}

#[aoc(day22, part1)]
pub fn part1((map, path): &(Map, Path)) -> usize {
    let mut you = You {
        row: 0,
        col: map.row_bounds[0].0,
        facing: Direction::Right,
    };

    for (count, direction) in path {
        (you.row, you.col, you.facing) = map.walk(you.row, you.col, *count, &you.facing);
        you.turn(direction);
    }

    you.password()
}

#[aoc(day22, part2)]
pub fn part2((initial_map, path): &(Map, Path)) -> usize {
    let mut map = initial_map.clone();
    map.structure = MapType::Cube;

    part1(&(map, path.clone()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{generator, part1};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day22.txt").expect("missing input");
        let parsed = generator(&input);
        assert_eq!(part1(&parsed), 6032);
    }

    // Because the cube edge transitions are hardcoded for 50x50 faces
    // with my input's format, they don't work at all for the example.
    // So, no test for part two.
}
