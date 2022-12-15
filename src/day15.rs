use std::{
    cmp::{max, min},
    collections::HashSet,
};

use nom::{
    bytes::complete::tag,
    character::complete::i32,
    character::complete::multispace0,
    combinator::map,
    multi::many1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    start: i32,
    end: i32,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        pair(preceded(tag("x="), i32), preceded(tag(", y="), i32)),
        |(x, y)| Point { x, y },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    tuple((
        preceded(tag("Sensor at "), parse_point),
        delimited(tag(": closest beacon is at "), parse_point, multispace0),
    ))(input)
}

fn get_covered_intervals(points_and_beacons: &[(Point, Point)], target_y: i32) -> Vec<Interval> {
    // For part 1 we want to find the number of points at y=2_000_000 that
    // *cannot* be the location of another beacon.
    //
    // The approach is to consider each point in turn. Since we know which beacon
    // is closest to that point, it has a sort of zone of exclusion where a new
    // beacon cannot be placed. We'll generate a list of (possibly-empty) intervals
    // where those exclusion zones intersect with the line y=2_000_000, then count
    // the total number of points in those intervals.
    let mut intervals: Vec<Interval> = points_and_beacons
        .iter()
        .filter_map(|(point, beacon)| {
            let distance = (beacon.x - point.x).abs() + (beacon.y - point.y).abs();
            let distance_to_line = (target_y - point.y).abs();
            let spread = distance - distance_to_line;
            if spread >= 0 {
                Some(Interval {
                    start: point.x - spread,
                    end: point.x + spread,
                })
            } else {
                None
            }
        })
        .collect();

    // Okay, we have our list of intervals. To avoid double-counting, we'll
    // merge the intervals into non-overlapping ones, then efficiently count up the points.
    // Start by sorting by the start point.
    intervals.sort_by(|lhs, rhs| lhs.start.cmp(&rhs.start));

    // We'll build a new list of intervals! With... eh, whatever.
    let mut merged: Vec<Interval> = Vec::new();

    // The first interval in `intervals` has the earliest start point, so start with that one.
    let mut current = intervals[0];

    // Now go through the list in turn, either extending the current interval or starting a new one.
    for interval in intervals.iter().skip(1) {
        if interval.start <= current.end + 1 {
            current.end = max(current.end, interval.end);
        } else {
            merged.push(current);
            current = *interval;
        }
    }
    merged.push(current);
    merged
}

fn count_covered_points(intervals: &[Interval]) -> i32 {
    // Since we know our intervals are non-overlapping, this is easy.
    intervals.iter().map(|int| int.end - int.start + 1).sum()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i32 {
    let (_, lines) = many1(parse_line)(input).expect("parsing error");
    let intervals = get_covered_intervals(&lines, 2_000_000);
    let covered_points = count_covered_points(&intervals);

    // The problem apparently wants us to avoid counting points that already have beacons.
    let beacons: HashSet<&Point> = lines.iter().map(|(_, beacon)| beacon).collect();
    let beacons_on_line = beacons
        .iter()
        .filter(|beacon| beacon.y == 2_000_000)
        .count();

    covered_points - (beacons_on_line as i32)
}

/********************
 * Stuff for Part 2 *
 ********************/

fn tuning_frequency(point: &Point) -> usize {
    (point.x as usize) * 4_000_000 + (point.y as usize)
}

// Takes a vector of *non-overlapping* intervals.
fn clamp_intervals(intervals: &Vec<Interval>, minimum: i32, maximum: i32) -> Vec<Interval> {
    let mut result: Vec<Interval> = Vec::new();
    let mut i: usize = 0;

    // Skip all of the intervals until we find one that ends on or after the minimum.
    while i < intervals.len() && intervals[i].end < minimum {
        i += 1;
    }

    // Include each until we hit one that starts on or after the max.
    while i < intervals.len() && intervals[i].start < maximum {
        result.push(Interval {
            start: max(intervals[i].start, minimum),
            end: min(intervals[i].end, maximum),
        });
        i += 1;
    }

    result
}

fn find_uncovered_point(points_and_beacons: &[(Point, Point)], max_coord: i32) -> Option<Point> {
    // First attempt: extremely brute force.
    for y in 0..=max_coord {
        let intervals = get_covered_intervals(points_and_beacons, y);

        // Get rid of all points outside of [0, max_coord]
        let clamped = clamp_intervals(&intervals, 0, max_coord);

        // Count the points.
        let points = count_covered_points(&clamped);
        if points != max_coord + 1 {
            // We found the right row!
            // The y-coordinate is trivial (it's y).
            // Go over the list of intervals to find the gap to get x.
            for i in 1..clamped.len() {
                if clamped[i].start == 2 + clamped[i - 1].end {
                    return Some(Point {
                        x: clamped[i].start - 1,
                        y,
                    });
                }
            }
        }
    }

    None
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let (_, lines) = many1(parse_line)(input).expect("parsing error");
    let new_beacon = find_uncovered_point(&lines, 4_000_000).unwrap();
    tuning_frequency(&new_beacon)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use nom::multi::many1;

    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day15.txt").expect("missing input");
        let (_, lines) = many1(parse_line)(&input).expect("parsing error");
        let intervals = get_covered_intervals(&lines, 10);
        let covered_points = count_covered_points(&intervals);

        let beacons: HashSet<&Point> = lines.iter().map(|(_, beacon)| beacon).collect();
        let beacons_on_line = beacons.iter().filter(|beacon| beacon.y == 10).count();

        let answer = covered_points - (beacons_on_line as i32);
        assert_eq!(answer, 26);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day15.txt").expect("missing input");
        let (_, lines) = many1(parse_line)(&input).expect("parsing error");
        let new_beacon = find_uncovered_point(&lines, 20).unwrap();

        assert_eq!(tuning_frequency(&new_beacon), 56000011);
    }
}
