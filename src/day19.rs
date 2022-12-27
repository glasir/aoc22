use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

enum Material {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

use Material::*;

/**
 * A generic bag of one of more resources.
 *
 * This can be used in a lot of ways, which we will definitely (ab)use:
 *  * how many of each resource does a factory have?
 *  * how many of each type of robot does a factory have?
 *  * how much does one type of robot cost?
 *
 * Since we'll never need to track more than ~30 resources of any type,
 * this is internally represented as a 32-bit integer:
 *32       24       16        8        0
 * +--------+--------+--------+--------+
 * |  geode |obsidian|  clay  |  ore   |
 * +--------+--------+--------+--------+
 *
 * This makes comparisons, addition, and subtraction very fast.
 *
 * This implementation is pretty unnecessary but I thought it was
 * an interesting micro-optimization. It probably would have been more
 * worthwhile to spend the time thinking about better heuristics.
 */
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Resources {
    data: u32,
}

impl Resources {
    /**
     * Adds the contents of two bags of resources.
     */
    fn add(&self, other: Self) -> Self {
        Self {
            data: self.data + other.data,
        }
    }

    /**
     * Utility method to add some quantity of a single material.
     * This comes up a lot (e.g. adding one Ore robot).
     */
    fn add_one(&self, material: Material) -> Self {
        Self {
            data: self.data + Resources::encode_material(&material),
        }
    }

    /**
     * Attempts to subtract one bag of resources from another.
     * Returns None if there isn't enough of any material to subtract,
     * or Some(difference) if there's enough of each material.
     */
    fn checked_sub(&self, other: Self) -> Option<Self> {
        // We explicitly *want* an underflowing subtraction.
        let difference = self.data.wrapping_sub(other.data);

        // We're not really subtracting u32's, we're subtracting four u8's in parallel.
        // Any of those u8 subtractions could have underflowed.
        // Since we will never store large numbers in this struct, we know:
        //   * the highest bit should *never* be set unless there's been an underflow;
        //   * the largest possible underflow is < 128
        // This means that a u8 subtraction has underflowed iff the high bit of any
        // byte is set, which we can check in a single operation.
        if difference & 0x80808080 == 0 {
            Some(Self { data: difference })
        } else {
            None
        }
    }

    /**
     * Creates a new empty bag of materials.
     */
    fn new() -> Self {
        Self { data: 0 }
    }

    /**
     * Creates a bag of resources out of a list.
     */
    fn from(materials: &[Material]) -> Self {
        let mut data = 0;
        for material in materials {
            data += Resources::encode_material(material);
        }
        Self { data }
    }

    /**
     * Creates a bag of resources containing a single material.
     */
    fn from_one(material: Material) -> Self {
        Self {
            data: Self::encode_material(&material),
        }
    }

    /**
     * Converts a material to the internal representation.
     */
    fn encode_material(material: &Material) -> u32 {
        match material {
            Ore(count) => *count,
            Clay(count) => *count << 8,
            Obsidian(count) => *count << 16,
            Geode(count) => *count << 24,
        }
    }

    /*
     * Getters for each resource type.
     */

    fn ore(&self) -> u32 {
        self.data & 0x000000FF
    }

    fn clay(&self) -> u32 {
        (self.data & 0x0000FF00) >> 8
    }

    fn obsidian(&self) -> u32 {
        (self.data & 0x00FF0000) >> 16
    }

    // Included for completeness; we've optimized out all calls to this.
    #[allow(dead_code)]
    fn geode(&self) -> u32 {
        (self.data & 0xFF000000) >> 24
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ ore: {}, clay: {}, obsidian: {}, geode: {} }}",
            self.data & 0x000000FF,
            (self.data & 0x0000FF00) >> 8,
            (self.data & 0x00FF0000) >> 16,
            (self.data & 0xFF000000) >> 24
        )
    }
}

#[derive(Clone, Debug)]
pub struct RobotCosts {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

#[derive(Clone)]
pub struct RobotFactory {
    id: u32,
    resources: Resources,
    robots: Resources,
    costs: RobotCosts,
}

impl fmt::Debug for RobotFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RobotFactory({}) {{ resources: {:?}, robots: {:?} }}",
            self.id, self.resources, self.robots
        )
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State {
    time_remaining: u32,
    resources: Resources,
    robots: Resources,
}

