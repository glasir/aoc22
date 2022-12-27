use std::collections::HashMap;

use pathfinding::directed::bfs::bfs;

#[derive(Clone, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            c => panic!("invalid operation: {}", c),
        }
    }

    fn resolve(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
            Self::Multiply => lhs * rhs,
            Self::Divide => lhs / rhs,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Monkey {
    Number(i64),
    Computation(String, String, Operation),
}

#[aoc_generator(day21)]
fn generator(input: &str) -> HashMap<String, Monkey> {
    input
        .lines()
        .map(|line| {
            let (name, computation) = line.split_once(": ").unwrap();
            let monkey: Monkey;
            if let Ok(value) = computation.parse::<i64>() {
                monkey = Monkey::Number(value);
            } else {
                let parts: Vec<&str> = computation.split(' ').collect();
                monkey = Monkey::Computation(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    Operation::from(parts[1]),
                );
            }
            (name.to_string(), monkey)
        })
        .collect()
}

fn evaluate(root: String, monkeys: &mut HashMap<String, Monkey>) -> i64 {
    // Strategy: DFS from "root" node.
    // As we resolve computation nodes, replace them with value nodes.
    let mut stack = Vec::new();
    stack.push(root.clone());

    while !stack.is_empty() {
        let name = stack.pop().unwrap();
        let monkey = &monkeys[&name];

        // If we already have a value for this monkey, nothing further is needed.
        // Otherwise we need to either compute its value, or determine which monkeys we need.
        let Monkey::Computation(lhs, rhs, operation) = monkey else { continue };

        // Check if both "parent" monkeys are value-typed.
        if let (Monkey::Number(lhs_val), Monkey::Number(rhs_val)) = (&monkeys[lhs], &monkeys[rhs]) {
            let value = operation.resolve(*lhs_val, *rhs_val);
            monkeys.insert(name, Monkey::Number(value));
            continue;
        }

        // At least one parent was computation-typed, so
        // we can't get a value for the current node yet.
        stack.push(name);

        // Add the computation-typed parent(s) to the stack of nodes to process.
        // Because it's a stack these will be processed before we try the
        // current node again.
        if let Monkey::Computation(_, _, _) = &monkeys[lhs] {
            stack.push(lhs.to_owned());
        }

        if let Monkey::Computation(_, _, _) = &monkeys[rhs] {
            stack.push(rhs.to_owned());
        }
    }

    if let Monkey::Number(num) = monkeys[&root] {
        num
    } else {
        panic!("could not derive value for root node")
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &HashMap<String, Monkey>) -> i64 {
    evaluate("root".to_string(), &mut input.clone())
}

#[aoc(day21, part2)]
pub fn part2(input: &HashMap<String, Monkey>) -> i64 {
    let mut monkeys = input.clone();

    // Find a path from "root" to "humn".
    // This is just a list of the monkeys' names.
    let path = bfs(
        &"root".to_string(),
        |name| match &monkeys[name] {
            Monkey::Number(_) => vec![],
            Monkey::Computation(lhs, rhs, _) => vec![lhs.to_owned(), rhs.to_owned()],
        },
        |name| name == "humn",
    )
    .unwrap();

    // The next step will be to walk that path, inverting each operation as we go.
    // We know the "target" value of the current node; by computing the value of 
    // the subtree not including "humn", we can figure out the target value for
    // the subtree that *does* include "humn", then repeat.

    // Replace the operation of the "root" monkey with a subtraction.
    // This lets us use the same logic here throughout the path-inverting loop.
    // Since A == B  <==>  A - B == 0, our initial target value will be 0.
    let root_name = "root".to_string();
    if let Monkey::Computation(lhs, rhs, _) = &monkeys[&root_name] {
        monkeys.insert(root_name, Monkey::Computation(lhs.to_owned(), rhs.to_owned(), Operation::Subtract));
    } else {
        panic!("root node cannot be a value");
    }

    let mut target = 0;

    // Now we can walk over the path.
    for i in 0..path.len() - 1 {
        let Monkey::Computation(lhs, rhs, operation) = monkeys[&path[i]].to_owned()
        else { panic!("unexpected value at {}: {:?}", path[i], &monkeys[&path[i]]) };

        // Since division and subtraction are not commutative, we need to handle
        // the case where "humn" is in the left subtree differently from when it
        // is in the right subtree.
        if lhs == path[i + 1] {
            let rhs_value = evaluate(rhs.to_owned(), &mut monkeys);

            target = match operation {
                Operation::Add => target - rhs_value, // target = path[i+1] + rhs
                Operation::Subtract => target + rhs_value, // target = path[i+1] - rhs
                Operation::Multiply => target / rhs_value, // target = path[i+1] * rhs
                Operation::Divide => target * rhs_value, // target = path[i+1] / rhs
            };
        } else {
            let lhs_value = evaluate(lhs.to_owned(), &mut monkeys);

            target = match operation {
                Operation::Add => target - lhs_value, // target = lhs + path[i+1]
                Operation::Subtract => lhs_value - target, // target = lhs - path[i+1]
                Operation::Multiply => target / lhs_value, // target = lhs * path[i+1]
                Operation::Divide => lhs_value / target, // target = lhs / path[i+1]
            };
        }
    }

    // Once we get to "humn", we know what value to shout.
    target
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "root: pppw + sjmn\n\
                           dbpl: 5\n\
                           cczh: sllz + lgvd\n\
                           zczc: 2\n\
                           ptdq: humn - dvpt\n\
                           dvpt: 3\n\
                           lfqf: 4\n\
                           humn: 5\n\
                           ljgn: 2\n\
                           sjmn: drzm * dbpl\n\
                           sllz: 4\n\
                           pppw: cczh / lfqf\n\
                           lgvd: ljgn * ptdq\n\
                           drzm: hmdt - zczc\n\
                           hmdt: 32";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 301);
    }
}
