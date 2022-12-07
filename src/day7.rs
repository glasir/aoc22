use std::collections::{HashMap, VecDeque};

pub enum Node {
    File(usize),
    Directory(HashMap<String, Node>),
}

impl Node {
    fn new_directory() -> Self {
        Self::Directory(HashMap::new())
    }

    fn new_file(size: usize) -> Self {
        Self::File(size)
    }

    fn size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Directory(contents) => contents.values().map(|elt| elt.size()).sum(),
        }
    }

    fn resolve(&mut self, path: &[String]) -> &mut Self {
        match path.get(0) {
            None => self,
            Some(component) => match self {
                Self::File(_) => panic!("cannot recurse into files"),
                Self::Directory(contents) => {
                    contents.get_mut(component).unwrap().resolve(&path[1..])
                }
            },
        }
    }

    fn iter(&self) -> NodeIterator<'_> {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        NodeIterator { queue }
    }
}

struct NodeIterator<'a> {
    // We're iterating over a tree. Do a breadth-first traversal.
    queue: VecDeque<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.queue.pop_front();
        if let Some(Node::Directory(children)) = next {
            self.queue.extend(children.values());
        }
        next
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> Node {
    let mut root = Node::new_directory();
    let mut path: Vec<String> = vec![];

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.by_ref().next() {
        let current = root.resolve(&path);

        // The current node should always be a directory.
        // Pull out its contents for use later.
        let children = match current {
            Node::Directory(children) => children,
            _ => panic!("current directory cannot be a file"),
        };

        // By construction, each line should start with a command.
        match &line[..4] {
            "$ cd" => {
                match &line[5..] {
                    ".." => {
                        path.pop();
                    }
                    "/" => {
                        path = vec![];
                    }
                    dir => path.push(String::from(dir)),
                };
            }
            "$ ls" => {
                loop {
                    // Loop until we find either the end of input, or another command
                    if lines.peek().map_or(true, |line| line.starts_with("$ ")) {
                        break;
                    }

                    match lines.next().unwrap().split_once(' ') {
                        Some(("dir", dir)) => {
                            children.insert(String::from(dir), Node::new_directory());
                        }
                        Some((size, file)) => {
                            children.insert(
                                String::from(file),
                                Node::new_file(size.parse::<usize>().unwrap()),
                            );
                        }
                        _ => panic!("unexpected ls entry"),
                    }
                }
            }
            other => panic!("unknown command: {other}"),
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn part1(root: &Node) -> usize {
    root.iter()
        .map(|node| match node {
            Node::File(_) => 0,
            Node::Directory(_) => {
                let size = node.size();
                if size < 100000 {
                    size
                } else {
                    0
                }
            }
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(root: &Node) -> usize {
    const TOTAL_SIZE: usize = 70000000;
    const TARGET_SIZE: usize = 30000000;

    // Do one pass to get the total size of all files on the system.
    let used_size: usize = root
        .iter()
        .map(|node| match node {
            Node::File(size) => *size,
            Node::Directory(_) => 0,
        })
        .sum();

    // We need to delete at least this much data
    let target_delete = TARGET_SIZE - (TOTAL_SIZE - used_size);

    root.iter()
        .map(|node| match node {
            Node::File(_) => 0,
            Node::Directory(_) => node.size(),
        })
        .filter(|size| *size >= target_delete)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const EXAMPLE: &str = "$ cd /\n\
                           $ ls\n\
                           dir a\n\
                           14848514 b.txt\n\
                           8504156 c.dat\n\
                           dir d\n\
                           $ cd a\n\
                           $ ls\n\
                           dir e\n\
                           29116 f\n\
                           2557 g\n\
                           62596 h.lst\n\
                           $ cd e\n\
                           $ ls\n\
                           584 i\n\
                           $ cd ..\n\
                           $ cd ..\n\
                           $ cd d\n\
                           $ ls\n\
                           4060174 j\n\
                           8033020 d.log\n\
                           5626152 d.ext\n\
                           7214296 k";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        assert_eq!(part2(&input), 24933642);
    }
}
