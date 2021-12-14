use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn parse(s: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = s.trim().lines();
    let start = lines.next().unwrap().chars().collect();

    let map = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(k, v)| {
            let mut k = k.chars();
            (
                (k.next().unwrap(), k.next().unwrap()),
                v.chars().next().unwrap(),
            )
        })
        .collect();
    (start, map)
}

fn partn(data: &[char], map: &HashMap<(char, char), char>, n: usize) -> usize {
    let mut datamap: HashMap<(char, char), usize> = HashMap::new();
    let mut value_map: HashMap<char, usize> = HashMap::new();
    for window in data.windows(2) {
        *datamap.entry((window[0], window[1])).or_default() += 1;
    }
    for c in data {
        *value_map.entry(*c).or_default() += 1;
    }
    for _ in 0..n {
        let mut new_map = HashMap::new();
        for (k, v) in datamap.into_iter().flat_map(|(k, v)| {
            let new = *map.get(&k).unwrap();
            *value_map.entry(new).or_default() += v;
            [((k.0, new), v), ((new, k.1), v)]
        }) {
            *new_map.entry(k).or_default() += v;
        }
        datamap = new_map;
    }
    value_map
        .values()
        .copied()
        .minmax()
        .into_option()
        .map(|n: (usize, usize)| n.1 - n.0)
        .unwrap()
}

fn part1(data: &[char], map: &HashMap<(char, char), char>) -> usize {
    partn(data, map, 10)
}

fn part2(data: &[char], map: &HashMap<(char, char), char>) -> usize {
    partn(data, map, 40)
}

fn main() {
    let (data, map) = parse(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(&data, &map));
    println!("Part 2: {}", part2(&data, &map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_parse() {
        parse(EXAMPLE);
    }

    #[test]
    fn test_part1() {
        let (data, map) = parse(EXAMPLE);
        assert_eq!(part1(&data, &map), 1588);
    }

    #[test]
    fn test_part2() {
        let (data, map) = parse(EXAMPLE);
        assert_eq!(part2(&data, &map), 2188189693529);
    }
}
