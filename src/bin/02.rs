use std::cmp;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut temp: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        temp.push(
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }
    temp
}

fn safe(report: &[i32]) -> bool {
    report.len() < 2 || {
        match report[1].cmp(&report[0]) {
            cmp::Ordering::Less => !report
                .windows(2)
                .any(|v| v[0] - v[1] < 1 || v[0] - v[1] > 3),
            cmp::Ordering::Greater => !report
                .windows(2)
                .any(|v| v[1] - v[0] < 1 || v[1] - v[0] > 3),
            _ => false,
        }
    }
}

fn safe2(report: Vec<i32>) -> bool {
    if safe(&report) {
        return true;
    }
    let mut temp = report;
    for i in 0..temp.len() {
        let rem = temp.remove(i);
        if safe(&temp) {
            return true;
        }
        temp.insert(i, rem);
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).into_iter().filter(|r| safe(r)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .into_iter()
            .filter(|r| safe2(r.clone()))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
