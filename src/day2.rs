/*
 * For reference:
 *  ROCK = 0
 *  PAPER = 1
 *  SCISSORS = 2
 * 
 * In the problem description, these are 1-indexed instead
 * of 0-indexed, which will affect our scoring function later.
 */

// Returns the play that beats `other`
fn beats(other: u32) -> u32 {
    (other + 1) % 3
}

// Returns the play that loses to `other`
fn loses_to(other: u32) -> u32 {
    (other + 2) % 3
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let opp_choice = u32::from(bytes[0] - b'A');
            let my_choice = u32::from(bytes[2] - b'X');

            if my_choice == opp_choice {
                // A tie is three points, plus the value of my choice.
                // Add an additional 1 to account for 0- vs 1-indexing.
                4 + my_choice
            } else if my_choice == beats(opp_choice) {
                7 + my_choice
            } else {
                1 + my_choice
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let opp_choice = u32::from(bytes[0] - b'A');

            match bytes[2] {
                b'X' => 1 + loses_to(opp_choice),
                b'Y' => 4 + opp_choice,
                b'Z' => 7 + beats(opp_choice),
                _ => panic!("unexpected outcome"),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y\n\
                           B X\n\
                           C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 12);
    }
}
