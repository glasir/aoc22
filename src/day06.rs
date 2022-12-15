// The obvious way to approach this problem is via hashsets.
//
// The simplest (and least efficient) is to create a hashset for each
// slice of `len` bytes and see whether its len() == len.
//
// Or, better, you can use a hashmap from byte -> count_in_window, and
// add/remove bytes as your sliding window moves.
//
// The following implements the latter, but without a real hashmap.
// Instead our "hash" is just h(b) = b - b'a', and we store the number
// of unique items in a separate variable for efficiency.
//
// It assumes that only lowercase alphabetical characters will be added,
// and that a character will be added at most 255 times.
struct CountingCharSet {
    counts: [u8; 26],
    unique: usize,
}

impl CountingCharSet {
    fn new() -> Self {
        CountingCharSet {
            counts: [0u8; 26],
            unique: 0,
        }
    }

    fn add(&mut self, char: u8) {
        let idx = usize::from(char - b'a');
        if self.counts[idx] == 0 {
            self.unique += 1;
        }
        self.counts[idx] += 1;
    }

    fn remove(&mut self, char: u8) {
        let idx = usize::from(char - b'a');
        self.counts[idx] -= 1;
        if self.counts[idx] == 0 {
            self.unique -= 1;
        }
    }
}

fn find_marker(len: usize, data: &[u8]) -> usize {
    let mut set = CountingCharSet::new();

    // Start by inserting the first `len` items.
    for char in data.iter().take(len) {
        set.add(*char);
    }

    // Loop until the charset contains `len` unique items.
    let mut i: usize = len;
    while set.unique < len {
        set.remove(data[i - len]);
        set.add(data[i]);
        i += 1;
    }

    i
}

#[aoc(day6, part1, Bytes)]
pub fn part1(input: &[u8]) -> usize {
    find_marker(4, input)
}

#[aoc(day6, part2, Bytes)]
pub fn part2(input: &[u8]) -> usize {
    find_marker(14, input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1(b"nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2(b"nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
