use regex::Regex;

advent_of_code::solution!(14);

type Robot = [i32; 4];

fn parse(input: &str) -> Vec<Robot> {
    let mut temp = vec![];

    let re = Regex::new(r"(-?\d*),(-?\d*)").unwrap();
    for line in input.lines() {
        let mut caps = re.captures_iter(line);
        let first = caps.next().unwrap();
        let p: (i32, i32) = (first[1].parse().unwrap(), first[2].parse().unwrap());

        let second = caps.next().unwrap();
        let v: (i32, i32) = (second[1].parse().unwrap(), second[2].parse().unwrap());

        temp.push([p.0, p.1, v.0, v.1]);
    }
    temp
}

fn mod_inverse(a: i32, m: i32) -> i32 {
    let a = a % m;
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let robots = parse(input);

    let mut quads = [0, 0, 0, 0];

    for [x, y, vx, vy] in robots {
        let nx = (x + 100 * vx).rem_euclid(width);
        let ny = (y + 100 * vy).rem_euclid(height);

        let mid_x = width / 2;
        let mid_y = height / 2;
        if nx < mid_x && ny < mid_y {
            quads[0] += 1;
        }
        if nx > mid_x && ny < mid_y {
            quads[1] += 1;
        }
        if nx < mid_x && ny > mid_y {
            quads[2] += 1;
        }
        if nx > mid_x && ny > mid_y {
            quads[3] += 1;
        }
    }

    Some(quads.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;

    let mut robots = parse(input);

    // columns repeat every width times, rows every height times
    // so we can solve for columns mod widt & rows mod height
    // and use Chinese remainder theory magic to get the actual solution

    let mut cols = Vec::new();

    for i in 0..width {
        let mut temp = vec![0; width];

        for [x, _, vx, _] in &robots {
            let ind = (x + vx * i as i32).rem_euclid(width as i32);
            temp[ind as usize] += 1;
        }

        // width of christmas tree bounding box is 33
        if temp.iter().filter(|&&v| v >= 33).count() >= 2 {
            cols.push(i);
        }
    }

    let mut rows = Vec::new();

    for i in 0..height {
        let mut temp = vec![0; height];

        for [_, y, _, vy] in &robots {
            let ind = (y + vy * i as i32).rem_euclid(height as i32);
            temp[ind as usize] += 1;
        }

        // height of christmas tree bounding box is 31
        if temp.iter().filter(|&&v| v >= 31).count() >= 2 {
            rows.push(i);
        }
    }

    if rows.len() == 1 && cols.len() == 1 {
        let a = cols[0] as i32;
        let b = rows[0] as i32;

        let width = width as i32;
        let height = height as i32;

        // Chinese remainder theory magic
        let mod_inv = mod_inverse(width, height);
        return Some((a + mod_inv * (b - a) * width).rem_euclid(width * height) as u32);
    }

    // if everything else fails look for combinations of all unique positions
    'outer: for i in 1..100_000 {
        robots.iter_mut().for_each(|r| {
            r[0] += r[2];
            r[1] += r[3]
        });
        for y in 0..height {
            for x in 0..width {
                if robots
                    .iter()
                    .filter(|r| r[0] == x as i32 && r[1] == y as i32)
                    .count()
                    > 1
                {
                    continue 'outer;
                }
            }
        }

        return Some(i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(226548000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7753));
    }
}
