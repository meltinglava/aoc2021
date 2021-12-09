use std::{cell::RefCell, collections::HashSet, fs::read_to_string};

fn parse_grid(file: &str) -> Vec<Vec<usize>> {
    file.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .map(|d| d as usize)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(data: &[Vec<usize>]) -> usize {
    let mut bottoms = Vec::new();
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let x_row = [x.checked_sub(1), Some(x + 1)]
                .into_iter()
                .map(|x| x.and_then(|n| row.get(n)))
                .flatten()
                .all(|n| value < n);
            let y_column = [y.checked_sub(1), Some(y + 1)]
                .into_iter()
                .map(|y| y.and_then(|n| data.get(n).map(|r| r[x])))
                .flatten()
                .all(|n| *value < n);
            if x_row && y_column {
                bottoms.push(value);
            }
        }
    }

    bottoms.into_iter().map(|n| n + 1).sum()
}

fn basin(data: &[Vec<usize>], x_orig: usize, y_orig: usize, searched: &RefCell<HashSet<(usize, usize)>>) -> usize {
    searched.borrow_mut().insert((x_orig, y_orig));
    let x_row: usize = [x_orig.checked_sub(1), Some(x_orig + 1)]
        .into_iter()
        .filter(|x| x.and_then(|n| data[y_orig].get(n)).is_some())
        .flatten()
        .filter_map(|x| {
            let mut s = searched.borrow_mut();
            let a = s.insert((x, y_orig));
            drop(s);
            match a {
                true => Some(basin(data, x, y_orig, searched)),
                false => None,
            }
        })
        .sum();
    let y_column: usize = [y_orig.checked_sub(1), Some(y_orig + 1)]
        .into_iter()
        .filter(|y| y.and_then(|n| data.get(n)).is_some())
        .flatten()
        .filter_map(|y| {
            let mut s = searched.borrow_mut();
            let a = s.insert((x_orig, y));
            drop(s);
            match a {
                true => Some(basin(data, x_orig, y, searched)),
                false => None,
            }
        })
        .sum();
    x_row + y_column + 1
}

fn part2(data: &[Vec<usize>]) -> usize {
    let mut points = HashSet::new();
    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 9 {
                points.insert((x, y));
            }
        }
    }
    let cell = RefCell::new(points);
    let mut basins = data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y)))
        .filter(|(x, y)| !cell.borrow().contains(&(*x, *y)))
        //.inspect(|(x, y)| {dbg!(x, y);})
        .map(|(x, y)| basin(data, x, y, &cell))
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn main() {
    let data = parse_grid(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &'static str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    #[test]
    fn test_part1() {
        let data = parse_grid(EXAMPLE);
        assert_eq!(part1(&data), 15)
    }

    #[test]
    fn test_part2() {
        let data = parse_grid(EXAMPLE);
        assert_eq!(part2(&data), 1134)
    }
}
