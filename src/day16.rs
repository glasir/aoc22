use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, u32},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use pathfinding::directed::dijkstra::dijkstra;
use std::{cmp::max, collections::HashMap, fmt::Display};

// For efficiency (and convenience!) we'll store room status in a bitset.
// This is *much* faster than using e.g. a HashSet<String>.
//
// This is an extremely limited implementation that supports at most 32 elements.
// It's fine for this problem, though, since we only have ~15 relevant nodes.
#[derive(Clone)]
struct Bitset {
    bits: u32,
}

#[allow(dead_code)]
impl Bitset {
    fn new() -> Self {
        Bitset { bits: 0 }
    }

    fn insert(&mut self, value: usize) {
        self.bits |= 1u32 << value;
    }

    fn remove(&mut self, value: usize) {
        self.bits &= !(1u32 << value);
    }

    fn contains(&self, value: usize) -> bool {
        (self.bits & (1u32 << value)) != 0
    }

    fn iter(&self) -> BitsetIterator {
        BitsetIterator {
            bitset: self,
            current: 0,
        }
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<usize> = self.iter().collect();
        write!(f, "{:?}", items)
    }
}

struct BitsetIterator<'a> {
    bitset: &'a Bitset,
    current: usize,
}

impl<'a> Iterator for BitsetIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current..32 {
            if self.bitset.contains(i) {
                self.current = i + 1;
                return Some(i);
            }
        }

        None
    }
}

/**
 * A naive representation of the graph of rooms.
 *
 * Graph.nodes maps room labels to flow capacity;
 * Graph.nodes map room labels to { labels of connected rooms }.
 *
 * This is quite inefficient, so this representation is mostly used
 * as an intermediate step to producing a much more optimized version.
 */
struct Graph {
    nodes: HashMap<String, u32>,
    edges: HashMap<String, Vec<String>>,
}

fn parse_room(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), u32),
        delimited(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
            multispace0,
        ),
    ))(input)
}

fn parse_graph(input: &str) -> Graph {
    let (_, rooms) = many1(parse_room)(input).expect("parse error");

    let mut flows = HashMap::new();
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();

    for entry in rooms.iter() {
        let label = entry.0.to_string();
        flows.insert(label.clone(), entry.1);
        neighbors.insert(label, entry.2.iter().map(|l| l.to_string()).collect());
    }

    Graph {
        nodes: flows,
        edges: neighbors,
    }
}

/* The full graph has a lot of nodes with value 0.
 * We don't really care about those, so after reading in the full graph,
 * we'll do some work to "compress" it:
 *   1. Eliminate zero-valued nodes.
 *   2. Precompute pairwise distances between all nonzero-valued nodes
 *      (effectively converting it to a complete graph).
 *   3. Represent the nodes with integers instead of strings
 *   4. Store values and edge weights in arrays for fast lookup
 *
 * The starting node is always mapped to the highest ID to help optimize some later stuff;
 * specifically, its ID is always graph.size - 1.
 */
const MAX_IMPORTANT_ROOMS: usize = 16;
struct CompressedGraph {
    flows: [u32; MAX_IMPORTANT_ROOMS],
    distances: [[u32; MAX_IMPORTANT_ROOMS]; MAX_IMPORTANT_ROOMS],
    size: usize,
}

