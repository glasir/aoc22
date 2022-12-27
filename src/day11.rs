use std::{collections::VecDeque, fmt};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0, u64},
    combinator::map,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

/**
 * Each monkey performs some mathematical operation.
 *
 * Each operation has an operator (add or multiply).
 *
 * Each operand has two parameters: the current value,
 * and either "old" (the current value), or an integer.
 *
 * These enums just capture this structure.
 */
enum Operand {
    Old,
    Value(u64),
}

enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        }
    }
}

struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn evaluate(&self, lhs: u64) -> u64 {
        let rhs = match self.operand {
            Operand::Old => lhs,
            Operand::Value(x) => x,
        };

        self.operator.evaluate(lhs, rhs)
    }
}

struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    if_true: u64,
    if_false: u64,
    inspections: u64,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey {}: {:?}", self.id, self.items)
    }
}

/************
 * Parsers! *
 ************/

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("+"), |_| Operator::Add),
        map(tag("*"), |_| Operator::Multiply),
    ))(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((map(tag("old"), |_| Operand::Old), map(u64, Operand::Value)))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        preceded(
            tag("new = old "),
            tuple((parse_operator, preceded(space0, parse_operand))),
        ),
        |(operator, operand)| Operation { operator, operand },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(tag("Monkey "), u64, tag(":\n")),
            delimited(
                pair(space0, tag("Starting items: ")),
                separated_list0(tag(", "), u64),
                newline,
            ),
            delimited(pair(space0, tag("Operation: ")), parse_operation, newline),
            delimited(pair(space0, tag("Test: divisible by ")), u64, newline),
            delimited(pair(space0, tag("If true: throw to monkey ")), u64, newline),
            delimited(
                pair(space0, tag("If false: throw to monkey ")),
                u64,
                many0(newline),
            ),
        )),
        |(id, item_vec, operation, divisor, if_true, if_false)| {
            // nom can only produce a Vec<>; convert to a VecDeque<> for ease of use later.
            let items = VecDeque::from(item_vec);
            let inspections = 0;
            Monkey {
                id,
                items,
                operation,
                divisor,
                if_true,
                if_false,
                inspections,
            }
        },
    )(input)
}

/*************************
 * The actual solutions! *
 *************************/

/**
 * Take a turn. Returns a list of pairs (item, target_monkey_idx).
 *
 * The second parameter here is the "worry reducer". For part 1, it's |x| x / 3.
 * Part 2 asks us to figure something else out.
 */
fn turn(monkey: &mut Monkey, worry_reducer: impl Fn(u64) -> u64) -> Vec<(u64, u64)> {
    monkey
        .items
        .drain(..)
        .map(|item| {
            monkey.inspections += 1;

            // Update the worry level for this item
            let mut worry: u64 = monkey.operation.evaluate(item);

            // Monkey loses interest
            worry = worry_reducer(worry);

            // Figure out which monkey to throw the item to.
            let catcher = if worry % monkey.divisor == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            (worry, catcher)
        })
        .collect()
}

/**
 * Does a whole round of monkey business: each monkey takes a single turn.
 */
fn round(monkeys: &mut Vec<Monkey>, worry_reducer: &impl Fn(u64) -> u64) {
    for idx in 0..monkeys.len() {
        // What items are being thrown, and to whom?
        let moves = turn(&mut monkeys[idx], worry_reducer);

        // Throw the items to each catching monkey in turn.
        for (item, to) in moves {
            monkeys[to as usize].items.push_back(item);
        }
    }
}

/**
 * Finds the two monkeys with the highest number of items inspected,
 * and multiplies their inspection counts.
 */
fn monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    // This is a little clunky, but it's a bit faster than sorting
    // and taking the top two.
    let mut most: u64 = 0;
    let mut next: u64 = 0;

    for monkey in monkeys {
        if monkey.inspections > most {
            next = most;
            most = monkey.inspections;
        } else if monkey.inspections > next {
            next = monkey.inspections;
        }
    }

    most * next
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let (_, mut monkeys) = many1(parse_monkey)(input).expect("parse error!");
    let worry_reducer = |n| n / 3;

    for _ in 0..20 {
        round(&mut monkeys, &worry_reducer);
    }

    monkey_business(&monkeys)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let (_, mut monkeys) = many1(parse_monkey)(input).expect("parse error!");

    // Stupid math trick alert!
    //
    // Each monkey cares about computing an item's worry value modulo some prime.
    // Those remainders don't change if we first take the worry value modulo some multiple
    // of that prime. By picking the LCM of all of the monkeys' primes, we get a modulus
    // that has this property for every monkey simultaneously.
    //
    // We can then make our worry-reducing function `|n| n % modulus`, which guarantees that
    // an item's worry value cannot ever be above our modulus.
    let modulus: u64 = monkeys
        .iter()
        .map(|m| m.divisor)
        .fold(1u64, num::integer::lcm);

    let worry_reducer = |n| n % modulus;

    // There might be a cycle-finding trick in here to reduce runtime, but just simulating
    // finishes pretty quickly.
    for _ in 0..10_000 {
        round(&mut monkeys, &worry_reducer);
    }

    monkey_business(&monkeys)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day11.txt").expect("missing input");
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day11.txt").expect("missing input");
        assert_eq!(part2(&input), 2713310158);
    }
}
