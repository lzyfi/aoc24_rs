use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
            .sum(),
    )
}

enum Op {
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let ops: Vec<Op> = re
        .find_iter(input)
        .map(|m| {
            let ms = m.as_str();
            match ms {
                _ if ms.starts_with("mul(") => {
                    let mut numbers = ms[4..ms.len() - 1].split(",");
                    Op::Mul(
                        numbers.next().unwrap().parse().unwrap(),
                        numbers.next().unwrap().parse().unwrap(),
                    )
                }
                "do()" => Op::Do,
                "don't()" => Op::Dont,
                _ => panic!(),
            }
        })
        .collect();

    let mut sum = 0;
    let mut on = true;

    for op in ops {
        match op {
            Op::Mul(a, b) => {
                if on {
                    sum += a * b
                }
            }
            Op::Do => on = true,
            Op::Dont => on = false,
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
