use itertools::Itertools;

/**
 * Unusually, basically all of the work happens in the parse step.
 *
 * This function returns a vector containing the value of `x` at each
 * time step.
 */
#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<i32> {
    let mut state: Vec<i32> = Vec::new();
    let mut x = 1;
    for line in input.lines() {
        match &line[..4] {
            "noop" => {
                // A no-op means that the next time step has the current
                // value of x, and no change is needed.
                state.push(x);
            }
            "addx" => {
                // Get the value to be added to x.
                let (_, value_str) = line.split_once(' ').unwrap();
                let value = value_str.parse::<i32>().unwrap();

                // An addx takes two cycles. For those cycles, x keeps its current
                // value; afterwards, the addx completes and we update x.
                state.push(x);
                state.push(x);
                x += value;
            }
            _ => unreachable!(),
        }
    }

    state
}

#[aoc(day10, part1)]
pub fn part1(input: &[i32]) -> i32 {
    // For part 1, we want the sum of `cycle * x` at cycles 20, 60, 100, ...
    // We have to switch to AoC's 1-indexed cycles, but otherwise this is easy.
    input
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(cycle_minus_1, x)| (1 + cycle_minus_1 as i32) * x)
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let bits = input.iter().enumerate().map(|(cycle_minus_1, x)| {
        // The x-coordinate of the pixel being painted during this cycle.
        let pixel_x = cycle_minus_1 as i32 % 40;
        if (x - pixel_x).abs() <= 1 {
            // If x is within 1 pixel of the current one, paint a #
            '#'
        } else {
            // Otherwise, don't.
            ' '
        }
    });

    // Take the array of pixels and chop it up into 40-wide rows.
    // Then turn each row into a string.
    let display: String = bits
        .chunks(40)
        .into_iter()
        .map(|chunk| String::from_iter(chunk) + "\n")
        .collect();

    // Print the whole thing.
    println!("{display}");

    // I didn't bother trying to OCR the actual answer - just read it from the screen.
    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{generator, part1};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day10.txt").expect("missing input");
        let instructions = generator(&input);
        assert_eq!(part1(&instructions), 13140);
    }
}
