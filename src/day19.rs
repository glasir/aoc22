use std::collections::HashMap;

/**
 * A generic bag of one of more resources.
 *
 * This can be used in a lot of ways, which we will definitely (ab)use:
 *  * how many of each resource does a factory have?
 *  * how many of each type of robot does a factory have?
 *  * how much does one type of robot cost?
 */
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resources {
    fn add(&self, other: Self) -> Self {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn checked_sub(&self, other: Self) -> Option<Self> {
        let Some(ore) = self.ore.checked_sub(other.ore) else { return None };
        let Some(clay) = self.clay.checked_sub(other.clay) else { return None };
        let Some(obsidian) = self.obsidian.checked_sub(other.obsidian) else { return None };
        let Some(geode) = self.geode.checked_sub(other.geode) else { return None };

        Some(Resources {
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

#[derive(Clone, Debug)]
struct RobotCosts {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

#[derive(Clone, Debug)]
struct Factory {
    id: u32,
    resources: Resources,
    robots: Resources,
    costs: RobotCosts,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct State {
    time_remaining: u32,
    resources: Resources,
    robots: Resources,
}

impl Factory {
    fn new(id: u32, costs: RobotCosts) -> Self {
        Factory {
            id,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            costs,
        }
    }

    fn do_nothing(&self) -> Factory {
        Factory {
            id: self.id,
            resources: self.resources.add(self.robots),
            robots: self.robots,
            costs: self.costs.clone(),
        }
    }

    fn build_ore_robot(&self) -> Option<Factory> {
        // Optimization: never, ever build more ore robots than the largest ore cost.
        let highest_ore_cost = self
            .costs
            .ore
            .ore
            .max(self.costs.clay.ore)
            .max(self.costs.obsidian.ore)
            .max(self.costs.geode.ore);
        if self.robots.ore >= highest_ore_cost {
            return None;
        }

        self.resources.checked_sub(self.costs.ore).map(|extras| {
            let resources = extras.add(self.robots);
            let robots = self.robots.add(Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            });
            Factory {
                id: self.id,
                resources,
                robots,
                costs: self.costs.clone(),
            }
        })
    }

    fn build_clay_robot(&self) -> Option<Factory> {
        if self.robots.clay >= self.costs.obsidian.clay {
            return None;
        }

        self.resources.checked_sub(self.costs.clay).map(|extras| {
            let resources = extras.add(self.robots);
            let robots = self.robots.add(Resources {
                ore: 0,
                clay: 1,
                obsidian: 0,
                geode: 0,
            });
            Factory {
                id: self.id,
                resources,
                robots,
                costs: self.costs.clone(),
            }
        })
    }

    fn build_obsidian_robot(&self) -> Option<Factory> {
        if self.robots.obsidian >= self.costs.geode.obsidian {
            return None;
        }

        self.resources
            .checked_sub(self.costs.obsidian)
            .map(|extras| {
                let resources = extras.add(self.robots);
                let robots = self.robots.add(Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                    geode: 0,
                });
                Factory {
                    id: self.id,
                    resources,
                    robots,
                    costs: self.costs.clone(),
                }
            })
    }

    fn build_geode_robot(&self) -> Option<Factory> {
        self.resources.checked_sub(self.costs.geode).map(|extras| {
            let resources = extras.add(self.robots);
            let robots = self.robots.add(Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 1,
            });
            Factory {
                id: self.id,
                resources,
                robots,
                costs: self.costs.clone(),
            }
        })
    }
}

fn create_factories(input: &str) -> Vec<Factory> {
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
                ore: Resources {
                    ore: numbers[1],
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay: Resources {
                    ore: numbers[2],
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian: Resources {
                    ore: numbers[3],
                    clay: numbers[4],
                    obsidian: 0,
                    geode: 0,
                },
                geode: Resources {
                    ore: numbers[5],
                    clay: 0,
                    obsidian: numbers[6],
                    geode: 0,
                },
            };

            Factory::new(id, costs)
        })
        .collect()
}

// Returns the maximum number of opened geodes possible starting from
// an initial factory state after `time_remaining` minutes.
fn find_best(factory: &Factory, time_remaining: u32, memo: &mut HashMap<State, u32>) -> u32 {
    // If there's no time remaining, return the number of geodes already opened.
    if time_remaining == 0 {
        return factory.resources.geode;
    }

    // If there's only one minute left, we can make some new robots, but
    // they won't have time to produce anything. So, all we can do is use
    // existing robots to open some more geodes.
    if time_remaining == 1 {
        return factory.resources.geode + factory.robots.geode;
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
    //  1. Figure out what robots the factory can create
    //  2. Generate a factory state after executing each option
    //  3. Recurse with an updated factory state and time_remaining
    //  4. Find the best option.
    let mut best: u32;

    // Optimization: if we *can* build a geode robot, we should do so.
    // No other options needs to be explored.
    if let Some(with_geode_robot) = factory.build_geode_robot() {
        let build_geode = find_best(&with_geode_robot, time_remaining - 1, memo);
        best = build_geode;
    } else {
        // We can always do nothing (e.g., to save up for a more expensive robot).
        let do_nothing = find_best(&factory.do_nothing(), time_remaining - 1, memo);
        best = do_nothing;

        // See whether we can make each type of robot in turn.
        if let Some(with_ore_robot) = factory.build_ore_robot() {
            let build_ore = find_best(&with_ore_robot, time_remaining - 1, memo);
            best = best.max(build_ore);
        }

        if let Some(with_clay_robot) = factory.build_clay_robot() {
            let build_clay = find_best(&with_clay_robot, time_remaining - 1, memo);
            best = best.max(build_clay);
        }

        if let Some(with_obsidian_robot) = factory.build_obsidian_robot() {
            let build_obsidian = find_best(&with_obsidian_robot, time_remaining - 1, memo);
            best = best.max(build_obsidian);
        }
    }

    // The factory states store an up-to-date record of the geodes opened,
    // and the recursive call returns the best *total* number of geodes.
    // Which is exactly the answer we want!
    // Store it for later use, then return it.
    memo.insert(state, best);

    best
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
    let factories = create_factories(input);

    let mut result: u32 = 0;
    for factory in factories.iter() {
        let mut memo = HashMap::new();
        let factory_best = find_best(factory, 24, &mut memo);
        println!("Best from factory {}: {}", factory.id, factory_best);
        result += factory_best * factory.id;
    }

    result
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u32 {
    let best: Vec<u32> = create_factories(input)
        .iter()
        .take(3)
        .map(|factory| find_best(factory, 32, &mut HashMap::new()))
        .collect();

    println!("Best results: {:?}", best);
    best[0] * best[1] * best[2]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day19.txt").expect("missing input");
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day19.txt").expect("missing input");
        assert_eq!(part2(&input), 4);
    }
}
