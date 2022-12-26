use take_until::TakeUntilExt;

/**
 * I'm going to make an iNTeResTInG choice and represent the 2-d grid of
 * tree heights with a 1-d array. This has a lot of disadvantages, but
 * allows one neat trick: it makes it trivial to create zero-copy iterators
 * over the columns of the grid.
 *
 * So, for a major sacrifice in readability (see the impls below), we get to
 * handle every computation in an iterator for maximum ~~functionality~~!
 */
pub struct TreeGrid {
    height: usize,
    width: usize,
    values: Vec<u32>,
}

impl TreeGrid {
    // Convenience function for translating from 2-d coordinates to our flat array.
    fn at(&self, row: usize, col: usize) -> u32 {
        self.values[self.width * row + col]
    }

    /*
     * The following somewhat-incomprehensible functions define iterators over the
     * grid elements you'd encounter by starting at (row, col) and walking in one direction.
     *
     * For example, say your grid looks like this:
     *      30373
     *      25512
     *      65332
     *      33549
     *      35390
     *
     * If you start at, say, the 4 in the second-to-last row (i.e., at row 3, column 3), then
     * the items you'll see in each direction are, in order:
     *   * above: [3, 1, 7]
     *   * below: [9]
     *   * left:  [5, 3, 3]
     *   * right: [9]
     *
     * Note that the 'left' and 'above' lists might be reversed from what you'd expect
     * when looking at the grid!
     */

    /*
     * For above() and below(), it's helpful to re-label the grid with the index
     * of each point in the flat array used for storage:
     *       0  1  2  3  4
     *       5  6  7  8  9
     *      10 11 12 13 14
     *      15 16 17 18 19
     *      20 21 22 23 24
     *
     * Say we're starting at row = 3, col = 3, which is index 3 * 5 + 3 = 18.
     *
     * To get the items above it, we:
     *   1. Take all of the items up to and including the starting point:
     *         [0, 1, 2, ..., 16, 17, 18]
     *   2. Reverse the list (since we'll be walking "up" the grid):
     *         [18, 17, 16, ..., 2, 1, 0]
     *   3. Take every (self.width)-th element. This is equivalent to moving up 1 row:
     *         [18, 13, 8, 3]
     *   4. Drop the first element, which is the starting point:
     *         [13, 8, 3]
     *
     * Getting the items below is basically the same, except we grab the items *starting*
     * at the starting point, and don't need to reverse.
     */
    fn above(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> + '_ {
        let start_idx = self.width * row + col;
        self.values
            .iter()
            .take(start_idx + 1)
            .rev()
            .step_by(self.width)
            .skip(1)
    }

    fn below(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> + '_ {
        let start_idx = self.width * row + col;
        self.values
            .iter()
            .skip(start_idx)
            .step_by(self.width)
            .skip(1)
    }

    /**
     * left() and right() are much simpler as they operate on a single row.
     *
     * For left(), we just skip to the start of the relevant row, grab the elements before
     * the starting point, and reverse the result.
     *
     * For right(), we skip until just after the starting point and grab the rest of the row.
     */
    fn left(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> + '_ {
        self.values.iter().skip(self.width * row).take(col).rev()
    }

    fn right(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> + '_ {
        let start_idx = self.width * row + col;
        self.values
            .iter()
            .skip(start_idx + 1)
            .take(self.width - col - 1)
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> TreeGrid {
    let mut values = Vec::new();

    let width = input.find('\n').unwrap();
    let mut height = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                height += 1;
            }
            height => {
                values.push(height.to_digit(10).unwrap());
            }
        }
    }

    TreeGrid {
        height,
        width,
        values,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &TreeGrid) -> usize {
    let mut visible = 0;
    for row in 0..input.height {
        for col in 0..input.width {
            let current_height = input.at(row, col);

            // A tree is visible from a direction iff every
            // tree in that direction is smaller than it.
            let visible_left = input.left(row, col).all(|h| *h < current_height);
            let visible_right = input.right(row, col).all(|h| *h < current_height);
            let visible_above = input.above(row, col).all(|h| *h < current_height);
            let visible_below = input.below(row, col).all(|h| *h < current_height);

            if visible_left || visible_right || visible_above || visible_below {
                visible += 1;
            }
        }
    }

    visible
}

#[aoc(day8, part2)]
pub fn part2(input: &TreeGrid) -> usize {
    let mut best = 0;
    for row in 0..input.height {
        for col in 0..input.width {
            let initial = input.at(row, col);

            // Count trees in each direction until you find either the edge or a larger one.
            // take_until() includes the first non-matching element, unlike take_while().
            let left = input.left(row, col).take_until(|h| **h >= initial).count();
            let right = input.right(row, col).take_until(|h| **h >= initial).count();
            let above = input.above(row, col).take_until(|h| **h >= initial).count();
            let below = input.below(row, col).take_until(|h| **h >= initial).count();

            let score = left * right * above * below;
            best = std::cmp::max(best, score);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "30373\n\
                           25512\n\
                           65332\n\
                           33549\n\
                           35390\n";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 8);
    }
}
