use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(input: &str) -> Self {
        match input {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&mut self, direction: &Direction) {
        match *direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn move_towards(&mut self, other: Point) {
        match (other.x - self.x, other.y - self.y) {
            (-1, -1)
            | (0, -1)
            | (1, -1)
            | (-1, 0)
            | (0, 0)
            | (1, 0)
            | (-1, 1)
            | (0, 1)
            | (1, 1) => {}
            (0, dy) => self.y += dy / 2,
            (dx, 0) => self.x += dx / 2,
            (dx, dy) => {
                self.x += dx / dx.abs();
                self.y += dy / dy.abs();
            }
        }
    }
}

struct Rope<const N: usize> {
    knots: [Point; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope {
            knots: [Point { x: 0, y: 0 }; N],
        }
    }

    fn pull(&mut self, direction: &Direction) {
        // Move the head of the rope
        self.knots[0].translate(direction);

        // Move each other knot in turn
        for knot in 1..N {
            self.knots[knot].move_towards(self.knots[knot - 1]);
        }
    }
}

pub struct Step {
    direction: Direction,
    count: usize,
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|line| {
            let (direction_str, count_str) = line.split_once(' ').unwrap();
            let direction = Direction::from_str(direction_str);
            let count = count_str.parse::<usize>().unwrap();

            Step { direction, count }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Step>) -> usize {
    let mut rope = Rope::<2>::new();
    let mut tail_positions: HashSet<Point> = HashSet::new();

    for step in input {
        for _ in 0..step.count {
            rope.pull(&step.direction);
            tail_positions.insert(rope.knots[1]);
        }
    }

    tail_positions.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Step>) -> usize {
    let mut rope = Rope::<10>::new();
    let mut tail_positions: HashSet<Point> = HashSet::new();

    for step in input {
        for _ in 0..step.count {
            rope.pull(&step.direction);
            tail_positions.insert(rope.knots[9]);
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const SMALL_EXAMPLE: &str = "R 4\n\
                                 U 4\n\
                                 L 3\n\
                                 D 1\n\
                                 R 4\n\
                                 D 1\n\
                                 L 5\n\
                                 R 2";

    const LARGE_EXAMPLE: &str = "R 5\n\
                                 U 8\n\
                                 L 8\n\
                                 D 3\n\
                                 R 17\n\
                                 D 10\n\
                                 L 25\n\
                                 U 20";

    #[test]
    fn test_part1() {
        let small_input = generator(SMALL_EXAMPLE);
        assert_eq!(part1(&small_input), 13);
    }

    #[test]
    fn test_part2() {
        let small_input = generator(SMALL_EXAMPLE);
        assert_eq!(part2(&small_input), 1);

        let large_input = generator(LARGE_EXAMPLE);
        assert_eq!(part2(&large_input), 36);
    }
}
