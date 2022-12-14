use itertools::Itertools;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<i32> {
    let mut state: Vec<i32> = Vec::new();
    let mut x = 1;
    for line in input.lines() {
        match &line[..4] {
            "noop" => {
                state.push(x);
            }
            "addx" => {
                let (_, value_str) = line.split_once(' ').unwrap();
                let value = value_str.parse::<i32>().unwrap();

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
        let pixel_x = cycle_minus_1 as i32 % 40;
        if (x - pixel_x).abs() <= 1 {
            '#'
        } else {
            ' '
        }
    });

    let display: String = bits
        .chunks(40)
        .into_iter()
        .map(|chunk| String::from_iter(chunk) + "\n")
        .collect();

    println!("{display}");

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
