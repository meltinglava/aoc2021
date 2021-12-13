use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> (HashSet<[usize; 2]>, Vec<Instruction>) {
    let (points, instructions) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let instructions = instructions
        .lines()
        .map(|l| {
            let (direction, line) = l.trim_start_matches("fold along ").split_once('=').unwrap();
            match direction {
                "x" => Instruction::X(line.parse().unwrap()),
                "y" => Instruction::Y(line.parse().unwrap()),
                n => panic!("Unknown direction: {}", n),
            }
        })
        .collect();
    (points, instructions)
}

fn part1(mut data: HashSet<[usize; 2]>, instructions: &[Instruction]) -> usize {
    let instruction = instructions[0];
    data = data
        .into_iter()
        .map(|p| match instruction {
            Instruction::X(n) if p[0] > n => [n * 2 - p[0], p[1]],
            Instruction::Y(n) if p[1] > n => [p[0], n * 2 - p[1]],
            _ => p,
        })
        .collect();
    data.len()
}

fn part2(mut data: HashSet<[usize; 2]>, instructions: &[Instruction]) {
    for instruction in instructions {
        data = data
            .into_iter()
            .map(|p| match instruction {
                Instruction::X(n) if p[0] > *n => [n * 2 - p[0], p[1]],
                Instruction::Y(n) if p[1] > *n => [p[0], n * 2 - p[1]],
                _ => p,
            })
            .collect();
    }
    let shape = data
        .iter()
        .fold([0, 0], |p, n| [p[0].max(n[0]), p[1].max(n[1])]);
    let mut display = vec![vec![' '; shape[0] + 1]; shape[1] + 1];
    data.into_iter().for_each(|p| display[p[1]][p[0]] = '#');
    display
        .into_iter()
        .map(|n| n.into_iter().join(""))
        .for_each(|line| println!("{}", line));
}

fn main() {
    let (data, instructions) = parse_input(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(data.clone(), &instructions));
    part2(data, &instructions);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part1() {
        let (data, instructions) = parse_input(EXAMPLE);
        assert_eq!(part1(data.clone(), &instructions), 17);
    }
}
