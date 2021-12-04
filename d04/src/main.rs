use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Bingo {
    board: Vec<usize>,
}

impl Bingo {
    fn new(v: Vec<usize>) -> Self {
        Self { board: v }
    }

    fn bingo(&self, drawn: &HashSet<usize>, last: usize) -> Result<usize, usize> {
        let horizontal = self
            .board
            .chunks(5)
            .into_iter()
            .map(|n| n.iter().filter(|n| drawn.contains(n)).count())
            .enumerate()
            .max_by_key(|(_, k)| *k)
            .unwrap();
        let vertical = (0..5)
            .map(|n| {
                self.board
                    .iter()
                    .skip(n)
                    .step_by(5)
                    .filter(|d| drawn.contains(d))
                    .count()
            })
            .enumerate()
            .max_by_key(|(_, k)| *k)
            .unwrap();
        match max(vertical.1, horizontal.1) {
            5 => Ok(self
                .board
                .iter()
                .copied()
                .filter(|d| !drawn.contains(d))
                .sum::<usize>()
                * last),
            n if (0..5).contains(&n) => Err(n),
            _ => unreachable!("boards with odd size"),
        }
    }
}

fn read_bingoes<P: AsRef<Path>>(path: P) -> io::Result<(Vec<usize>, Vec<Bingo>)> {
    let f = BufReader::new(File::open(path)?);
    let mut lines = f.lines();
    let numbers = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();
    let bingoes = lines
        .map(Result::unwrap)
        .filter(|l| !l.trim().is_empty())
        .chunks(5)
        .into_iter()
        .map(|c| {
            c.flat_map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec()
        })
        .map(Bingo::new)
        .collect();
    Ok((numbers, bingoes))
}

fn part1(numbers: &[usize], boards: &[Bingo]) -> usize {
    let mut n_to_draw = 5;
    let mut drawn = HashSet::<usize>::new();
    let mut to_draw = numbers.iter().copied();
    let mut last = 0;
    loop {
        drawn.extend(
            std::iter::from_fn(|| {
                last = to_draw.next()?;
                Some(last)
            })
            .take(n_to_draw),
        );
        let res = boards
            .iter()
            .map(|b| b.bingo(&drawn, last))
            .fold(Err(0), |p, v| match p {
                Ok(p) => Ok(p),
                Err(n) => match v {
                    Ok(n) => Ok(n),
                    Err(v) => Err(max(n, v)),
                },
            });
        match res {
            Ok(n) => break n,
            Err(n) => n_to_draw = 5 - n,
        }
    }
}

fn part2(numbers: &[usize], boards: &[Bingo]) -> usize {
    let mut n_to_draw = 5;
    let mut drawn = HashSet::<usize>::new();
    let mut to_draw = numbers.iter().copied();
    let mut last = 0;
    let mut boards = boards.to_vec();
    loop {
        drawn.extend(
            std::iter::from_fn(|| {
                last = to_draw.next()?;
                Some(last)
            })
            .take(n_to_draw),
        );
        let res = boards.iter().map(|b| b.bingo(&drawn, last)).collect_vec();
        match boards.len() {
            0 => unreachable!(),
            1 => match res[0] {
                Ok(e) => break e,
                Err(n) => n_to_draw = 5 - n,
            },
            _ => {
                boards = boards
                    .into_iter()
                    .zip(res)
                    .filter(|(_, r)| match r {
                        Ok(_) => false,
                        Err(e) => {
                            n_to_draw = min(n_to_draw, 5 - e);
                            true
                        }
                    })
                    .map(|(v, _)| v)
                    .collect()
            }
        }
    }
}

fn main() -> io::Result<()> {
    let (numbers, bingo) = read_bingoes("input.txt")?;
    let p1 = part1(&numbers, &bingo);
    println!("Part1: {}", p1);
    let p2 = part2(&numbers, &bingo);
    println!("Part2: {}", p2);
    Ok(())
}
