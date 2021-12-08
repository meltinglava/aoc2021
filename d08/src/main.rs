use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

type Segment = [bool; 7];

// const SEGMENTS: [Segment; 10] = [
//     //  aaaaaa, bbbbb, ccccc, ddddd, eeeee, fffff, ggggg
//     [ true,  true,  true, false,  true,  true,  true],
//     [false, false,  true, false, false,  true, false],
//     [ true, false,  true,  true,  true, false,  true],
//     [ true, false,  true,  true, false,  true,  true],
//     [false,  true,  true,  true, false,  true, false],
//     [ true,  true, false,  true, false,  true,  true],
//     [ true,  true, false,  true,  true,  true,  true],
//     [ true, false,  true, false, false,  true, false],
//     [ true,  true,  true,  true,  true,  true,  true],
//     [ true,  true,  true,  true, false,  true,  true],
// ];

fn segment(s: &str) -> Segment {
    let mut seg = [false; 7];
    s.chars()
        .map(|c| match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            n => unreachable!("Unknown segment letter: {}", n),
        })
        .for_each(|s| seg[s] = true);
    seg
}

#[derive(Debug)]
struct Segments {
    input: [Segment; 10],
    output: [Segment; 4],
}

impl Segments {
    fn get_lens(&self, s: usize) -> Vec<Segment> {
        if s > 7 && s < 2 {
            panic!("Segments are from number 0 to 9 inclusive")
        }
        self.input
            .iter()
            .filter(|n| segment_len(n) == s)
            .copied()
            .collect()
    }

    // [6, 2, 5, 5, 4, 5, 6, 3, 7, 6]
    // |0, 1, 2, 3, 4, 5, 6, 7, 8, 9|
    fn create_matching(&self) -> HashMap<Segment, usize> {
        let mut map = HashMap::new();
        // one
        let seg_one = self.get_lens(2)[0];
        map.insert(seg_one, 1);
        let one = seg_pos_to_numbers(&seg_one);
        // seven
        let seg_seven = self.get_lens(3)[0];
        map.insert(seg_seven, 7);
        // six
        let mut six_len = self.get_lens(6);
        let seg_six = six_len.remove(
            six_len
                .iter()
                .map(seg_pos_to_numbers)
                .enumerate()
                .filter(|(_, s)| !one.is_subset(s))
                .map(|(n, _)| n)
                .next()
                .unwrap(),
        );
        map.insert(seg_six, 6);
        let segment_c = *(0..7)
            .collect::<HashSet<_>>()
            .difference(&seg_pos_to_numbers(&seg_six))
            .next()
            .unwrap();
        // five
        let mut five_len = self.get_lens(5);
        let seg_five = five_len.remove(
            five_len
                .iter()
                .map(seg_pos_to_numbers)
                .enumerate()
                .filter(|(_, s)| !s.contains(&segment_c))
                .map(|(n, _)| n)
                .next()
                .unwrap(),
        );
        map.insert(seg_five, 5);
        let mut seg_five_with_c = seg_pos_to_numbers(&seg_five);
        seg_five_with_c.insert(segment_c);
        let segment_e = *(0..7)
            .collect::<HashSet<_>>()
            .difference(&seg_five_with_c)
            .next()
            .unwrap();
        // nine
        let seg_nine = six_len.remove(
            six_len
                .iter()
                .map(seg_pos_to_numbers)
                .enumerate()
                .filter(|(_, s)| !s.contains(&segment_e))
                .map(|(n, _)| n)
                .next()
                .unwrap(),
        );
        map.insert(seg_nine, 9);
        // zero
        let seg_zero = six_len.remove(0);
        map.insert(seg_zero, 0);
        // eight
        map.insert(self.get_lens(7)[0], 8);
        // four
        map.insert(self.get_lens(4)[0], 4);
        // three
        let seg_three = five_len.remove(
            five_len
                .iter()
                .map(seg_pos_to_numbers)
                .enumerate()
                .filter(|(_, s)| !s.contains(&segment_e))
                .map(|(n, _)| n)
                .next()
                .unwrap(),
        );
        map.insert(seg_three, 3);
        // two
        map.insert(five_len[0], 2);
        map
    }

    fn get_number(&self) -> usize {
        let matching = self.create_matching();
        let mut ans = 0;
        self.output.iter().for_each(|o| {
            ans *= 10;
            ans += matching.get(o).unwrap();
        });
        ans
    }
}

impl FromStr for Segments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .trim()
            .split_once('|')
            .ok_or_else(|| "Missing `|` symbol".to_string())?;
        let ans = [split.0, split.1]
            .into_iter()
            .map(|s| s.trim().split(' ').map(segment).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self {
            input: ans[0].clone().try_into().unwrap(),
            output: ans[1].clone().try_into().unwrap(),
        })
    }
}

fn parse_text(text: &str) -> Vec<Segments> {
    text.lines()
        .map(Segments::from_str)
        .map(|s| s.unwrap())
        .collect()
}

fn segment_len(seg: &Segment) -> usize {
    seg.iter().filter(|&n| *n).count()
}

fn unique(seg: &Segment) -> bool {
    let unique_lens = [2, 4, 3, 7];
    let len = segment_len(seg);
    unique_lens.contains(&len)
}

fn part1(data: &[Segments]) -> usize {
    data.iter()
        .map(|s| s.output.iter().map(unique).filter(|n| *n).count())
        .sum()
}

fn seg_pos_to_numbers(seg: &Segment) -> HashSet<usize> {
    seg.iter()
        .copied()
        .enumerate()
        .filter(|(_, s)| *s)
        .map(|(n, _)| n)
        .collect()
}

fn part2(data: &[Segments]) -> usize {
    data.iter().map(|s| s.get_number()).sum()
}

fn main() {
    let data = parse_text(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    //println!("{:?}", SEGMENTS.iter().map(|n| n.iter().filter(|&n| *n).count()).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_first() {
        let data = parse_text(EXAMPLE);
        assert_eq!(part1(&data), 26)
    }

    #[test]
    fn test_part2() {
        let data = parse_text(EXAMPLE);
        assert_eq!(part2(&data), 61229)
    }

    #[test]
    fn test_second() {
        let data = parse_text(EXAMPLE);
        let ans = [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
        data.iter()
            .map(Segments::get_number)
            .zip(ans)
            .enumerate()
            .for_each(|(n, (actual, expected))| {
                assert_eq!(actual, expected, "number {} is wrong", n);
            })
    }
}
