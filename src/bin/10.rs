advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

static DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn score(x: usize, y: usize, map: &[Vec<u32>], nines: &mut Vec<(usize, usize)>) -> usize {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let old = map[y][x];

    let mut count = 0;

    for (dx, dy) in DIRS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 || nx >= width || ny >= height {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        let new = map[ny][nx];

        if new == 9 && old == 8 && !nines.contains(&(nx, ny)) {
            nines.push((nx, ny));
            count += 1;
        }
        if new == old + 1 {
            count += score(nx, ny, map, nines);
        }
    }
    count
}

fn score2(x: usize, y: usize, map: &[Vec<u32>]) -> usize {
    let old = map[y][x];
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut count = 0;

    for (dx, dy) in DIRS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 || nx >= width || ny >= height {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        let new = map[ny][nx];
        
        if new == 9 && old == 8 {
            count += 1;
        }
        if new == old + 1 {
            count += score2(nx, ny, map);
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let mut nines = Vec::new();

    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                nines.clear();
                count += score(x, y, &map, &mut nines);
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                count += score2(x, y, &map)
            }
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
