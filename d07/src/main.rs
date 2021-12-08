#![feature(int_abs_diff)]
use std::{fs::read_to_string, io, path::Path};

use itertools::Itertools;

fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<isize>> {
    read_to_string(path)?.trim().split(',').map(|n| Ok(n.parse().unwrap())).collect()
}

fn part1(data: &[isize]) -> usize {
    let (min, max) = data
        .iter()
        .minmax()
        .into_option()
        .unwrap();
    (*min..=*max)
        .map(|n| data.iter().map(|d| d.abs_diff(n)).sum())
        .min()
        .unwrap()
}

fn part2(data: &[isize]) -> usize {
    let (min, max) = data
        .iter()
        .minmax()
        .into_option()
        .unwrap();
    (*min..=*max)
        .map(|n| data.iter().map(|d| -> usize {(1..=d.abs_diff(n)).sum()}).sum())
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let data = parse_file("input.txt")?;
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1(&[16,1,2,0,4,2,7,1,2,14]), 37)
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(&[16,1,2,0,4,2,7,1,2,14]), 168)
    }
}
