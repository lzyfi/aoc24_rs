advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Machine {
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    goal: (isize, isize),
}

impl Machine {
    fn cost(&self) -> Option<usize> {
        let (a_x, a_y, b_x, b_y, (x, y)) = (self.a_x, self.a_y, self.b_x, self.b_y, self.goal);

        let but_a = (b_x * y - b_y * x).checked_div(a_x * b_y - a_y * b_x);
        let but_b = (a_x * y - a_y * x).checked_div(a_x * b_y - a_y * b_x);

        if let (Some(ba), Some(bb)) = (but_a, but_b) {
            let ba = -ba;

            if ba * a_x + bb * b_x == x && ba * a_y + bb * b_y == y {
                return Some((ba * 3 + bb) as usize);
            }
        }

        None
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let parts: Vec<Vec<String>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<_>>();

    let mut temp = vec![];

    for part in parts {
        let mut tp: [isize; 6] = [0, 0, 0, 0, 0, 0];
        let mut count = 0;
        for line in part {
            if line.starts_with("Button") {
                let b = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .replace("X+", "")
                    .replace("Y+", "")
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>();

                tp[count] = b[0];
                tp[count + 1] = b[1];
            } else {
                let b = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .replace("X=", "")
                    .replace("Y=", "")
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>();

                tp[count] = b[0];
                tp[count + 1] = b[1];
            }
            count += 2;
        }

        temp.push(Machine {
            a_x: tp[0],
            a_y: tp[1],
            b_x: tp[2],
            b_y: tp[3],
            goal: (tp[4], tp[5]),
        });
    }

    temp
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse(input).iter().filter_map(Machine::cost).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .iter()
            .filter_map(|m| {
                let mut m = m.clone();
                m.goal = (m.goal.0 + 10_000_000_000_000, m.goal.1 + 10_000_000_000_000);
                m.cost()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
