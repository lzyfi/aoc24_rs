use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Map {
    plots: Vec<Vec<u8>>,
    regions: HashMap<u8, Vec<(usize, usize)>>
}

impl Map {
    fn construct_regions(&mut self) {
    }
}

fn parse(input: &str) -> Map {
    let plots = input.lines().map(|s| s.bytes().collect()).collect();
    Map { plots, regions: HashMap::new() }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    map.construct_regions();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
