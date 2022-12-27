use itertools::Itertools;

fn parse(input: &str) -> Vec<u32> {
    input
        // Each elf's stack of cookies is separated by an empty line, so
        // splitting on \n\n gives us chunks of data separated by elf.
        .split("\n\n")
        .map(|group|
            // Take this elf's data, split it into lines, convert each line
            // to an integer, and add them up to get the elf's total calories.
            group.lines().filter_map(|line| line.parse::<u32>().ok()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    parse(input).into_iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    parse(input).into_iter().sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000\n\
                           2000\n\
                           3000\n\
                               \n\
                           4000\n\
                               \n\
                           5000\n\
                           6000\n\
                               \n\
                           7000\n\
                           8000\n\
                           9000\n\
                               \n\
                           10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 45000);
    }
}
