use std::{collections::HashSet, fmt};

use pathfinding::prelude::astar;

#[derive(Clone, Debug)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

type Point = (i32, i32);
type Blizzard = (Point, Direction);

#[derive(Clone)]
pub struct State {
    obstacles: HashSet<Point>,
    blizzards: Vec<Blizzard>,
    dimensions: (i32, i32),
    start: Point,
    end: Point,
}

impl State {
    fn next(&self) -> State {
        // Move each blizzard one step forward (wrapping if necessary).
        let mut blizzards = Vec::new();
        let mut obstacles = HashSet::new();

        for blizzard in self.blizzards.iter() {
            let new_blizzard = self.move_blizzard(blizzard);
            obstacles.insert(new_blizzard.0);
            blizzards.push(new_blizzard);
        }

        State {
            obstacles,
            blizzards,
            dimensions: self.dimensions,
            start: self.start,
            end: self.end,
        }
    }

    fn move_blizzard(&self, blizzard: &Blizzard) -> Blizzard {
        let coords = &blizzard.0;
        let new_coords = match blizzard.1 {
            Direction::Right => {
                if coords.1 == self.dimensions.1 - 1 {
                    (coords.0, 0)
                } else {
                    (coords.0, coords.1 + 1)
                }
            }
            Direction::Down => {
                if coords.0 == self.dimensions.0 - 1 {
                    (0, coords.1)
                } else {
                    (coords.0 + 1, coords.1)
                }
            }
            Direction::Left => {
                if coords.1 == 0 {
                    (coords.0, self.dimensions.1 - 1)
                } else {
                    (coords.0, coords.1 - 1)
                }
            }
            Direction::Up => {
                if coords.0 == 0 {
                    (self.dimensions.0 - 1, coords.1)
                } else {
                    (coords.0 - 1, coords.1)
                }
            }
        };

        (new_coords, blizzard.1.clone())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The first row is always a bunch of #'s, with one gap at the start point.
        for col in 0..=self.dimensions.1 + 1 {
            if col == self.start.1 + 1 {
                write!(f, " ")?;
            } else {
                write!(f, "#")?;
            }
        }
        writeln!(f)?;

        // Now we draw the map. It always starts and ends with a '#' for the wall.
        for row in 0..self.dimensions.0 {
            write!(f, "#")?;
            for col in 0..self.dimensions.1 {
                let blizzards: Vec<&Blizzard> = self
                    .blizzards
                    .iter()
                    .filter(|b| (row, col) == b.0)
                    .collect();
                if blizzards.is_empty() {
                    write!(f, " ")?;
                } else if blizzards.len() > 1 {
                    write!(f, "{}", blizzards.len())?;
                } else {
                    write!(
                        f,
                        "{}",
                        match blizzards[0].1 {
                            Direction::Right => '>',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Up => '^',
                        }
                    )?;
                }
            }
            write!(f, "#")?;
            writeln!(f)?;
        }

        // The last row is also a bunch of #'s, with one gap at the end point.
        for col in 0..=self.dimensions.1 + 1 {
            if col == self.end.1 + 1 {
                write!(f, " ")?;
            } else {
                write!(f, "#")?;
            }
        }
        writeln!(f)?;

        Ok(())
    }
}

fn neighbors(state: &State, point: &Point) -> Vec<Point> {
    [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|(dy, dx)| (point.0 + dy, point.1 + dx))
        .filter(|&p| {
            p == state.start
                || p == state.end
                || (0 <= p.0
                    && p.0 < state.dimensions.0
                    && 0 <= p.1
                    && p.1 < state.dimensions.1
                    && !state.obstacles.contains(&p))
        })
        .collect::<Vec<_>>()
}

fn arrival_time(start: &Point, end: &Point, start_time: usize, states: &mut Vec<State>) -> u32 {
    let (_, distance) = astar(
        &(*start, start_time),
        |(p, time)| {
            if states.len() <= 1 + time {
                let last_state = states.last().unwrap();
                let next_state = last_state.next();
                states.push(next_state);
            }

            neighbors(&states[time + 1], p)
                .iter()
                .map(|&neighbor| ((neighbor, time + 1), 1))
                .collect::<Vec<_>>()
        },
        |(p, _)| end.0.abs_diff(p.0) + end.1.abs_diff(p.1),
        |(p, _)| *p == *end,
    )
    .expect("no path found");

    start_time as u32 + distance
}

#[aoc_generator(day24)]
fn generator(input: &str) -> State {
    let num_cols = input.find('\n').unwrap() - 2;
    let start = (-1, input.find('.').unwrap() as i32 - 1);

    let mut obstacles = HashSet::new();
    let mut blizzards = Vec::new();

    for (row, line) in input
        .lines()
        .skip(1)
        .enumerate()
        .take_while(|(_, line)| line.as_bytes()[1] != b'#')
    {
        for (col, c) in line.chars().skip(1).enumerate().take(num_cols) {
            let coords = (row as i32, col as i32);
            if let Some(blizzard) = match c {
                '.' => None,
                '>' => Some((coords, Direction::Right)),
                'v' => Some((coords, Direction::Down)),
                '<' => Some((coords, Direction::Left)),
                '^' => Some((coords, Direction::Up)),
                c => panic!("bad map character {}", c),
            } {
                obstacles.insert(blizzard.0);
                blizzards.push(blizzard);
            }
        }
    }

    let num_rows = input.lines().count() - 2;
    let dimensions = (num_rows as i32, num_cols as i32);

    let last_line = input.lines().last().unwrap();
    let end = (num_rows as i32, last_line.find('.').unwrap() as i32 - 1);

    State {
        obstacles,
        blizzards,
        dimensions,
        start,
        end,
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &State) -> u32 {
    let mut states = Vec::new();
    states.push(input.clone());

    arrival_time(&input.start, &input.end, 0, &mut states)
}

#[aoc(day24, part2)]
pub fn part2(input: &State) -> u32 {
    let mut states = Vec::new();
    states.push(input.clone());

    let get_to_end = arrival_time(&input.start, &input.end, 0, &mut states);
    println!("Got to the end at time {}", get_to_end);

    let back_to_start = arrival_time(&input.end, &input.start, get_to_end as usize, &mut states);
    println!("Back at the start at time {}", back_to_start);

    let back_to_end = arrival_time(
        &input.start,
        &input.end,
        back_to_start as usize,
        &mut states,
    );
    println!("Finished (with snacks) at time {}", back_to_end);

    back_to_end
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{generator, part1, part2};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day24.txt").expect("missing input");
        let world = generator(&input);
        assert_eq!(part1(&world), 18);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day24.txt").expect("missing input");
        let world = generator(&input);
        assert_eq!(part2(&world), 54);
    }
}
