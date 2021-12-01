use std::{fs::File, io::{Result, BufReader, BufRead}, path::Path};

fn read_lines_as_numbers<P: AsRef<Path>>(path: P) -> Result<Vec<usize>> {
    let f = BufReader::new(File::open(path)?);
    f.lines().map(|l| Ok(l?.parse::<usize>().unwrap())).collect()
}

fn increases(input: &[usize]) -> usize {
    input.windows(2).filter(|i| i[0] < i[1]).count()
}

fn main() -> Result<()> {
    let input = read_lines_as_numbers("input.txt")?;
    println!("Part1: {}", increases(&input));
    let part2: Vec<usize> = input.windows(3).map(|t| t.into_iter().sum()).collect();
    println!("Part2: {}", increases(&part2));
    Ok(())
}
