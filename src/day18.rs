use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

type Point = (i32, i32, i32);
type State = HashSet<Point>;

fn parse_line(input: &str) -> IResult<&str, (i32, i32, i32)> {
    tuple((
        terminated(i32, tag(",")),
        terminated(i32, tag(",")),
        terminated(i32, multispace0),
    ))(input)
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

#[aoc_generator(day18)]
fn generator(input: &str) -> State {
    let (_, lavas) = many1(parse_line)(input).expect("parse error");
    lavas.iter().cloned().collect()
}

#[aoc(day18, part1)]
pub fn part1(lava: &State) -> usize {
    // For part 1, we want to count all of the faces of lava blocks that aren't
    // themselves facing another lava block.
    lava.iter()
        .map(|point| {
            neighbors(point)
                .iter()
                .filter(|n| !lava.contains(n))
                .count()
        })
        .sum()
}

/*
 * The next few functions implement a 3D bounding box for part 2. 
 */
fn lower_bounds(lhs: &Point, rhs: &Point) -> Point {
    (min(lhs.0, rhs.0), min(lhs.1, rhs.1), min(lhs.2, rhs.2))
}

fn upper_bounds(lhs: &Point, rhs: &Point) -> Point {
    (max(lhs.0, rhs.0), max(lhs.1, rhs.1), max(lhs.2, rhs.2))
}

fn inside(point: &Point, lower_bound: &Point, upper_bound: &Point) -> bool {
    lower_bound.0 <= point.0
        && point.0 <= upper_bound.0
        && lower_bound.1 <= point.1
        && point.1 <= upper_bound.1
        && lower_bound.2 <= point.2
        && point.2 <= upper_bound.2
}

#[aoc(day18, part2)]
pub fn part2(input: &State) -> usize {
    let lava = input.clone();

    // Find the bounding box for the lava.
    const SMALLEST_POINT: Point = (i32::MIN, i32::MIN, i32::MIN);
    const LARGEST_POINT: Point = (i32::MAX, i32::MAX, i32::MAX);
    let (mut lower_bounds, mut upper_bounds) = lava.iter().fold(
        (LARGEST_POINT, SMALLEST_POINT),
        |bounds: (Point, Point), point| {
            (
                lower_bounds(&bounds.0, point),
                upper_bounds(&bounds.1, point),
            )
        },
    );

    // Extend the bounding box by 1 in each direction to make sure that there
    // is a shell of "exterior" points outside the lava.
    lower_bounds = (lower_bounds.0 - 1, lower_bounds.1 - 1, lower_bounds.2 - 1);
    upper_bounds = (upper_bounds.0 + 1, upper_bounds.1 + 1, upper_bounds.2 + 1);

    // Pick an arbitrary point in the (extended) bounding box that we *know* is air.
    let start_point = (lower_bounds.0, lower_bounds.1, lower_bounds.2);

    // Run BFS starting from that point to identify all points on the "outside" of the lava.
    // Many thanks to zarvox for pointing out this approach!
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut exterior: HashSet<Point> = HashSet::new();
    queue.push_back(start_point);
    exterior.insert(start_point);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for p in neighbors(&current) {
            if !exterior.contains(&p)
                && !lava.contains(&p)
                && inside(&p, &lower_bounds, &upper_bounds)
            {
                exterior.insert(p);
                queue.push_back(p);
            }
        }
    }

    // Now copy/paste from part 1 to find the number of exposed faces.
    // It's actually a little nicer now because we have an explicit list
    // of all exterior points!
    lava.iter()
        .map(|point| {
            neighbors(point)
                .iter()
                .filter(|n| exterior.contains(n))
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
