use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
struct Ecosystem {
    fish: [usize; 9],
    start: bool,
}

impl Ecosystem {
    fn new(fish: &[usize]) -> Self {
        let mut map = [0; 9];
        for f in fish {
            map[*f] += 1;
        }
        Self {
            fish: map,
            start: true,
        }
    }
}

impl Iterator for Ecosystem {
    type Item = [usize; 9];

    fn next(&mut self) -> Option<Self::Item> {
        match self.start {
            true => {
                self.start = false;
            }
            false => {
                self.fish.rotate_left(1);
                self.fish[6] += self.fish[8];
            }
        };
        Some(self.fish)
    }
}

impl FromStr for Ecosystem {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            &s.trim()
                .split(',')
                .map(|p| p.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn main() {
    let mut fish: Ecosystem = read_to_string("input.txt").unwrap().parse().unwrap();
    println!(
        "Part 1: {}",
        fish.clone().nth(80).unwrap().iter().sum::<usize>()
    );
    println!("Part 2: {}", fish.nth(256).unwrap().iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &[&[usize]] = &[
        &[3, 4, 3, 1, 2],
        &[2, 3, 2, 0, 1],
        &[1, 2, 1, 6, 0, 8],
        &[0, 1, 0, 5, 6, 7, 8],
        &[6, 0, 6, 4, 5, 6, 7, 8, 8],
        &[5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
        &[4, 5, 4, 2, 3, 4, 5, 6, 6, 7],
        &[3, 4, 3, 1, 2, 3, 4, 5, 5, 6],
        &[2, 3, 2, 0, 1, 2, 3, 4, 4, 5],
        &[1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8],
        &[0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8],
        &[6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8],
        &[5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],
        &[4, 5, 4, 2, 3, 4, 5, 6, 6, 0, 4, 5, 6, 6, 6, 7, 7, 8, 8],
        &[3, 4, 3, 1, 2, 3, 4, 5, 5, 6, 3, 4, 5, 5, 5, 6, 6, 7, 7, 8],
        &[2, 3, 2, 0, 1, 2, 3, 4, 4, 5, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7],
        &[
            1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
        ],
        &[
            0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
        ],
        &[
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ],
    ];

    #[test]
    fn days() {
        let fish = Ecosystem::new(TEST[0]);
        fish.zip(TEST.iter())
            .for_each(|(f, t)| assert_eq!(f, Ecosystem::new(t).fish));
    }

    #[test]
    fn long_test() {
        let mut fish = Ecosystem::new(TEST[0]);
        assert_eq!(fish.nth(256).unwrap().iter().sum::<usize>(), 26984457539)
    }
}
