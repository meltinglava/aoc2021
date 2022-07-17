use std::{error::Error, str::FromStr, fmt::{Display, Debug}, ops::{Not, Add, AddAssign}, iter::{Sum, repeat_with}};

use slice_group_by::StrGroupBy;

#[derive(Clone, PartialEq, Eq)]
enum SFValue {
    Number(usize),
    Sf(Box<SFValue>, Box<SFValue>)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Side {
    Left,
    Right
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Right => Side::Left,
            Side::Left => Side::Right,
        }
    }
}

impl SFValue {
    fn magnitude(&self) -> usize {
        match self {
            SFValue::Number(n) => *n,
            SFValue::Sf(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn find_nested_inside_for_parirs(&self, mut path: Vec<Side>) -> Option<Vec<Side>> {
        match self {
            SFValue::Number(_) => {
                match path.len() {
                    0 | 1 | 2 | 3 | 4 => None,
                    5.. => Some(path),
                    _ => unreachable!()
                }
            },
            SFValue::Sf(left, right) => {
                let mut l = path.clone();
                l.push(Side::Left);
                let ans = left.find_nested_inside_for_parirs(l);
                path.push(Side::Right);
                ans.or(right.find_nested_inside_for_parirs(path))
            }
        }
    }

    fn number_greater_than_10(&self, mut path: Vec<Side>) -> Option<Vec<Side>> {
        match self {
            SFValue::Number(n) => if *n >= 10 {
                Some(path)
            } else { None },
            SFValue::Sf(left, right) => {
                let mut l = path.clone();
                l.push(Side::Left);
                let ans = left.number_greater_than_10(l);
                path.push(Side::Right);
                ans.or(right.number_greater_than_10(path))
            }
        }
    }

    fn partial_parse<T>(symbols: &mut T) -> Self
    where T: Iterator<Item=String> {
        let left = match symbols.next().unwrap().as_str() {
            "[" => Self::partial_parse(symbols),
            n => {
                return SFValue::Number(n.parse().unwrap());
            }
        };
        symbols.next().as_deref();
        let right = Self::partial_parse(symbols);
        symbols.next().as_deref();
        Self::Sf(Box::new(left), Box::new(right))
    }
}

impl Debug for SFValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFValue::Number(n) => write!(f, "{n}"),
            SFValue::Sf(l, r) => write!(f, "[{l},{r}]"),
        }
    }
}

impl Display for SFValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFValue::Number(n) => write!(f, "{n}"),
            SFValue::Sf(l, r) => write!(f, "[{l},{r}]"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct SnailFish {
    sf: SFValue,
}

impl SnailFish {
    fn magnitude(&self) -> usize {
        self.sf.magnitude()
    }

    fn reduce(&mut self) {
        'main: loop {
            while let Some(path) = self.sf.find_nested_inside_for_parirs(Vec::new()) {
                self.explode(&path);
            }
            if let Some(path) = self.sf.number_greater_than_10(Vec::new()) {
                self.split(&path);
                continue 'main;
            }
            break;
        }
    }

    fn split(&mut self, path: &[Side]) {
        let node = self.index_mut(path);
        let n = match node {
            SFValue::Sf(_, _) => unreachable!("On split we got a Sf, not a number"),
            SFValue::Number(n) => {
                n.clone()
            },
        };
        *node = SFValue::Sf(Box::new(SFValue::Number(n/2)), Box::new(SFValue::Number(n/2 + n%2)));
    }

    fn explode(&mut self, path: &[Side]) {
        let (left_value, right_value): (usize, usize) = match self.index(&path[..4]) {
            SFValue::Number(_) => unreachable!("expected Sf, found number"),
            SFValue::Sf(l, r) => {
                (match **l {
                    SFValue::Number(n) => n,
                    SFValue::Sf(_, _) => unreachable!("expected number, found Sf"),
                }, match **r {
                    SFValue::Number(n) => n,
                    SFValue::Sf(_, _) => unreachable!("expected number, found Sf"),
                })
            },
        };
        let base_path = &path[..4];
        for (v, s) in [
            (left_value, Side::Left),
            (right_value, Side::Right)
        ] {
            let mut find_path = base_path.to_vec();
            find_path.push(s);
            match self.find(&find_path, s) {
                Some(p) => match self.index_mut(&p) {
                    SFValue::Number(ref mut n) => *n += v,
                    SFValue::Sf(_, _) => unreachable!("exppected number, found Sf"),
                },
                None => (),
            }
        }
        *self.index_mut(&path[..4]) = SFValue::Number(0);
    }

    fn index(&self, path: &[Side]) -> &SFValue {
        let mut current = &self.sf;
        for p in path {
            match current {
                SFValue::Number(_) => (),
                SFValue::Sf(l, r) => {
                    current = match p {
                        Side::Left => l,
                        Side::Right => r,
                    }
                },
            }
        }
        current
    }

    fn index_mut(&mut self, path: &[Side]) -> &mut SFValue {
        let mut current = &mut self.sf;
        for p in path {
            match current {
                SFValue::Number(_) => (),
                SFValue::Sf(l, r) => {
                    current = match p {
                        Side::Left => l,
                        Side::Right => r,
                    }
                },
            }
        }
        current
    }


    fn find(&self, path: &[Side], side: Side) -> Option<Vec<Side>> {
        let mut sides = path.to_vec();
        let mut len = 0;
        let mut edits = false;
        for (n, s) in sides.iter_mut().rev().enumerate() {
            if *s != side {
                *s = side;
                len = n;
                edits = true;
                break;
            }
        }
        match edits {
            true => {
                sides.iter_mut().skip(path.len() - len).for_each(|s| *s = !side);
                sides.push(!side);
                (0..sides.len()).find(|&n| match self.index(&sides[0..n]) {
                    SFValue::Number(_) => true,
                    SFValue::Sf(_, _) => false,
                }).map(|n| sides[0..n].to_vec())
            }
            false => None,
        }

    }
}

