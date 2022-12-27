use std::{collections::VecDeque, fmt::Display};
use text_io::scan;

/**
 * Holds the state of the stacks of crates.
 *
 * `stacks[i]` contains the crates in the i-th stack, with
 * the bottom crate in stacks[i][0].
 *
 * The problem statement uses 1-indexing for the stacks; we
 * convert this to 0-indexing at parse time. So "Stack 1"
 * in the problem statement is stacks[0] here.
 */
#[derive(Clone)]
pub struct State {
    stacks: Vec<VecDeque<char>>,
}

/**
 * A single step in the crate-rearrangement procedure.
 *
 * The stack indices are 0-based here as well.
 */
pub struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl State {
    /**
     * Applies a single step ("move N from A to B") to the state.
     *
     * The third parameter specifies whether to reverse the order of the crates
     * before adding them to their new stack. This lets us use one function to
     * handle both parts of the problem: "move 3 crates, 1 at a time" is
     * equivalent to "get three crates, reverse their order, and append them".
     */
    fn apply(&mut self, step: &Step, reverse: bool) {
        let mut crates = self.remove_crates(step.from, step.count);

        if reverse {
            crates.reverse();
        }

        self.add_crates(step.to, crates);
    }

    /**
     * Adds a list of crates to the top of a stack.
     */
    fn add_crates(&mut self, stack: usize, crates: Vec<char>) {
        self.stacks[stack].extend(crates.iter())
    }

    /**
     * Removes crates from a stack, returning the removed crates in a list.
     */
    fn remove_crates(&mut self, stack: usize, count: usize) -> Vec<char> {
        let initial_len = self.stacks[stack].len();
        self.stacks[stack].drain(initial_len - count..).collect()
    }

    /**
     * Returns a string containing the letters of the crates at the top
     * of each stack in order.
     */
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
            // Add 1 to switch back to 1-indexing
            write!(f, "{}:", i + 1)?;
            for krate in &self.stacks[i] {
                write!(f, " {}", krate)?;
            }
        }
        Ok(())
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Remember to switch back to 1-indexing.
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            self.from + 1,
            self.to + 1
        )
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
            let (count, from, to): (usize, usize, usize);
            scan!(line.bytes() => "move {} from {} to {}", count, from, to);

            // Create a new Step object. Subtract 1 from the stack indicies
            // to correct for AoC's 1-indexing.
            Step {
                count,
                from: from - 1,
                to: to - 1,
            }
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
