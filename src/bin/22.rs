advent_of_code::solution!(22);

fn parse(input: &str) -> Vec<usize> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

fn mix(other: usize, secret: usize) -> usize {
    secret ^ other
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next(secret: usize) -> usize {
    let mut sec;
    sec = mix(secret * 64, secret);
    sec = prune(sec);

    sec = mix(sec / 32, sec);
    sec = prune(sec);

    sec = mix(sec * 2048, sec);


    prune(sec)
}

pub fn part_one(input: &str) -> Option<usize> {

    let mut count = 0;
    
    let secrets = parse(input);
    for secret in secrets {
        let mut sec = secret;
        for _ in 0..2000 {
            sec = next(sec);
        }
        count += sec;
    }
    
    Some(count)
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
        assert_eq!(result, Some(69));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