impl RobotFactory {
    fn new(id: u32, costs: RobotCosts) -> Self {
        RobotFactory {
            id,
            resources: Resources::new(),
            robots: Resources::from_one(Ore(1)),
            costs,
        }
    }

    /*
     * The next several functions attempt to build a robot starting from an
     * initial factory state, given the number of minutes remaining.
     *
     * Returns either:
     *   * None, if the robot cannot be completed in the time remaining, or
     *     if building the robot would not be useful;
     *   * A pair containing the time remaining after the ore robot is
     *     completed, and the state of the factory once it's finished building.
     *
     * Note that it might take multiple minutes to gather the necessary resources
     * before building the robot!
     */

    fn build_ore_robot(&self, time_remaining: u32) -> Option<(u32, RobotFactory)> {
        // Optimization: never, ever build more ore robots than the largest ore cost.
        // This would cause us to generate more ore per minute than we can spend.
        #[rustfmt::skip]
        let highest_ore_cost = self.costs.ore.ore()
            .max(self.costs.clay.ore())
            .max(self.costs.obsidian.ore())
            .max(self.costs.geode.ore());
        if self.robots.ore() >= highest_ore_cost {
            return None;
        }

        self.build_robot(Ore(1), self.costs.ore, time_remaining)
    }

    fn build_clay_robot(&self, time_remaining: u32) -> Option<(u32, RobotFactory)> {
        // Never build more clay robots than the highest clay cost.
        // Only obsidian robots cost clay, so this is thankfully easier than ore.
        if self.robots.clay() >= self.costs.obsidian.clay() {
            return None;
        }

        self.build_robot(Clay(1), self.costs.clay, time_remaining)
    }

    fn build_obsidian_robot(&self, time_remaining: u32) -> Option<(u32, RobotFactory)> {
        // Never build more obsidian robots than the highest obsidian cost.
        if self.robots.obsidian() >= self.costs.geode.obsidian() {
            return None;
        }

        // We will always have at least one ore robot; we need at least one
        // clay robot to be able to *eventually* produce an obsidian robot.
        if self.robots.clay() == 0 {
            return None;
        }

        self.build_robot(Obsidian(1), self.costs.obsidian, time_remaining)
    }

    fn build_geode_robot(&self, time_remaining: u32) -> Option<(u32, RobotFactory)> {
        // Make sure there's an obsidian robot available to gather the resources
        // necessary for this geode robot.
        if self.robots.obsidian() == 0 {
            return None;
        }

        // Optimization: we never need to track the number of geode robots, since
        // we count up the geodes it opens as soon as it's built (in find_best()).
        // So, add "zero" geode robots to our state to reduce the space of
        // possible cache keys.
        self.build_robot(Geode(0), self.costs.geode, time_remaining)
    }

    fn build_robot(
        &self,
        robot_type: Material,
        cost: Resources,
        time_remaining: u32,
    ) -> Option<(u32, RobotFactory)> {
        // It always takes 1 minute to build the robot.
        let mut new_time_remaining = time_remaining - 1;

        // Simulate minutes repeatedly until we either run out of time, or
        // have enough resources to build the robot.
        let mut resources = self.resources;
        loop {
            if new_time_remaining == 0 {
                // Ran out of time! We can't usefully build this robot.
                return None;
            } else if let Some(after_build) = resources.checked_sub(cost) {
                // We have enough resources!
                // Gather resources for the minute we'll spend building the robot.
                resources = after_build.add(self.robots);

                // Return a factory state with updated resources robots.
                return Some((
                    new_time_remaining,
                    RobotFactory {
                        id: self.id,
                        resources,
                        robots: self.robots.add_one(robot_type),
                        costs: self.costs.clone(),
                    },
                ));
            } else {
                // We don't have enough resources this minute; gather for another
                // minute then try again.
                new_time_remaining -= 1;
                resources = resources.add(self.robots);
            }
        }
    }
}