impl Debug for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Add for SnailFish {
    type Output = SnailFish;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ans = Self {
            sf: SFValue::Sf(Box::new(self.sf), Box::new(rhs.sf)),
        };
        ans.reduce();
        ans
    }
}

impl AddAssign for SnailFish {
    fn add_assign(&mut self, rhs: Self) {
        let mut temp = SFValue::Number(1);
        std::mem::swap(&mut self.sf, &mut temp);
        self.sf = SFValue::Sf(Box::new(temp), Box::new(rhs.sf));
        self.reduce();
    }
}

impl Sum for SnailFish {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut a = iter.next().unwrap();
        for i in iter {
            a += i;
        }
        a
    }
}

impl Display for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&&self.sf, f)
    }
}

impl FromStr for SnailFish {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = s.linear_group_by_key(char::is_alphanumeric).flat_map(|s| -> Box<dyn Iterator<Item=String>> {
            match s.chars().all(char::is_alphanumeric) {
                true => Box::new(std::iter::once(s.to_string())),
                false => Box::new(s.chars().map(|c| c.to_string())),
            }
        });
        Ok(Self {
            sf: SFValue::partial_parse(&mut symbols),
        })
    }
}

fn parse_input(s: &str) -> Result<Vec<SnailFish>, Box<dyn Error>> {
    s
        .lines()
        .map(SnailFish::from_str)
        .collect()
}


fn part1(sf: &[SnailFish]) -> usize {
    sf.to_vec().into_iter().sum::<SnailFish>().magnitude()
}

fn part2(sf: &[SnailFish]) -> usize {
    sf.iter().flat_map(|f| repeat_with(|| f.clone()).zip(sf.iter().cloned())).map(|(x, y)| x + y).map(|sf| sf.magnitude()).max().unwrap()
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = include_str!("../input.txt");
    let sf = parse_input(input)?;
    println!("Part1: {}", part1(&sf));
    println!("Part2: {}", part2(&sf));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[&'static str] = &[
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
        "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
    ];
    const EXAMPLE_2: &[&'static str] = &[
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]"
    ];

    const EXAMPLE_3: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    fn parse_for_test(s: &'static str) -> SnailFish {
        s.parse().unwrap()
    }

    #[test]
    fn test_sum_ans() {
        let s: SnailFish = EXAMPLE_2.iter().map(|&s| parse_for_test(s)).sum();
        assert_eq!(s, parse_for_test("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"))
    }

    #[test]
    fn test_parse_and_print() {
        for n in EXAMPLE_1 {
            assert_eq!(*n, &format!("{}", n.parse::<SnailFish>().unwrap()))
        }
    }

    #[test]
    fn test_homework() {
        assert_eq!(part1(&parse_input(EXAMPLE_3).unwrap()), 4140)
    }

    #[test]
    fn test_homework_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE_3).unwrap()), 3993)
    }

    #[test]
    fn test_reduce() {
        assert_eq!(parse_for_test("[[[[4,3],4],4],[7,[[8,4],9]]]") + parse_for_test("[1,1]"), parse_for_test("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }
}
