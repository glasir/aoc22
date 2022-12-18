use std::{
    cmp::{max, min},
    collections::HashSet,
};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use pathfinding::prelude::astar;

type Point = (i32, i32, i32);
type State = HashSet<Point>;

fn parse_line(input: &str) -> IResult<&str, (i32, i32, i32)> {
    tuple((
        terminated(i32, tag(",")),
        terminated(i32, tag(",")),
        terminated(i32, multispace0),
    ))(input)
}

#[aoc_generator(day18)]
fn generator(input: &str) -> State {
    let (_, lavas) = many1(parse_line)(input).expect("parse error");
    lavas.iter().cloned().collect()
}

fn neighbors(point: &Point) -> Vec<Point> {
    vec![
        (point.0 - 1, point.1, point.2),
        (point.0 + 1, point.1, point.2),
        (point.0, point.1 - 1, point.2),
        (point.0, point.1 + 1, point.2),
        (point.0, point.1, point.2 - 1),
        (point.0, point.1, point.2 + 1),
    ]
}

#[aoc(day18, part1)]
pub fn part1(state: &State) -> usize {
    state
        .iter()
        .map(|point| {
            neighbors(point)
                .iter()
                .filter(|n| !state.contains(n))
                .count()
        })
        .sum()
}

fn lower_bounds(lhs: &Point, rhs: &Point) -> Point {
    (min(lhs.0, rhs.0), min(lhs.1, rhs.1), min(lhs.2, rhs.2))
}

fn upper_bounds(lhs: &Point, rhs: &Point) -> Point {
    (max(lhs.0, rhs.0), max(lhs.1, rhs.1), max(lhs.2, rhs.2))
}

fn open_neighbors(state: &State, point: &Point) -> Vec<(Point, i32)> {
    neighbors(point)
        .iter()
        .filter(|p| !state.contains(p))
        .map(|p| (*p, 1))
        .collect()
}

#[aoc(day18, part2)]
pub fn part2(input: &State) -> usize {
    let mut state = input.clone();

    // Find the bounding box for the lava.
    const SMALLEST_POINT: Point = (i32::MIN, i32::MIN, i32::MIN);
    const LARGEST_POINT: Point = (i32::MAX, i32::MAX, i32::MAX);
    let (lower_bounds, upper_bounds) = state.iter().fold(
        (LARGEST_POINT, SMALLEST_POINT),
        |bounds: (Point, Point), point| {
            (
                lower_bounds(&bounds.0, point),
                upper_bounds(&bounds.1, point),
            )
        },
    );

    // Pick an arbitrary point outside the bounding box
    let target_point = (lower_bounds.0 - 1, lower_bounds.1, lower_bounds.2);

    // For every point in the bounding box, see if there's an uninterrupted path to it from the target point.
    // If there isn't, that point is part of a "bubble" in the lava; fill it in with lava.

    // Note: running 20^3 iterations of A* is obviously not very efficient.
    // I was trying to figure out how to flood-fill the "inside" of the lava blob, but
    // got stuck on how to find initial points on the inside.
    // Then I realized I could use a pathfinding algorithm: if I start from any point,
    // and can find a path to a point I *know* is outside the lava, that point must also
    // be outside.
    // Then we can mark every point we *couldn't* reach as "inside" and re-run part 1.
    // I don't love this solution, but it runs acceptably fast (300ms) and works...
    for x in lower_bounds.0..=upper_bounds.0 {
        for y in lower_bounds.1..=upper_bounds.1 {
            for z in lower_bounds.2..=upper_bounds.2 {
                if astar(
                    &(x, y, z),
                    |p| open_neighbors(&state, p),
                    |p| {
                        (target_point.0 - p.0).abs()
                            + (target_point.1 - p.1).abs()
                            + (target_point.2 - p.2).abs()
                    },
                    |p| *p == target_point,
                )
                .is_none()
                {
                    state.insert((x, y, z));
                }
            }
        }
    }

    // Now re-run part 1 to find the adjusted number of faces.
    state
        .iter()
        .map(|point| {
            neighbors(point)
                .iter()
                .filter(|n| !state.contains(n))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "2,2,2\n\
                           1,2,2\n\
                           3,2,2\n\
                           2,1,2\n\
                           2,3,2\n\
                           2,2,1\n\
                           2,2,3\n\
                           2,2,4\n\
                           2,2,6\n\
                           1,2,5\n\
                           3,2,5\n\
                           2,1,5\n\
                           2,3,5";

    #[test]
    fn test_part1() {
        let state = generator(EXAMPLE);
        assert_eq!(part1(&state), 64);
    }

    #[test]
    fn test_part2() {
        let state = generator(EXAMPLE);
        assert_eq!(part2(&state), 58);
    }
}
