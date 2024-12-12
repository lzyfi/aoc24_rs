use rustc_hash::FxHashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blink(stone: usize) -> (usize, Option<usize>) {
    match stone {
        0 => (1, None),
        n => {
            let digits = n.ilog10() + 1;
            if digits % 2 == 0 {
                let tens = 10_usize.pow(digits / 2);
                (n / tens, Some(n % tens))
            } else {
                (n * 2024, None)
            }
        }
    }
}

fn count(stone: usize, gens: usize, cache: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if gens == 0 {
        1
    } else {
        let key = (stone, gens);

        if let Some(res) = cache.get(&key) {
            return *res;
        }

        let (f, s) = blink(stone);

        let mut c = count(f, gens - 1, cache);

        if let Some(second) = s {
            c += count(second, gens - 1, cache);
        }

        cache.insert(key, c);

        c
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cache = FxHashMap::default();

    let mut total = 0;

    for stone in parse(input) {
        total += count(stone, 25, &mut cache);
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = FxHashMap::default();

    let mut total = 0;

    for stone in parse(input) {
        total += count(stone, 75, &mut cache);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
