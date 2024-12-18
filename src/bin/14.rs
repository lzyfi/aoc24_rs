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

    let robots = parse(input);

    // the christmas tree seems to have every robot in a unique position
    'outer: for i in 1..100_000 {
        let mut locations = vec![vec![false; width]; height];

        for r in &robots {
            let x = (r[0] + i * r[2]).rem_euclid(width as i32) as usize;
            let y = (r[1] + i * r[3]).rem_euclid(height as i32) as usize;
            if locations[y][x] {
                continue 'outer;
            } else {
                locations[y][x] = true;
            }
        }
        
        return Some(i as u32);
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
