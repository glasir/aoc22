use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn of(&self, point: Point) -> Point {
        match self {
            Direction::North => (point.0 - 1, point.1),
            Direction::South => (point.0 + 1, point.1),
            Direction::West => (point.0, point.1 - 1),
            Direction::East => (point.0, point.1 + 1),
        }
    }
}

type Point = (i32, i32);
type Elves = HashSet<Point>;

#[aoc_generator(day23)]
fn generator(input: &str) -> Elves {
    let mut elves = Elves::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((row as i32, col as i32));
            }
        }
    }

    elves
}

fn print_map(elves: &Elves) {
    let (lower_bounds, upper_bounds) = bounding_box(elves);
    for row in lower_bounds.0..=upper_bounds.0 {
        for col in lower_bounds.1..=upper_bounds.1 {
            if elves.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn has_neighbors(point: Point, elves: &Elves) -> bool {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(dx, dy)| elves.contains(&(point.0 + dx, point.1 + dy)))
        .count()
        > 1
}

fn empty_in_direction(point: Point, direction: &Direction, elves: &Elves) -> bool {
    let deltas_to_check = match direction {
        Direction::North => [(-1, -1), (-1, 0), (-1, 1)],
        Direction::South => [(1, -1), (1, 0), (1, 1)],
        Direction::West => [(-1, -1), (0, -1), (1, -1)],
        Direction::East => [(-1, 1), (0, 1), (1, 1)],
    };

    let num_occupied_spots = deltas_to_check
        .iter()
        .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
        .filter(|p| elves.contains(p))
        .count();

    num_occupied_spots == 0
}

fn proposed_move(point: Point, initial_direction: &Direction, elves: &Elves) -> Option<(i32, i32)> {
    if !has_neighbors(point, elves) {
        return None;
    }

    let mut direction = initial_direction.clone();
    for _ in 0..4 {
        if empty_in_direction(point, &direction, elves) {
            return Some(direction.of(point));
        }

        direction = direction.next();
    }

    None
}

fn do_round(elves: &mut Elves, direction: &mut Direction) -> bool {
    let mut any_moved = false;

    let proposed_moves: HashMap<Point, Point> = elves
        .iter()
        .filter_map(|&p| proposed_move(p, direction, elves).map(|new_p| (p, new_p)))
        .collect();

    let mut destinations: HashMap<&Point, usize> = HashMap::new();
    for dest in proposed_moves.values() {
        let new_count = match destinations.get(dest) {
            Some(count) => 1 + *count,
            None => 1,
        };

        destinations.insert(dest, new_count);
    }

    for (elf, dest) in proposed_moves.iter() {
        if destinations[dest] == 1 {
            elves.remove(elf);
            elves.insert(*dest);
            any_moved = true;
        }
    }

    *direction = direction.next();

    any_moved
}

fn lower_bounds(lhs: &Point, rhs: &Point) -> Point {
    (min(lhs.0, rhs.0), min(lhs.1, rhs.1))
}

fn upper_bounds(lhs: &Point, rhs: &Point) -> Point {
    (max(lhs.0, rhs.0), max(lhs.1, rhs.1))
}

fn bounding_box(elves: &Elves) -> (Point, Point) {
    const SMALLEST_POINT: Point = (i32::MIN, i32::MIN);
    const LARGEST_POINT: Point = (i32::MAX, i32::MAX);

    elves.iter().fold(
        (LARGEST_POINT, SMALLEST_POINT),
        |bounds: (Point, Point), point| {
            (
                lower_bounds(&bounds.0, point),
                upper_bounds(&bounds.1, point),
            )
        },
    )
}

#[aoc(day23, part1)]
pub fn part1(input: &Elves) -> i32 {
    let mut elves = input.clone();
    let mut direction = Direction::North;
    println!("Before:");
    print_map(&elves);

    for _ in 0..10 {
        do_round(&mut elves, &mut direction);
    }

    println!("After:");
    print_map(&elves);

    let (lower_bounds, upper_bounds) = bounding_box(&elves);

    // The answer is the size of the bounding box, minus the number of elf-occupied places.
    (upper_bounds.0 - lower_bounds.0 + 1) * (upper_bounds.1 - lower_bounds.1 + 1)
        - (elves.len() as i32)
}

#[aoc(day23, part2)]
pub fn part2(input: &Elves) -> u32 {
    let mut elves = input.clone();
    let mut direction = Direction::North;
    println!("Before:");
    print_map(&elves);

    let mut rounds = 1;
    while do_round(&mut elves, &mut direction) {
        rounds += 1;
    }

    println!("After:");
    print_map(&elves);

    rounds
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "....#..\n\
                           ..###.#\n\
                           #...#.#\n\
                           .#...##\n\
                           #.###..\n\
                           ##.#.##\n\
                           .#..#..\n";
    // const EXAMPLE: &str = ".....\n\
    //                        ..##.\n\
    //                        ..#..\n\
    //                        .....\n\
    //                        ..##.\n\
    //                        .....\n";
    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 20);
    }
}
