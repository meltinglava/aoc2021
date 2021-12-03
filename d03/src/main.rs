use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::{BitXor, Shl, Shr},
    path::Path,
};

fn read_lines_as_numbers<P: AsRef<Path>>(path: P) -> Result<Vec<usize>> {
    let f = BufReader::new(File::open(path)?);
    f.lines()
        .map(|l| Ok(usize::from_str_radix(&l?, 2).unwrap()))
        .collect()
}

fn digit(n: usize, p: usize) -> usize {
    assert!((0..12).contains(&p));
    n.shl((usize::BITS as usize) - 12 + p).shr(usize::BITS - 1)
}

fn gamma(input: &[usize]) -> usize {
    let mut ones = [0usize; 12];
    let common = input.len() / 2;
    input
        .iter()
        .cloned()
        .for_each(|n| (0usize..12).for_each(|p| ones[p] += digit(n, p)));
    ones.into_iter()
        .map(|n| match n < common {
            true => 0usize,
            false => 1,
        })
        .fold(0usize, |ans, n| ans.shl(1) + n)
}

fn epsilon(n: usize) -> usize {
    let op: usize = 0b_1111_1111_1111;
    n.bitxor(op)
}

fn life_support_rating(input: &[usize], a: usize) -> usize {
    let mut input = input.to_vec();
    for p in 0..12 {
        let find = match (input.iter().cloned().filter(|n| digit(*n, p) == a).count() * 2)
            .cmp(&input.len())
        {
            Ordering::Less => 0,
            Ordering::Equal => a,
            Ordering::Greater => 1,
        };
        input = input.into_iter().filter(|n| digit(*n, p) == find).collect();
        match input.len() {
            1 => return input[0],
            0 => unreachable!(),
            _ => continue,
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input = read_lines_as_numbers("input.txt")?;
    let gam = gamma(&input);
    let eps = epsilon(gam);
    println!("Part1: {}", gam * eps);
    let oxy = life_support_rating(&input, 1);
    let c02 = life_support_rating(&input, 0);
    println!("Part1: {}", oxy * c02);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() {
        let d = 0b_101010101010;
        let mut expected = 1;
        for i in 0..12 {
            assert_eq!(digit(d, i), expected, "\nround: {}", i);
            match expected {
                0 => expected = 1,
                1 => expected = 0,
                n => unreachable!("should alway be 0 or 1, found: {}", n),
            }
        }
    }

    #[test]
    fn test_eps() {
        let d = 0b_101010101010;
        let o = 0b_010101010101;
        assert_eq!(d, epsilon(o));
        assert_eq!(o, epsilon(d));
    }
}