fn compress_graph(graph: &Graph) -> CompressedGraph {
    // Find all of the rooms with nonzero flow.
    // These (and AA) are the only ones we actually care about.
    let important_rooms: Vec<String> = graph
        .nodes
        .iter()
        .filter_map(|(label, flow)| {
            if label == "AA" || *flow > 0 {
                Some(label.to_owned())
            } else {
                None
            }
        })
        .collect();

    // Find pairwise distances between each of the important rooms.
    // This uses N^2 runs of Dijkstra, where N is the number of important rooms.
    // There is probably a more efficient way - I thought about Floyd-Warshall
    // but there are quite a lot of unimportant rooms. *shrug*
    let mut pairwise_distances: HashMap<(String, String), u32> = HashMap::new();
    for source in important_rooms.iter() {
        for dest in important_rooms.iter() {
            if source == dest {
                continue;
            }

            let (_, distance) = dijkstra(
                source,
                |room| {
                    graph
                        .edges
                        .get(room)
                        .unwrap()
                        .iter()
                        .map(|label| (label.clone(), 1))
                },
                |room| *room == *dest,
            )
            .expect("no path found");

            pairwise_distances.insert((source.to_owned(), dest.to_owned()), distance);
        }
    }

    // Okay, we've reduced the graph to the nodes we care about.
    // Let's produce an efficient representation of that smaller graph.

    // First, convert string labels to numeric IDs.
    // There is some hackiness to make sure that "AA" has the largest ID.
    // This is messy now but greatly simplifies some things later.
    let mut label_to_id: HashMap<String, usize> = HashMap::new();
    for label in important_rooms.iter() {
        if label != "AA" {
            label_to_id.insert(label.to_owned(), label_to_id.len());
        }
    }
    label_to_id.insert(String::from("AA"), label_to_id.len());

    // Convert the HashMap of flows to a flat array, indexed by room ID.
    let mut flows = [0; MAX_IMPORTANT_ROOMS];
    for label in important_rooms.iter() {
        let room_id = label_to_id[label];
        flows[room_id] = graph.nodes[label];
    }

    // Instead of using nested HashMaps to store distances between nodes,
    // use a 2d array, indexed by source ID and destination ID.
    let mut distances = [[0; MAX_IMPORTANT_ROOMS]; MAX_IMPORTANT_ROOMS];
    for ((source, dest), distance) in pairwise_distances.iter() {
        let source_id = label_to_id[source];
        let dest_id = label_to_id[dest];
        distances[source_id][dest_id] = distance.to_owned();
    }

    CompressedGraph {
        flows,
        distances,
        size: important_rooms.len(),
    }
}

/**
 * Uses backtracking to find the maximum release-able pressure.
 * Inputs:
 *   - the compressed graph we're computing over
 *   - the time remaining
 *   - the current room (represented as an ID)
 *   - the set of rooms we could visit next (as a Bitset)
 *
 * Returns:
 *   - the maximum pressure releasable in the remaining time.
 *
 * Preconditions:
 *   - the current room was the active room in a previous step.
 *     This
 */
fn backtrack(
    graph: &CompressedGraph,
    time_remaining: u32,
    current_room: usize,
    active_rooms: Bitset,
) -> u32 {
    // If there's 0 minutes left, we're done.
    // If there's 1 minute left, we can spend it by either
    //   - opening the valve in the current room
    //   - go to another room
    //   - do nothing
    // None of these release any pressure, so just do nothing.
    if time_remaining <= 1 {
        return 0;
    }

    // If there's exactly 2 minutes remaining, the only way to actually
    // release any pressure is to open the valve in the current room.
    // This takes one minute, and the last minute is spent releasing pressure.
    if time_remaining == 2 {
        return graph.flows[current_room];
    }

    // There are at least 3 minutes left. We have options!

    // First, because of the precondition, we know that a previous step chose to
    // visit this room next. That means that we should open the valve here!
    // The only exception is on the very first move: the starting room may have
    // flow 0, so there's no reason to open the valve.
    let mut current_room_cost = 0;
    let mut current_room_value: u32 = 0;

    let current_flow = graph.flows[current_room];
    if current_flow > 0 {
        current_room_cost = 1;

        // Multiply by time_remaining - 1 because it takes a minute to open the valve.
        current_room_value = current_flow * (time_remaining - 1);
    }

    // Now we need to figure out the best room to visit next.
    // Fortunately, we have a bitset of the possible options.
    let mut best: u32 = current_room_value;

    for next_room in active_rooms.iter() {
        // Going to this next room will take some time.
        // This might eliminate it as a possibility.
        let movement_cost = graph.distances[current_room][next_room];
        if movement_cost > time_remaining - 1 {
            continue;
        }

        // Once we go to that room, there'll never be a reason to go back,
        // so remove it from the list of active rooms.
        let mut next_possibilities = active_rooms.clone();
        next_possibilities.remove(next_room);

        // Recurse!
        let next_room_value = backtrack(
            graph,
            time_remaining - current_room_cost - movement_cost,
            next_room,
            next_possibilities,
        );

        best = max(best, current_room_value + next_room_value);
    }

    best
}

