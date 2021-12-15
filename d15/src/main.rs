use std::fs::read_to_string;

fn parse_maze(maze: &str) -> Vec<Vec<usize>> {
    maze
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        .collect()
}

fn range(x: usize, y: usize, x_size: usize, y_size: usize) -> impl Iterator<Item = (usize, usize)> {
    (x.saturating_sub(1)..x_size.min(x + 2))
        .flat_map(move |n| (y.saturating_sub(1)..y_size.min(y + 2)).map(move |o| (n, o)))
        .filter(move |(n, o)| (x != *n) ^ (y != *o))
}

fn part1(maze: &[Vec<usize>]) -> usize {
    let x_max = maze[0].len();
    let y_max = maze.len();
    let mut sums = vec![vec![usize::MAX; x_max]; y_max];
    sums[0][0] = 0;
    let mut edits = true;
    while edits {
        edits = false;
        for y in 0..y_max {
            for x in 0..x_max {
                let self_sum = sums[y][x];
                for (x_s, y_s) in range(x, y, x_max, y_max) {
                    let candidate_sum = self_sum + maze[y_s][x_s];
                    if sums[y_s][x_s] > candidate_sum {
                        edits = true;
                        sums[y_s][x_s] = candidate_sum;
                    }
                }
            }
        }
    }
    sums[y_max - 1][x_max - 1]
}

fn wrap(mut n: usize) -> usize {
    while n > 9 {
        n -= 9
    }
    n
}

fn part2(maze: &[Vec<usize>]) -> usize {
    let x_max = maze[0].len();
    let y_max = maze.len();
    let mut large = vec![Vec::with_capacity(x_max * 5); y_max * 5];
    for y_n in 0..5 {
        for (y, row) in maze.iter().enumerate() {
            for x_n in 0..5 {
                for v in row {
                    large[y_n * y_max + y].push(wrap(v + y_n + x_n));
                }
            }
        }
    }
    part1(&large)
}


fn main() {
    let maze = parse_maze(&read_to_string("input.txt").unwrap());
    println!("Part 1: {}", part1(&maze));
    println!("Part 2: {}", part2(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part1() {
        let maze = parse_maze(EXAMPLE);
        assert_eq!(part1(&maze), 40);
    }


    #[test]
    fn test_part2() {
        let maze = parse_maze(EXAMPLE);
        assert_eq!(part2(&maze), 315);
    }


    #[test]
    fn test_range() {
        assert_eq!(range(0, 2, 10, 10).count(), 3);
    }
}
