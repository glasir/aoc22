fn priority(item: u8) -> usize {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as usize,
        b'A'..=b'Z' => (item - b'A' + 27) as usize,
        _ => panic!("invalid char"),
    }
}

/**
 * A very simple bitset specialized for day 3.
 * 
 * The i-th bit is 1 if a character with priority i has been
 * added to the set, and is 0 otherwise.
 */
struct CharSet {
    counts: u64
}

impl CharSet {
    fn new() -> Self {
        Self { counts: 0 }
    }

    fn from(string: &str) -> Self {
        let mut charset = Self::new();
        charset.add(string);
        charset
    }

    fn add(&mut self, string: &str) {
        for b in string.bytes() {
            self.counts |= 1 << priority(b);
        }
    }

    /**
     * Returns the priority of the (assumed-unique) character
     * in the intersection of two CharSets.
     */
    fn intersect(&self, other: &Self) -> usize {
        let mut mask = 1;
        for idx in 0..53 {
            if (self.counts & mask > 0) && (other.counts & mask > 0) {
                return idx;
            }
            mask <<= 1;
        }
        0
    }

    /**
     * Returns the priority of the (assumed-unique) character
     * in the intersection of *three* CharSets.
     */
    fn intersect3(&self, other: &Self, third: &Self) -> usize {
        let mut mask = 1;
        for idx in 0..53 {
            if (self.counts & mask > 0) && (other.counts & mask > 0) && (third.counts & mask > 0) {
                return idx;
            }
            mask <<= 1;
        }
        0
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            let compartment_size = line.len() / 2;
            let compartment1 = &line[0..compartment_size];
            let compartment2 = &line[compartment_size..];

            let set1 = CharSet::from(compartment1);
            let set2 = CharSet::from(compartment2);

            set1.intersect(&set2)
        })
        .sum();
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut charsets = [CharSet::new(), CharSet::new(), CharSet::new()];
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        charsets[i % 3] = CharSet::from(line);

        if i % 3 == 2 {
            total += charsets[0].intersect3(&charsets[1], &charsets[2]);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                           jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                           PmmdzqPrVvPwwTWBwg\n\
                           wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                           ttgJtRGJQctTZtZT\n\
                           CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 70);
    }
}
