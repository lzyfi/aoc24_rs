use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        if let (Some(le), Some(ri)) = (split.next(), split.next()) {
            if let (Ok(le_num), Ok(ri_num)) = (le.parse(), ri.parse()) {
                left.push(le_num);
                right.push(ri_num);
            }
        }
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let mut count = 0;
    for i in 0..left.len() {
        count += (right[i] - left[i]).abs()
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();
    let mut map = HashMap::with_capacity(right.len());

    for n in right {
        *map.entry(n).or_insert(0) += 1;
    }

    let mut count = 0;
    for n in left {
        count += n * map.get(&n).unwrap_or(&0);
    }
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
