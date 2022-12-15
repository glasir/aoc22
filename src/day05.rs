use std::{collections::VecDeque, fmt::Display};
use text_io::scan;

#[derive(Clone)]
pub struct State {
    stacks: Vec<VecDeque<char>>,
}

pub struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl State {
    fn apply(&mut self, step: &Step, reverse: bool) {
        let mut crates = self.remove_crates(step.from, step.count);

        if reverse {
            crates.reverse();
        }

        self.add_crates(step.to, crates);
    }

    fn add_crates(&mut self, stack: usize, crates: Vec<char>) {
        self.stacks[stack - 1].extend(crates.iter())
    }

    fn remove_crates(&mut self, stack: usize, count: usize) -> Vec<char> {
        let initial_len = self.stacks[stack - 1].len();
        self.stacks[stack - 1]
            .drain(initial_len - count..)
            .collect()
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.back().unwrap())
            .collect::<String>()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in 0..self.stacks.len() {
            write!(f, "{}:", i)?;
            for krate in &self.stacks[i] {
                write!(f, " {}", krate)?;
            }
        }
        Ok(())
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "move {} from {} to {}", self.count, self.from, self.to)
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (State, Vec<Step>) {
    let mut lines = input.lines();

    // The first section of the input contains the initial state.
    let mut state = State { stacks: Vec::new() };

    // Once we get to a line containing a number, we're done.
    for line in lines.by_ref().take_while(|line| !line.contains('1')) {
        // Find all of the letters in this row, and their indices.
        for (idx, letter) in line.match_indices(|c| ('A'..='Z').contains(&c)) {
            // For each letter, convert its index in the line into a column in the state.
            let stack = (idx - 1) / 4;

            // Add it to that stack, creating the stack (and all preceding ones) if necessary.
            while state.stacks.len() <= stack {
                state.stacks.push(VecDeque::new());
            }

            // it is unbelievable that this could be the simplest way to get the first char of a str.
            state.stacks[stack].push_front(letter.chars().next().unwrap());
        }
    }

    // The rest of the lines include the steps to follow.
    let steps: Vec<Step> = lines
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            let (count, from, to);
            scan!(line.bytes() => "move {} from {} to {}", count, from, to);
            Step { count, from, to }
        })
        .collect();

    (state, steps)
}

#[aoc(day5, part1)]
pub fn part1((input_state, steps): &(State, Vec<Step>)) -> String {
    let mut state = input_state.clone();

    for step in steps {
        state.apply(step, true);
    }

    state.top_crates()
}

#[aoc(day5, part2)]
pub fn part2((input_state, steps): &(State, Vec<Step>)) -> String {
    let mut state = input_state.clone();

    for step in steps {
        state.apply(step, false);
    }

    state.top_crates()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "    [D]    \n\
                           [N] [C]    \n\
                           [Z] [M] [P]\n\
                            1   2   3 \n\
                           \n\
                           move 1 from 2 to 1\n\
                           move 3 from 1 to 3\n\
                           move 2 from 2 to 1\n\
                           move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), String::from("MCD"));
    }
}
