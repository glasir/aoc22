use std::{cmp::Ordering, iter::zip};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, terminated},
    IResult,
};

#[derive(PartialEq, Debug)]
enum Data {
    Int(i32),
    List(Vec<Data>),
}

impl Data {
    // Creates a Data containing a list of a single element.
    fn list_of(value: i32) -> Self {
        Data::List(vec![Data::Int(value)])
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Data::Int(lhs), Data::Int(rhs)) => lhs.partial_cmp(rhs),
            (Data::Int(lhs), Data::List(_)) => Data::list_of(*lhs).partial_cmp(other),
            (Data::List(_), Data::Int(rhs)) => self.partial_cmp(&Data::list_of(*rhs)),
            (Data::List(lhs), Data::List(rhs)) => {
                for (l, r) in zip(lhs, rhs) {
                    match l.partial_cmp(r) {
                        Some(Ordering::Less) => return Some(Ordering::Less),
                        Some(Ordering::Greater) => return Some(Ordering::Greater),
                        _ => {}
                    }
                }

                // We got to the end of one of the lists.
                // Compare the lengths of the lists to finish this element.
                lhs.len().partial_cmp(&rhs.len())
            }
        }
    }
}

fn parse_data(data: &str) -> IResult<&str, Data> {
    alt((
        map(i32, Data::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_data), tag("]")),
            Data::List,
        ),
    ))(data)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Data>> {
    many1(terminated(parse_data, multispace0))(input)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let (_, packets) = parse_input(input).expect("parse error");

    let mut result = 0;
    for i in (0..packets.len()).step_by(2) {
        if packets[i] < packets[i + 1] {
            result += 1 + i / 2;
        }
    }

    result
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let (_, packets) = parse_input(input).expect("parse error");

    // We can avoid sorting by just comparing each divider against every packet.
    let divider0 = Data::list_of(2);
    let divider1 = Data::list_of(6);

    let mut less_than_first = 0;
    let mut less_than_second = 0;

    for packet in packets {
        if packet < divider0 {
            less_than_first += 1;
            less_than_second += 1; // optimization!
        } else if packet < divider1 {
            less_than_second += 1;
        }
    }

    // The +1 is because the list of packets is 1-indexed.
    // The +2 is because it's 1-indexed, and we need to count the first divider.
    (less_than_first + 1) * (less_than_second + 2)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day13.txt").expect("missing input");
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day13.txt").expect("missing input");
        assert_eq!(part2(&input), 140);
    }
}
