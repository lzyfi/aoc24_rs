advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Map {
    plots: Vec<Vec<u8>>,
}

fn parse(input: &str) -> Map {
    let plots = input
        .lines()
        .map(|s| s.bytes().collect())
        .collect::<Vec<_>>();
    Map {
        plots: plots.clone(),
    }
}

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn cover(x: usize, y: usize, map: Map, visited: &mut Vec<Vec<bool>>) -> (u32, u32, u32) {
    let height = map.plots.len() as i32;
    let width = map.plots[0].len() as i32;
    let curr = map.plots[y][x];

    let mut perimeter = 0;
    let mut area = 0;
    let mut corners = 0;

    visited[y][x] = true;

    for i in 0..4 {
        let a_x = x as i32 + DIRS[i].0;
        let a_y = y as i32 + DIRS[i].1;

        let b_x = x as i32 + DIRS[(i + 1) % 4].0;
        let b_y = y as i32 + DIRS[(i + 1) % 4].1;

        let ab_x = x as i32 + DIRS[i].0 + DIRS[(i + 1) % 4].0;
        let ab_y = y as i32 + DIRS[i].1 + DIRS[(i + 1) % 4].1;

        let w = vec![];
        let v = curr + 1;

        let a = map
            .plots
            .get(a_y as usize)
            .unwrap_or(&w)
            .get(a_x as usize)
            .unwrap_or(&v);

        let b = map
            .plots
            .get(b_y as usize)
            .unwrap_or(&w)
            .get(b_x as usize)
            .unwrap_or(&v);

        let ab = map
            .plots
            .get(ab_y as usize)
            .unwrap_or(&w)
            .get(ab_x as usize)
            .unwrap_or(&v);

        if a != &curr && b != &curr {
            corners += 1;
        }
        if a == &curr && b == &curr && ab != &curr {
            corners += 1;
        }
    }

    for dir in DIRS {
        let nx = x as i32 + dir.0;
        let ny = y as i32 + dir.1;

        if nx < 0 || ny < 0 || nx >= width || ny >= height {
            perimeter += 1;
        } else {
            let nx = nx as usize;
            let ny = ny as usize;
            let new = map.plots[ny][nx];
            if curr == new && !visited[ny][nx] {
                visited[ny][nx] = true;
                area += 1;
                let next = cover(nx, ny, map.clone(), visited);
                area += next.0;
                perimeter += next.1;
                corners += next.2
            } else if curr != new {
                perimeter += 1;
            }
        }
    }
    (area, perimeter, corners)
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let height = map.plots.len();
    let width = map.plots[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, perimeter, _) = cover(x, y, map.clone(), &mut visited);
                count += (area + 1) * perimeter;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);

    let height = map.plots.len();
    let width = map.plots[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, _, corners) = cover(x, y, map.clone(), &mut visited);
                count += (area + 1) * corners;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
