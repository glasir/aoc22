/**
 * Performs one iteration of the "mix" operation in-place.
 * Operates on a list of (original index, value) tuples; this pair
 * structure makes it easy to process elements in their original order,
 * even if mixing multiple times.
 */
fn mix(indexed_numbers: &mut Vec<(usize, i64)>) {
    for original_idx in 0..indexed_numbers.len() {
        // Find the *current* index of the value *originally* at original_idx.
        let current_idx = indexed_numbers
            .iter()
            .position(|&(idx, _)| original_idx == idx)
            .unwrap();

        // Remove that element from the list.
        let (orig_idx, value) = indexed_numbers[current_idx];
        indexed_numbers.remove(current_idx);

        // Find the new index that element should be moved to.
        // rem_euclid is basically like % but always returns a nonnegative value.
        let new_idx = (current_idx as i64 + value).rem_euclid(indexed_numbers.len() as i64);

        // Insert the element into its new location.
        indexed_numbers.insert(new_idx as usize, (orig_idx, value));
    }
}

/**
 * Returns the "grove positioning coordinates" for a given decrypted message.
 */
fn coordinates(indexed_numbers: &[(usize, i64)]) -> i64 {
    // Find the index of value 0 in the list provided.
    let zero_idx = indexed_numbers
        .iter()
        .position(|&(_, val)| val == 0)
        .unwrap();

    // Find the values 1000, 2000, and 3000 out from that and add them.
    indexed_numbers[(zero_idx + 1000) % indexed_numbers.len()].1
        + indexed_numbers[(zero_idx + 2000) % indexed_numbers.len()].1
        + indexed_numbers[(zero_idx + 3000) % indexed_numbers.len()].1
}

/**
 * Given a string containing one number per line, returns a list of
 * (index, number) pairs, where `number` originally appeared on the
 * `index`-th line.
 */
fn parse_numbers(input: &str) -> Vec<(usize, i64)> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.parse().unwrap()))
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    let mut indexed_numbers = parse_numbers(input);
    mix(&mut indexed_numbers);
    coordinates(&indexed_numbers)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    // This time we have to multiply each number by the "decryption key".
    let mut indexed_numbers = parse_numbers(input)
        .iter()
        .map(|&(idx, n)| (idx, n * 811589153))
        .collect();

    // We also have to mix 10 times.
    for _ in 0..10 {
        mix(&mut indexed_numbers);
    }

    coordinates(&indexed_numbers)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "1\n2\n-3\n3\n-2\n0\n4\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 1623178306);
    }
}
