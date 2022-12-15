use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt,
};

#[derive(Clone, Copy, PartialEq)]
enum Material {
    Rock,
    Air,
    Sand,
}

#[derive(Clone)]
pub struct Cave {
    map: HashMap<(i32, i32), Material>,
    bounds: BoundingBox,
}

#[derive(Clone)]
pub struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl BoundingBox {
    fn extend(&mut self, x: i32, y: i32) {
        self.x_min = min(self.x_min, x);
        self.x_max = max(self.x_max, x);
        self.y_min = min(self.y_min, y);
        self.y_max = max(self.y_max, y);
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }
}

impl Cave {
    fn new() -> Self {
        Cave {
            map: HashMap::new(),
            bounds: BoundingBox {
                x_min: 500,
                x_max: 500,
                y_min: 0,
                y_max: 0,
            },
        }
    }

    fn get(&self, x: i32, y: i32) -> Material {
        self.map.get(&(x, y)).copied().unwrap_or(Material::Air)
    }

    fn add_wall(&mut self, x: i32, y: i32) {
        self.map.insert((x, y), Material::Rock);
        self.bounds.extend(x, y);
    }

    fn add_sand(&mut self) -> Option<(i32, i32)> {
        // Every piece of sand starts at the source, at (500, 0).
        let mut x: i32 = 500;
        let mut y: i32 = 0;

        loop {
            // If we've broken out of the bounding box, bail.
            if !self.bounds.contains(x, y) {
                return Option::None;
            }

            // First, try to go straight down.
            // Note the inverted coordinate system.
            if self.get(x, y + 1) == Material::Air {
                y += 1;
            } else if self.get(x - 1, y + 1) == Material::Air {
                (x, y) = (x - 1, y + 1);
            } else if self.get(x + 1, y + 1) == Material::Air {
                (x, y) = (x + 1, y + 1);
            } else {
                // We got stuck! This sand has completed falling.
                self.map.insert((x, y), Material::Sand);
                return Option::Some((x, y));
            }
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bounds.y_min..=self.bounds.y_max {
            for x in self.bounds.x_min..=self.bounds.x_max {
                match self.get(x, y) {
                    Material::Rock => {
                        write!(f, "#")?;
                    }
                    Material::Sand => {
                        write!(f, "o")?;
                    }
                    Material::Air => {
                        write!(f, " ")?;
                    }
                }
            }
            writeln!(f)?;
        }
        fmt::Result::Ok(())
    }
}

fn parse_coords(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Cave {
    let mut cave = Cave::new();

    for line in input.lines() {
        let mut wall = line.split(" -> ").map(parse_coords);
        let mut current = wall.next().unwrap();

        for corner in wall {
            // Only one of these loops will do something useful.
            for x in min(current.0, corner.0)..=max(current.0, corner.0) {
                cave.add_wall(x, current.1);
            }

            for y in min(current.1, corner.1)..=max(current.1, corner.1) {
                cave.add_wall(current.0, y);
            }

            current = corner;
        }
    }

    cave
}

#[aoc(day14, part1)]
pub fn part1(input: &Cave) -> u32 {
    let mut cave = input.clone();

    let mut count = 0;
    while cave.add_sand().is_some() {
        count += 1;
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Cave) -> i32 {
    let mut cave = input.clone();

    // Add a floor
    let floor_height = cave.bounds.y_max + 2;
    for x in cave.bounds.x_min - 500..cave.bounds.x_max + 500 {
        cave.add_wall(x, floor_height);
    }

    // Simulate until the sand is placed at (500, 0)
    let mut count = 0;
    while let Some((x, y)) = cave.add_sand() {
        count += 1;

        if (x, y) == (500, 0) {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6\n\
                           503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 93);
    }
}
