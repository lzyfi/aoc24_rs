use std::num::Wrapping;

advent_of_code::solution!(6);

static DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map {
    width: usize,
    height: usize,
    obstacles: Vec<Vec<u8>>,
    guard: (usize, usize),
    dir: usize,
}

impl Map {
    fn update_guard(&mut self) -> bool {
        let dir = DIRS[self.dir % 4];
        let nx = Wrapping((self.guard.0 as i32 + dir.0) as usize).0;
        let ny = Wrapping((self.guard.1 as i32 + dir.1) as usize).0;
        if ny >= self.height || nx >= self.width {
            return true;
        }

        if self.obstacles[ny][nx] == 1 {
            self.dir += 1;
        } else {
            self.guard = (nx, ny);
            self.obstacles[ny][nx] = 2;
        }

        false
    }

    fn run(&mut self) -> u32 {
        loop {
            if self.update_guard() {
                return self.obstacles.iter().flatten().filter(|n| n == &&2).count() as u32;
            }
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.guard {
                    match self.dir % 4 {
                        0 => print!("^"),
                        1 => print!(">"),
                        2 => print!("v"),
                        3 => print!("<"),
                        _ => print!("X"),
                    }
                } else if self.obstacles[y][x] == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut guard = (0, 0);
    let mut obstacles = vec![];
    for y in 0..height {
        let mut temp = vec![];
        for x in 0..width {
            let char = input.lines().nth(y).unwrap().chars().nth(x);
            if char == Some('^') {
                guard = (x, y);
                temp.push(0);
            } else if char == Some('#') {
                temp.push(1);
            } else if char == Some('.') {
                temp.push(0);
            }
        }
        obstacles.push(temp);
    }
    Map {
        width,
        height,
        obstacles,
        guard,
        dir: 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    Some(map.run())
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
