use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Default)]
struct OctoGrid {
    grid: [[usize; 10]; 10],
}

impl OctoGrid {
    #[allow(unused)]
    fn debug_print(&self) {
        println!("Grid:");
        for n in self.grid {
            for s in n {
                print!("{}, ", s);
            }
            println!();
        }
    }
}

fn range(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (x.saturating_sub(1)..=9.min(x + 1))
        .flat_map(move |n| (y.saturating_sub(1)..=9.min(y + 1)).map(move |o| (n, o)))
        .filter(move |(n, o)| x != *n || y != *o)
}

impl Iterator for OctoGrid {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid
            .iter_mut()
            .flat_map(|l| l.iter_mut())
            .for_each(|n| *n += 1);
        let mut flashes = HashMap::new();
        let mut last_len = usize::MAX;
        let mut len = 0;
        while last_len != len {
            last_len = len;
            for (y, row) in self.grid.iter().enumerate() {
                for (x, value) in row.iter().enumerate() {
                    if *value > 9 {
                        flashes.entry((x, y)).or_insert_with(|| range(x, y));
                    }
                }
            }
            for i in flashes.values_mut() {
                for (x, y) in i {
                    self.grid[y][x] += 1
                }
            }
            len = self
                .grid
                .iter()
                .flat_map(|l| l.iter())
                .filter(|&n| *n > 9)
                .count();
        }
        self.grid
            .iter_mut()
            .flat_map(|l| l.iter_mut())
            .filter(|n| **n > 9)
            .for_each(|n| *n = 0);
        Some(len)
    }
}

fn part1(grid: OctoGrid) -> usize {
    grid.take(100).sum()
}

fn part2(grid: OctoGrid) -> usize {
    grid.take_while(|n| *n < 100).count() + 1
}

fn parse_input(input: &str) -> OctoGrid {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[_; 10]>>()
        .try_into()
        .unwrap();
    OctoGrid { grid }
}

fn main() {
    let data = parse_input(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(data.clone()));
    println!("Part 2: {}", part2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[&'static str] = &[
        "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
",
        "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
",
        "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
",
        "\
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000
",
        "\
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211
",
        "\
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322
",
        "\
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433
",
        "\
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644
",
        "\
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755
",
        "\
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976
",
        "\
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
",
    ];
    #[test]
    fn test_interator() {
        let mut data = parse_input(EXAMPLES[0]);
        for example in EXAMPLES.into_iter().skip(1) {
            let expected = example.chars().filter(|c| *c == '0').count();
            let actual = data.next().unwrap();
            parse_input(*example).debug_print();
            data.debug_print();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLES[0]);
        assert_eq!(part1(data.clone()), 1656)
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLES[0]);
        assert_eq!(part2(data.clone()), 195)
    }

    #[test]
    fn test_range() {
        assert_eq!(range(0, 2).count(), 5);
    }
}
