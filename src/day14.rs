use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt,
};

/**
 * The various materials used in this problem.
 *
 * It doesn't really matter when simulating whether a grain of sand
 * is stopped by rock or by more sand, but this makes for a nicer picture.
 */
#[derive(Clone, Copy, PartialEq)]
enum Material {
    Rock,
    Air,
    Sand,
}

/**
 * The cave into which we're dropping sand.
 *
 * Uses a sparse map of (x,y) -> material, and tracks the bounding
 * box of all of the walls in the cave so we can tell when sand
 * starts to escape.
 */
#[derive(Clone)]
pub struct Cave {
    map: HashMap<(i32, i32), Material>,
    bounds: BoundingBox,
}

/**
 * A simple 2D bounding box.
 */
#[derive(Clone)]
pub struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl BoundingBox {
    /**
     * Extends the bounding box to contain a new point.
     */
    fn extend(&mut self, x: i32, y: i32) {
        self.x_min = min(self.x_min, x);
        self.x_max = max(self.x_max, x);
        self.y_min = min(self.y_min, y);
        self.y_max = max(self.y_max, y);
    }

    /**
     * Checks whether this bounding box contains a point.
     */
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }
}

impl Cave {
    /**
     * Creates a new cave with no contents.
     */
    fn new() -> Self {
        Cave {
            map: HashMap::new(),

            // The "source point" for the sand is at (500, 0),
            // so initialize the bounding box to include that point.
            bounds: BoundingBox {
                x_min: 500,
                x_max: 500,
                y_min: 0,
                y_max: 0,
            },
        }
    }

    /**
     * Gets the material at (x,y), defaulting to Air.
     */
    fn get(&self, x: i32, y: i32) -> Material {
        self.map.get(&(x, y)).copied().unwrap_or(Material::Air)
    }

    /**
     * Adds a wall at (x,y), extending the bounding box to include that point.
     */
    fn add_wall(&mut self, x: i32, y: i32) {
        self.map.insert((x, y), Material::Rock);
        self.bounds.extend(x, y);
    }

    /**
     * Simulates dropping a grain of sand from the source point.
     *
     * If the sand stops falling at a point (x,y) within the cave, returns Some((x,y)).
     * If the sand exits the cave's bounding box, returns None.
     */
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
                // Try down-and-left.
                (x, y) = (x - 1, y + 1);
            } else if self.get(x + 1, y + 1) == Material::Air {
                // Try down-and-right.
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

        // For each set of coordinates, draw a wall from the current point to that coordinate
        for corner in wall {
            // Only one of these loops will do something useful.
            for x in min(current.0, corner.0)..=max(current.0, corner.0) {
                cave.add_wall(x, current.1);
            }

            for y in min(current.1, corner.1)..=max(current.1, corner.1) {
                cave.add_wall(current.0, y);
            }

            // Update the current point for the next wall segment.
            current = corner;
        }
    }

    cave
}

#[aoc(day14, part1)]
pub fn part1(input: &Cave) -> u32 {
    let mut cave = input.clone();

    // Simulate until we try to drop a grain of sand and it falls out of the cave.
    let mut count = 0;
    while cave.add_sand().is_some() {
        count += 1;
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Cave) -> i32 {
    let mut cave = input.clone();

    // Add an "infinite" floor (i.e., wide enough so that sand must
    // reach the source point before it falls off the floor).
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
