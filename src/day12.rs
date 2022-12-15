
#[aoc(day12, part1)]
pub fn part1(input: &str) -> u32 {
    return input;
}



#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    return input;
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "Sabqponm\n\
                           abcryxxl\n\
                           accszExk\n\
                           acctuvwj\n\
                           abdefghi";

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
