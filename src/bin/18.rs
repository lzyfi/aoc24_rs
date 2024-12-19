use std::collections::VecDeque;

advent_of_code::solution!(18);

fn parse(input: &str) -> Vec<[usize; 2]> {
    input
        .trim()
        .replace("\n", ",")
        .replace("\r", ",")
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| [c[0], c[1]])
        .collect()
}

static DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn bfs(root: [usize; 3], goal: [usize; 2], map: &[Vec<bool>]) -> Option<u32> {
    let height = map.len();
    let width = map[0].len();

    let mut visited = map.to_vec();

    let mut q = VecDeque::new();
    q.push_back(root);

    while let Some(v) = q.pop_front() {
        if [v[0], v[1]] == goal {
            return Some(v[2] as u32);
        }

        for dir in DIRS {
            let [nx, ny] = [
                (v[0] as i32 + dir[0]) as usize,
                (v[1] as i32 + dir[1]) as usize,
            ];
            if !(nx >= width || ny >= height || visited[ny][nx]) {
                visited[ny][nx] = true;
                q.push_back([nx, ny, v[2] + 1]);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = vec![vec![false; 71]; 71];

    let bytes = parse(input);
    for i in 0..1024 {
        map[bytes[i][1]][bytes[i][0]] = true;
    }

    bfs([0, 0, 0], [70, 70], &map)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut map = vec![vec![false; 71]; 71];

    let bytes = parse(input);

    for i in 0.. {
        map[bytes[i][1]][bytes[i][0]] = true;
        if bfs([0, 0, 0], [70, 70], &map).is_none() {
            return Some(format!("{:?}", bytes[i]));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(250));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("[56, 8]".to_string()));
    }
}
