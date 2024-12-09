use core::fmt;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Map {
    antennaes: HashMap<char, Vec<Coords>>,
    antinodes: Vec<Coords>,
    height: usize,
    width: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = String::new();
        for y in 0..self.height {
            'x: for x in 0..self.width {
                for (char, coords) in self.antennaes.clone() {
                    if coords.contains(&Coords { x, y }) {
                        temp += &char.to_string();
                        continue 'x;
                    }
                }
                if self.antinodes.contains(&Coords { x, y }) {
                    temp += "#";
                    continue;
                }
                temp += ".";
            }
            temp += "\n";
        }
        write!(f, "{}", temp)
    }
}

fn parse(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut map: HashMap<char, Vec<Coords>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if char != '.' {
                map.entry(char)
                    .and_modify(|coords| coords.push(Coords { x, y }))
                    .or_insert(vec![Coords { x, y }]);
            }
        }
    }
    Map {
        antennaes: map,
        antinodes: vec![],
        height,
        width,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);

    for antennae in map.antennaes.clone() {
        for i in 0..antennae.1.len() {
            for j in 0..antennae.1.len() {
                if i != j {
                    let x = antennae.1[i].x as isize;
                    let y = antennae.1[i].y as isize;
                    let x2 = antennae.1[j].x as isize;
                    let y2 = antennae.1[j].y as isize;

                    let nx = 2 * x - x2;
                    let ny = 2 * y - y2;

                    if nx < 0 || ny < 0 || nx >= map.width as isize || ny >= map.height as isize {
                        continue;
                    }

                    let pos = Coords {
                        x: nx as usize,
                        y: ny as usize,
                    };

                    map.antinodes.push(pos);
                }
            }
        }
    }
    map.antinodes.sort_unstable();
    map.antinodes.dedup_by(|a, b| a == b);
    //println!("{map}");
    Some(map.antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);
    for antennae in map.antennaes.clone() {
        for i in 0..antennae.1.len() {
            for j in 0..antennae.1.len() {
                if i != j {
                    let x = antennae.1[i].x as isize;
                    let y = antennae.1[i].y as isize;
                    let x2 = antennae.1[j].x as isize;
                    let y2 = antennae.1[j].y as isize;

                    let dx = x2 - x;
                    let dy = y2 - y;

                    let mut iter = 0;

                    loop {
                        let nx = x + dx * iter;
                        let ny = y + dy * iter;

                        if nx < 0 || ny < 0 || nx >= map.width as isize || ny >= map.height as isize
                        {
                            break;
                        }

                        map.antinodes.push(Coords {
                            x: nx as usize,
                            y: ny as usize,
                        });
                        iter += 1;
                    }
                }
            }
        }
    }
    map.antinodes.sort_unstable();
    map.antinodes.dedup_by(|a, b| a == b);
    //println!("{map}");
    Some(map.antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