/**
 * Generates all partitions of a set of n objects into 2 subsets.
 * Returns a series of pairs of bitsets representing the subsets.
 *
 * The implementation relies heavily on the internal representation of a bitset.
 * Specifically, it uses the fact that a bitset containing [0, 1, ..., n-1]
 * is stored as 0b011...11 (n '1' bits). That means that we can
 * generate partitions by simply counting from 0 up to 2^n - 1;
 * the '0' bits correspond to elements in one partition, while the
 * '1' bits correspond to elements in the other.
 *
 * Example: if n = 6 and the counter is 0b011001, then the partitions
 * are {0, 3, 4} (the '1' bits) and {1, 2, 5} (the '0' bits).
 * To get the next pair of partitions, add 1 to the counter to get
 * 0b011010, representing {1, 3, 4} and {0, 2, 5}.
 *
 * Then because a bitset is just a u32, we can create bitsets representing
 * the two partitions as just `counter` and `!counter`, modulo
 * masking out some irrelevant high bits.
 *
 * As an additional optimization, we can use the fact that both
 * partitions are processed identically to skip generating half of them.
 * For example, ({0, 1, 3, 4}, {2, 5}) and ({2, 5}, {0, 1, 3, 4}) will
 * give the same results, so don't bother checking both.
 * This is implemented using a popcount, which limits the first partition
 * to having <= half of its bits set. There is still some repeated work,
 * but it cuts the number of pairs returned by ~1/2 and is fast enough.
 */
fn partitions(n: usize) -> impl Iterator<Item = (Bitset, Bitset)> {
    let max_value = 1u32 << n;
    let mask = max_value - 1;
    let max_bits = (n as u32) / 2;

    (0..max_value).filter_map(move |value| {
        if value.count_ones() > max_bits {
            return None;
        }

        let inverted = mask & !value;
        Some((Bitset { bits: value }, Bitset { bits: inverted }))
    })
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let full_graph = parse_graph(input);
    let graph = compress_graph(&full_graph);

    // At the start, all rooms are active except the starting room,
    // which we already know has the highest ID.
    let start_room = graph.size - 1;
    let active_rooms = Bitset {
        bits: (1u32 << start_room) - 1,
    };

    backtrack(&graph, 30, start_room, active_rooms)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let full_graph = parse_graph(input);
    let graph = compress_graph(&full_graph);
    let start_room = graph.size - 1;

    // We'll handle some valves, and the elephant will handle others.
    // There'll never be any reason for both us and the elephant to visit the same room.
    // So, we'll generate every way to partition the set of active rooms into two subsets,
    // and find the most pressure releasable for each subset in the time limit.
    // The best result over all partitionings is our answer.

    // There are 15 active nodes, so there will be 2^14 distinct partitionings.
    // Better hope the backtracking code from part 1 is efficient!
    let mut best = 0;
    for (my_rooms, elephant_rooms) in partitions(start_room) {
        let my_best = backtrack(&graph, 26, start_room, my_rooms);
        let elephant_best = backtrack(&graph, 26, start_room, elephant_rooms);
        best = max(best, my_best + elephant_best);
    }

    best
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2022/test/day16.txt").expect("missing input");
        assert_eq!(part1(&input), 1651);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2022/test/day16.txt").expect("missing input");
        assert_eq!(part2(&input), 1707);
    }
}
