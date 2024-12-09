use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .replace(":", "")
        .lines()
        .map(|l| l.split(" ").map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn calc(v: &[usize], ins: usize) -> usize {
    let mut res = v[0];
    for i in 1..v.len() {
        match (ins & (1 << (i - 1))) != 0 {
            true => res += v[i],
            false => res *= v[i],
        };
    }
    res
}

fn ternary(num: usize) -> Vec<u8> {
    let mut res = vec![0; 41];
    let mut num = num;
    for i in 0..41 {
        let quot = num / 3;
        let rem = num % 3;
        num = quot;
        res[i] = rem as u8;
        if quot == 0 {
            break;
        }
    }
    res
}

fn concat(a: usize, b: usize) -> usize {
    let mut n = 10;
    while b >= n {
        n *= 10;
    }
    a * n + b
}

fn calc2(v: &[usize], ins: &[u8]) -> usize {
    let mut res = v[0];
    for i in 1..v.len() {
        match ins[i - 1] {
            0 => res += v[i],
            1 => res *= v[i],
            2 => res = concat(res, v[i]),
            _ => (),
        };
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let eqs = parse(input);
    let mut count = 0;

    for eq in eqs {
        if (0..(2_usize).pow((eq.len() - 2) as u32)).any(|ins| calc(&eq[1..], ins) == eq[0]) {
            count += eq[0];
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let eqs = parse(input);
    let mut count = 0;

    for eq in eqs {
        if (0..(3_usize).pow((eq.len() - 2) as u32))
            .into_par_iter()
            .any(|ins| calc2(&eq[1..], &ternary(ins)) == eq[0])
        {
            count += eq[0];
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
