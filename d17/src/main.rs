static INPUT: &'static str = "target area: x=25..67, y=-260..-200";

use lazy_static::lazy_static;
use regex::Regex;

use std::{ops::{RangeInclusive, RangeFrom}, error::Error, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Area {
    x: RangeInclusive<usize>,
    y: RangeInclusive<isize>,
}

#[derive(Debug)]
struct TimesToPowers {
    set_times: HashMap<usize, HashSet<usize>>,
    minimums: HashMap<RangeFrom<usize>, usize>
}

impl TimesToPowers {
    fn setup(area: &Area) -> Self {
        let mut s = Self {
            set_times: HashMap::new(),
            minimums: HashMap::new(),
        };
        let min_power_x_to_reach_area = min_power_x_to_reach_area(area.x.clone().min().unwrap());
        let max_power_x_to_reach_area = area.x.clone().max().unwrap();

        for start_power in min_power_x_to_reach_area..=max_power_x_to_reach_area {
            let mut power = start_power;
            let mut distance = power;
            let mut time = 1;
            while distance <= max_power_x_to_reach_area {
                if power == 0 {
                    s.minimums.insert(time.., start_power);
                    break
                } else if area.x.contains(&distance) {
                    s.set_times.entry(time).or_insert_with(HashSet::new).insert(start_power);
                }
                time += 1;
                power -= 1;
                distance += power;
            }
        }
        s
    }

    fn contains(&self, time: usize) -> HashSet<usize> {
        let mut times = self.set_times.get(&time).cloned().unwrap_or_default();
        times.extend(self.minimums.iter().filter(|(rng, _)| rng.contains(&time)).map(|(_, v)| v));
        times
    }
}

fn parse(s: &str) -> Result<Area, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+)\.\.(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    }
    let captures = RE.captures(s).unwrap();
    Ok(Area {
        x: usize::from_str_radix(&captures[1], 10)?..=usize::from_str_radix(&captures[2], 10)?,
        y: isize::from_str_radix(&captures[3], 10)?..=isize::from_str_radix(&captures[4], 10)?
    })
}

fn part1(area: &Area) -> isize {
    (1..=(area.y.clone().min().unwrap().abs() - 1)).sum()
}

fn min_power_x_to_reach_area(x_start: usize) -> usize {
    let mut sum = 0;
    for i in 1.. {
        sum += i;
        if sum >=x_start {
            return i;
        }
    }
    unreachable!()
}

fn start_powers(area: &Area) -> HashSet<(usize, isize)> {
    let powers = TimesToPowers::setup(area);
    let min_y = area.y.clone().min().unwrap();
    let max_y_power = area.y.clone().min().unwrap().abs() - 1;

    let mut valid_powers = HashSet::new();

    for y_power_start in min_y..=max_y_power {
        let mut y = y_power_start;
        let mut y_power = y_power_start;
        let mut time = 1;
        let mut current_x_powers = HashSet::new();
        while y >= min_y {
            if area.y.contains(&y) {
                current_x_powers.extend(powers.contains(time))
            }
            y_power -= 1;
            y += y_power;
            time += 1
        }
        valid_powers.extend(current_x_powers.into_iter().map(|n| (n, y_power_start)))
    }
    valid_powers
}

fn part2(area: &Area) -> usize {
    start_powers(area).len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let area = parse(INPUT)?;
    println!("Part 1: {}", part1(&area));
    println!("Part 2: {}", part2(&area));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STRING: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse() {
        parse(TEST_STRING).unwrap();
    }

    #[test]
    fn test_part1() {
        let area = parse(TEST_STRING).unwrap();
        assert_eq!(part1(&area), 45)
    }

    #[test]
    fn test_part2() {
        let area = parse(TEST_STRING).unwrap();
        assert_eq!(part2(&area), 112)
    }

    #[test]
    fn test_start_powers() {
        let area = parse(TEST_STRING).unwrap();
        let start_powers = start_powers(&area);
        let expected_arr = [
            (23,-10),  (25,-9),   (27,-5),   (29,-6),   (22,-6),   (21,-7),   (9,0),     (27,-7),   (24,-5),
            (25,-7),   (26,-6),   (25,-5),   (6,8),     (11,-2),   (20,-5),   (29,-10),  (6,3),     (28,-7),
            (8,0),     (30,-6),   (29,-8),   (20,-10),  (6,7),     (6,4),     (6,1),     (14,-4),   (21,-6),
            (26,-10),  (7,-1),    (7,7),     (8,-1),    (21,-9),   (6,2),     (20,-7),   (30,-10),  (14,-3),
            (20,-8),   (13,-2),   (7,3),     (28,-8),   (29,-9),   (15,-3),   (22,-5),   (26,-8),   (25,-8),
            (25,-6),   (15,-4),   (9,-2),    (15,-2),   (12,-2),   (28,-9),   (12,-3),   (24,-6),   (23,-7),
            (25,-10),  (7,8),     (11,-3),   (26,-7),   (7,1),     (23,-9),   (6,0),     (22,-10),  (27,-6),
            (8,1),     (22,-8),   (13,-4),   (7,6),     (28,-6),   (11,-4),   (12,-4),   (26,-9),   (7,4),
            (24,-10),  (23,-8),   (30,-8),   (7,0),     (9,-1),    (10,-1),   (26,-5),   (22,-9),   (6,5),
            (7,5),     (23,-6),   (28,-10),  (10,-2),   (11,-1),   (20,-9),   (14,-2),   (29,-7),   (13,-3),
            (23,-5),   (24,-8),   (27,-9),   (30,-7),   (28,-5),   (21,-10),  (7,9),     (6,6),     (21,-5),
            (27,-10),  (7,2),     (30,-9),   (21,-8),   (22,-7),   (24,-9),   (20,-6),   (6,9),     (29,-5),
            (8,-2),    (27,-8),   (30,-5),   (24,-7)
        ];
        let arr_len = expected_arr.len();
        let expected = HashSet::from(expected_arr);
        assert_eq!(arr_len, expected.len());
        //let correct = start_powers.intersection(&expected);
        let false_positives = start_powers.difference(&expected).collect::<HashSet<_>>();
        let false_negatives = expected.difference(&expected).collect::<HashSet<_>>();
        assert_eq!(expected.len(), start_powers.len());
        assert!(false_positives.len() == 0 && false_negatives.len() == 0, "False Positives: {:?},\nFalse Negatives: {:?}", false_positives, false_negatives);
    }
}
