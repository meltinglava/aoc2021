use slice_group_by::StrGroupBy;
use std::{fmt::Write, ops::Add, iter::Sum, fmt::Debug, cmp::Ordering::{Less, Equal, Greater}};

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnailFishNumber {
    value: usize,
    height: usize,
}

impl SnailFishNumber {
    fn new(value: usize, height: usize) -> Self {
        Self {
            value,
            height,
        }
    }

    fn split(self) -> (Self, Self) {
        if self.value < 10 {
            panic!("You tried to split a number that is to small");
        }
        let left_over = self.value % 2;
        let core = self.value / 2;
        (Self::new(core, self.height + 1), Self::new(core + left_over, self.height + 1))
    }

}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct SnailFish {
    fishes: Vec<SnailFishNumber>
}

impl SnailFish {
    fn print(&self) -> String {
        let mut indent = 0;
        let mut stack = Vec::new();
        let mut s = String::new();
        for fish in &self.fishes {
            let diff = fish.height.abs_diff(indent);
            match fish.height.cmp(&indent) {
                Greater => {
                    write!(s, "{}{}", "[".repeat(diff), fish.value).unwrap();
                    (indent..fish.height).for_each(|n| stack.push(n));
                    indent = fish.height;
                },
                Less => {
                    write!(s, "{}{},", "]".repeat(diff), fish.value).unwrap();
                    (fish.height..indent).rev().for_each(|n| stack.push(n));
                    indent = fish.height;
                },
                Equal => {
                    match stack.last().unwrap().1 {
                        true => {
                            write!(s, ",{}", fish.value).unwrap();
                            stack.push(fish.height);
                        },
                        false => {
                            write!(s, "],[{}", fish.value).unwrap();
                            stack.push(fish.height);
                        },
                    }
                },
            }
        }

        todo!()
    }


    pub fn reduce(&mut self) {
        let base = 0..self.fishes.len();
        let iter = &mut base.clone();
        for i in iter.next() {
            if self.fishes[i].height > 4 {
                // left
                let left_value = self.fishes[i].value;
                match i.checked_sub(1).map(|n| self.fishes[n].value += left_value) {
                    Some(_) => {
                        self.fishes.remove(i);
                    },
                    None => {
                        let mut s = &mut self.fishes[i];
                        s.value = 0;
                        s.height -= 1;
                    },
                }
                // right
                let i = i + 1;
                let right_value = self.fishes[i].value;
                match self.fishes.get_mut(i + 1).map(|mut f| f.value += right_value) {
                    Some(_) => {
                        self.fishes.remove(i);
                    },
                    None => {
                        let mut s = &mut self.fishes[i];
                        s.value = 0;
                        s.height -= 1;
                    },
                }
            } else if self.fishes[i].value >= 10 {
                let (left, right) = self.fishes.remove(i).split();
                self.fishes[i] = left;
                self.fishes.insert(i + 1, right)
            } else {
                continue;
            }
            *iter = base.clone();
        }
    }

    fn magnitude(self) -> usize {
        let mut stack = Vec::with_capacity(4);
        let mut values = self.fishes.into_iter();
        stack.push(values.next().unwrap());
        for mut v in values {
             while v.height != 0 && v.height == stack.last().unwrap().height {
                v.height -= 1;
                v.value *= 2;
                v.value += stack.pop().unwrap().value * 3;
            }
            stack.push(v)
        }
        stack[0].value
    }

}

impl Add for SnailFish {
    type Output = SnailFish;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.fishes.extend(rhs.fishes);
        self.fishes.iter_mut().for_each(|mut v| v.height += 1);
        self.reduce();
        self
    }
}

impl Sum for SnailFish {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sf = SnailFish::default();
        for i in iter {
            sf = sf + i;
        }
        sf
    }
}

fn parse_number(s: &str) -> SnailFish {
    let mut snailfish = SnailFish::default();
    let mut level = 0;
    for s in s.linear_group_by_key(char::is_alphanumeric) {
        match s.chars().all(|c| c.is_alphanumeric()) {
            true => snailfish.fishes.push(SnailFishNumber{value: s.parse().unwrap(), height: level }),
            false => for c in s.chars() {
                match c {
                    '[' => level += 1,
                    ',' => continue,
                    ']' => level -= 1,
                    _ => panic!("Unknown char: {c}")
                }
            },
        }
    }
    snailfish
}


// fn parse_number(s: &mut&str) -> SnailfishNumber {
//     debug_assert_eq!(s.chars().next(), Some('['));
//     *s = dbg!(&s[1..]);
//     let left = match s.chars().next().unwrap() {
//         '[' => Either::SnailFish(Box::new(parse_number(s))),
//         c if c.is_ascii_alphanumeric() => {
//             *s = dbg!(&s[1..]);
//             Either::Number(c.to_digit(10).unwrap() as usize)
//         },
//         c => unreachable!("Expected number, found {c}")
//     };{}
//     debug_assert_eq!(s.chars().next(), Some(','));
//     *s = dbg!(&s[1..]);
//     let right = match s.chars().next().unwrap() {
//         '[' => Either::SnailFish(Box::new(parse_number(s))),
//         c if c.is_ascii_alphanumeric() => {
//             *s = dbg!(&s[1..]);
//             Either::Number(c.to_digit(10).unwrap() as usize)
//         },
//         c => unreachable!("Expected number, found {c}")
//     };
//     debug_assert_eq!(s.chars().next(), Some(']'));
//     *s = dbg!(&s[1..]);
//     SnailfishNumber { left, right }
// }

fn parse_all(input: &str) -> Vec<SnailFish> {
    input.trim().lines().map(|l| parse_number(&l)).collect()
}

fn part1(input: &[SnailFish]) -> usize {
    input.iter().cloned().sum::<SnailFish>().magnitude()
}

fn main() {
    let input = include_str!("../input.txt");
    let all = parse_all(input);
    println!("Part1: {}", part1(&all));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NUMBERS: [&'static str; 7] = [
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
        "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
    ];

    #[test]
    fn test_parse_numbers() {
        for t in TEST_NUMBERS {
            parse_number(t);
        }
    }

    static TEST_REDUCE: [(&'static str, &'static str); 5] = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    ];

    #[test]
    fn test_reduce_numbers() {
        for (original, reduced) in TEST_REDUCE {
            let mut original = parse_number(original);
            original.reduce();
            assert_eq!(original, parse_number(reduced), );
        }
    }

    static TEST_ADD: (&'static str, &'static str, &'static str) = ("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    #[test]
    fn test_add_numbers() {
        assert_eq!(parse_number(TEST_ADD.0) + parse_number(TEST_ADD.1), parse_number(TEST_ADD.2))
    }

    static TEST_ALL: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_all(TEST_ALL)), 4140)
    }
}
