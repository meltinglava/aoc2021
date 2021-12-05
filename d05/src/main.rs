#![feature(int_abs_diff)]
use std::{cmp::{Ordering, max}, collections::HashMap, io, path::Path};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: &str, y: &str) -> Result<Self, std::num::ParseIntError>
    {
        Ok(Self{
            x: x.parse()?,
            y: y.parse()?,
        })
    }

    fn straight_range(&self, p2: Point) -> Option<Vec<Point>> {
        if self.x == p2.x {
            let (min, max) = match self.y < p2.y {
                true => (self.y, p2.y),
                false => (p2.y, self.y),
            };
            Some((min..=max).map(|n| Point {x: self.x, y: n}).collect())
        } else if self.y == p2.y {
            let (min, max) = match self.x < p2.x {
                true => (self.x, p2.x),
                false => (p2.x, self.x),
            };
            Some((min..=max).map(|n| Point {x: n, y: self.y}).collect())
        } else {
            None
        }
    }

    fn delta(n1: usize, n2: usize) -> isize {
        match ((n2 as isize) - (n1 as isize)).cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    fn range(&self, p2: Point) -> Vec<Point> {
        let dx = Self::delta(self.x, p2.x);
        let dy = Self::delta(self.y, p2.y);
        (0isize..=((max(self.x.abs_diff(p2.x),self.y.abs_diff(p2.y))) as isize))
            .map(|n| Point {
                x: ((self.x as isize) + dx * n) as usize,
                y: ((self.y as isize) + dy * n) as usize,
            })
            .collect()
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<(Point, Point)>> {
    Ok(parse_lines(&std::fs::read_to_string(path)?))
}

fn parse_lines(file: &str) -> Vec<(Point, Point)> {
    file
        .lines()
        .map(|line| {
            let mut split = line
            .split("->")
            .map(|s| s.trim().split_once(',').unwrap())
            .map(|(x, y)| Point::new(x, y).unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

fn part1(lines: &[(Point, Point)]) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();
    lines
        .iter()
        .filter_map(|(p1, p2)| p1.straight_range(*p2))
        .flatten()
        .for_each(|p| *map.entry(p).or_default() += 1);
    map.values().filter(|&v| *v >= 2).count()
}

fn part2(lines: &[(Point, Point)]) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();
    lines
        .iter()
        .map(|(p1, p2)| p1.range(*p2))
        .flatten()
        .for_each(|p| *map.entry(p).or_default() += 1);
    map.values().filter(|&v| *v >= 2).count()
}


fn main() -> io::Result<()> {
    let lines = read_file("input.txt")?;
    println!("Part 1: {}", part1(&lines));
    println!("Part 1: {}", part2(&lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let lines = parse_lines(EXAMPLE);
        assert_eq!(part1(&lines), 5)
    }

    #[test]
    fn test_part2() {
        let lines = parse_lines(EXAMPLE);
        assert_eq!(part2(&lines), 12)
    }
}
