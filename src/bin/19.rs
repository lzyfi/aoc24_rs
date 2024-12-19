use std::sync::{LazyLock, Mutex};
use rustc_hash::FxHashMap;

advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs = lines.skip(1).map(|s| s.to_string()).collect();

    (towels, designs)
}

fn try_rec(design: String, towels: &[String], cache: &mut FxHashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(n) = cache.get(&design) {
        return *n;
    }

    let count = towels
        .iter()
        .map(|t| {
            if design.starts_with(t) {
                try_rec(design[t.len()..].to_string(), towels, cache)
            } else {
                0
            }
        })
        .sum();
    
    cache.insert(design, count);
    count
}

static CACHE: LazyLock<Mutex<FxHashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(FxHashMap::default()));

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, designs) = parse(input);

    let mut count = 0;
    for d in designs {
        if try_rec(d, &towels, &mut CACHE.lock().unwrap()) != 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, designs) = parse(input);

    let mut count = 0;
    for d in designs {
        count += try_rec(d, &towels, &mut CACHE.lock().unwrap());
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
