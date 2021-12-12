use std::{collections::HashSet, fs::read_to_string};

use petgraph::graphmap::UnGraphMap;

type Graph<'a> = UnGraphMap<Node<'a>, ()>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    name: &'a str,
    size: Size,
}

impl<'a> Node<'a> {
    fn new (name: &'a str, size: Size) -> Self {
        Self { name, size }
    }

    fn start() -> Self {
        Self::new("start", Size::Small)
    }

    fn end() -> Self {
        Self::new("end", Size::Small)
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {

    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let size = match s {
            s if s.chars().all(|c| c.is_ascii_uppercase()) => Size::Large,
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Size::Small,
            n => return Err(format!("all letters need to be the same case, found: {}", n))
        };
        Ok(Self::new(s, size))
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Size {
    Large,
    Small,
}

fn parse_data(input: &str) -> Graph {
    let edges: Vec<_> = input.lines().map(|n| n.split_once('-').unwrap()).map(|n| {
        [n.0, n.1].map(|n| Node::try_from(n).unwrap())
    }).collect();
    let mut graph = UnGraphMap::with_capacity(edges.len()*2, edges.len());
    for edge in &edges {
        for node in edge {
            if !graph.contains_node(*node) {
                graph.add_node(*node);
            }
        }
        graph.add_edge(edge[0], edge[1], ());
    }
    graph
}

fn traverse_p1<'a, 'b>(node: Node<'a>, mut small: HashSet<Node<'a>>, graph: &Graph<'b>) -> usize
where 'a: 'b
{
    if node == Node::end() {
        return 1;
    }
    if node.size == Size::Small && !small.insert(node) {
        return 0
    }
    graph.neighbors(node).map(|n| traverse_p1(n, small.clone(), graph)).sum()
}

fn part1(graph: &Graph) -> usize {
    let small = HashSet::new();
    traverse_p1(Node::start(), small, graph)
}

fn traverse_p2<'a, 'b>(node: Node<'a>, mut small: HashSet<Node<'a>>, graph: &Graph<'b>, mut one_twice: bool) -> usize
where 'a: 'b
{
    if node == Node::end() {
        return 1;
    }
    if node.size == Size::Small && !small.insert(node) {
        if node == Node::start() || one_twice {
            return 0;
        } else {
            one_twice = true;
        }
    }
    graph.neighbors(node).map(|n| traverse_p2(n, small.clone(), graph, one_twice)).sum()
}

fn part2(graph: &Graph) -> usize {
    let small = HashSet::new();
    traverse_p2(Node::start(), small, graph, false)
}


fn main() {
    let input = read_to_string("input.txt").unwrap();
    let data = parse_data(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part1() {
        let data = parse_data(EXAMPLE);
        assert_eq!(part1(&data), 226);
    }

    #[test]
    fn test_part2() {
        let data = parse_data(EXAMPLE);
        assert_eq!(part2(&data), 3509);
    }
}
