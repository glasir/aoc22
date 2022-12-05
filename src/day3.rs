use std::collections::HashSet;

use itertools::Itertools;

fn priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as u32,
        b'A'..=b'Z' => (item - b'A' + 27) as u32,
        _ => panic!("invalid char"),
    }
}

fn as_set(chars: &str) -> HashSet<u8> {
    HashSet::from_iter(chars.bytes())
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let compartment_size = line.len() / 2;
            let compartment1 = &line[0..compartment_size];
            let compartment2 = &line[compartment_size..];

            let set1 = as_set(compartment1);
            let set2 = as_set(compartment2);

            let common_char = set1.intersection(&set2).next().unwrap();

            priority(*common_char)
        })
        .sum();
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    return input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            priority(
                chunk
                    .map(|elf| HashSet::from_iter(elf.bytes()) as HashSet<u8>)
                    .reduce(|acc, elf| &acc & &elf)
                    .expect("wut")
                    .drain()
                    .next()
                    .unwrap(),
            )
        })
        .sum();
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
