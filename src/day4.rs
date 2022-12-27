use std::cmp::{max, min};

/**
 * Represents a closed interval over the (nonnegative) integers.
 * For example, Range { start: 2, end: 4 } represents [2, 3, 4].
 */
#[derive(Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn from_str(s: &str) -> Range {
        s.split_once('-')
            .map(|(start, end)| Range {
                start: start.parse::<usize>().unwrap(),
                end: end.parse::<usize>().unwrap(),
            })
            .unwrap()
    }

    /**
     * Checks whether this range entirely contains another.
     */
    fn contains(&self, other: &Range) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }

    /**
     * Checks whether this range has any overlap with another by
     * checking if the intersection is nonempty.
     */
    fn overlaps(&self, other: &Range) -> bool {
        // This is maybe the first time that leetcode has been
        // useful in the real world (to the extent that AoC counts).
        max(self.start, other.start) <= min(self.end, other.end)
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let elves: Vec<Range> = line.split(',').map(Range::from_str).collect();
            (elves[0], elves[1])
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| elf1.contains(elf2) || elf2.contains(elf1))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| elf1.overlaps(elf2))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "2-4,6-8\n\
                           2-3,4-5\n\
                           5-7,7-9\n\
                           2-8,3-7\n\
                           6-6,4-6\n\
                           2-6,4-8";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 4);
    }
}
