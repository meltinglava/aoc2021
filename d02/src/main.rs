use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, len) = s.trim().split_once(" ").ok_or("Malformated")?;
        let len: usize = len.parse().map_err(|_| "Not a number")?;
        match direction {
            "forward" => Ok(Self::Forward(len)),
            "up" => Ok(Self::Up(len)),
            "down" => Ok(Self::Down(len)),
            n => {
                dbg!("this is strange: {}", n);
                Err("Unknown direction")
            }
        }
    }
}

fn parse_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<Command>> {
    let reader = BufReader::new(File::open(path)?);
    reader.lines().map(|l| Ok(l?.parse().unwrap())).collect()
}

fn part1(input: &[Command]) {
    let mut horizontal = 0;
    let mut vertical = 0;
    input.iter().for_each(|n| match n {
        Command::Up(n) => vertical -= n,
        Command::Down(n) => vertical += n,
        Command::Forward(n) => horizontal += n,
    });
    println!("Part 1: {}", horizontal * vertical);
}

fn part2(input: &[Command]) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    input.iter().for_each(|n| match n {
        Command::Up(n) => aim -= n,
        Command::Down(n) => aim += n,
        Command::Forward(n) => {
            horizontal += n;
            vertical += aim * n
        }
    });
    println!("Part 2: {}", horizontal * vertical);
}

fn main() -> io::Result<()> {
    let input = parse_input("input.txt")?;
    part1(&input);
    part2(&input);
    Ok(())
}
