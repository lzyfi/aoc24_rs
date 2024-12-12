use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::num::Wrapping;

advent_of_code::solution!(6);

static DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    obstacles: Vec<Vec<u8>>,
    guard: (usize, usize),
    dir: usize,
    unique: usize,
}

impl Map {
    fn update_guard(&mut self) -> bool {
        let dir = DIRS[self.dir % 4];
        let nx = Wrapping((self.guard.0 as i32 + dir.0) as usize).0;
        let ny = Wrapping((self.guard.1 as i32 + dir.1) as usize).0;
        if ny >= self.height || nx >= self.width {
            return true;
        }

        match self.obstacles[ny][nx] {
            1 => self.dir += 1,
            2 => self.guard = (nx, ny),
            0 => {
                self.guard = (nx, ny);
                self.obstacles[ny][nx] = 2;
                self.unique += 1;
            }
            _ => (),
        }
        false
    }

    fn run(&mut self) -> u32 {
        loop {
            if self.update_guard() {
                return self.unique as u32;
            }
        }
    }

    fn escapes(&mut self, threshold: usize) -> bool {
        let mut old;
        let mut count = 0;

        loop {
            old = self.unique;

            if self.update_guard() {
                return true;
            }

            count = if self.unique == old { count + 1 } else { 0 };
            if count > threshold {
                return false;
            }
        }
    }

    fn _print(&self) {
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
                } else if self.obstacles[y][x] == 2 {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
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
        unique: 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    Some(map.run())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut ran = map.clone();
    let max = map.height * map.width;

    ran.run();

    Some(
        (0..map.height)
            .into_par_iter()
            .map(|y| {
                let mut count = 0;
                for x in 0..map.width {
                    let mut map_cl = map.clone();

                    if ran.obstacles[y][x] != 2 || map.guard == (x, y) {
                        continue;
                    }

                    map_cl.obstacles[y][x] = 1;
                    if !map_cl.escapes(max) {
                        count += 1;
                    }
                }
                count
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