/**
 * Returns the maximum number of geodes that can be opened by robots produced
 * on or after the current time.
 *
 * This somewhat-awkward phrasing means that we no longer need to track the
 * total number of geodes or geode robots; this reduces the number of states.
 *
 * Inputs:
 *  * the current factory state
 *  * the amount of time remaining
 *  * a cache of visited states
 *
 * The general approach is to pick out a type of robot to build next and recurse
 * to find how many geodes we can open given that choice, then return the best.
 *
 * My original code simulated each minute rather than each decision; this approach
 * cuts down the number of branches we explore and is much faster.
 */
fn find_best(factory: &RobotFactory, time_remaining: u32, memo: &mut HashMap<State, u32>) -> u32 {
    // If there's no time left, we can neither open geodes nor build robots.
    // If there's only one minute left, we can make some new robots, but
    // they won't have time to produce anything.
    // Either way, no new robots can open geodes, so return 0.
    if time_remaining <= 1 {
        return 0;
    }

    // If we've already explored this state, we know the answer.
    let state = State {
        time_remaining,
        resources: factory.resources,
        robots: factory.robots,
    };

    if memo.contains_key(&state) {
        return memo[&state];
    }

    // There are at least two minutes left, so we have options.
    //  1. Figure out what robots the factory can build (possibly over several minutes!).
    //  2. Generate the factory state and updated time remaining for each option.
    //  3. Recurse with an updated factory state and time_remaining.
    //  4. Find the best option.
    let mut best: u32 = 0;

    // build_geode_robot() returns (time remaining after build, factory state after build).
    if let Some((time, after_build)) = factory.build_geode_robot(time_remaining) {
        // The new geode robot will open 1 geode per minute after being built.
        best = time;

        // Figure out how many geodes can be opened by future robots we build.
        best += find_best(&after_build, time, memo);

        // Optimization: if we *can* build a geode robot this minute, we should do so.
        // No other options needs to be explored.
        //
        // Note that it's possible to construct pathological blueprints for which this
        // optimization gives the wrong answer! I believe this can only happen when a
        // geode robot costs very little obsidian, which isn't the case for my input.
        if time == time_remaining - 1 {
            return best;
        }
    }

    // See whether we can make each type of robot in turn given the robots available.
    if let Some((time, after_build)) = factory.build_ore_robot(time_remaining) {
        let build_ore = find_best(&after_build, time, memo);
        best = best.max(build_ore);
    }

    if let Some((time, after_build)) = factory.build_clay_robot(time_remaining) {
        let build_clay = find_best(&after_build, time, memo);
        best = best.max(build_clay);
    }

    if let Some((time, after_build)) = factory.build_obsidian_robot(time_remaining) {
        let build_obsidian = find_best(&after_build, time, memo);
        best = best.max(build_obsidian);
    }

    // The recursive call returns the best *total* number of geodes.
    // Store it for later use, then return it.
    memo.insert(state, best);

    best
}

#[aoc_generator(day19)]
fn create_factories(input: &str) -> Vec<RobotFactory> {
    let re = regex::Regex::new(r"(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = re
                .captures_iter(line)
                .map(|m| m.get(1).unwrap().as_str().parse::<u32>().unwrap())
                .collect();
            let id = numbers[0];
            let costs = RobotCosts {
                ore: Resources::from_one(Ore(numbers[1])),
                clay: Resources::from_one(Ore(numbers[2])),
                obsidian: Resources::from(&[Ore(numbers[3]), Clay(numbers[4])]),
                geode: Resources::from(&[Ore(numbers[5]), Obsidian(numbers[6])]),
            };

            RobotFactory::new(id, costs)
        })
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(factories: &[RobotFactory]) -> u32 {
    let mut result: u32 = 0;
    for factory in factories.iter() {
        let mut memo = HashMap::new();
        let factory_best = find_best(factory, 24, &mut memo);
        result += factory_best * factory.id;
    }

    result
}

#[aoc(day19, part2)]
pub fn part2(factories: &[RobotFactory]) -> u32 {
    let best: Vec<u32> = factories
        .iter()
        .take(3)
        .map(|factory| find_best(factory, 32, &mut HashMap::new()))
        .collect();

    best[0] * best[1] * best[2]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{create_factories, part1};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day19.txt").expect("missing input");
        let factories = create_factories(&input);
        assert_eq!(part1(&factories), 33);
    }
}
