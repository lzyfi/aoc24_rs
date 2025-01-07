use std::collections::VecDeque;

advent_of_code::solution!(20);

fn parse(input: &str) -> (Vec<Vec<bool>>, [usize; 2], [usize; 2]) {
    let mut temp = vec![];
    let mut start = [0, 0];
    let mut end = [0, 0];
    for (y, line) in input.lines().enumerate() {
        let mut temp2 = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = [x, y];
                    temp2.push(false);
                }
                'E' => {
                    end = [x, y];
                    temp2.push(false);
                }
                '#' => temp2.push(true),
                _ => temp2.push(false),
            }
        }
        temp.push(temp2);
    }

    (temp, start, end)
}

static DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn bfs(map: &[Vec<bool>], start: [usize; 3], end: [usize; 2]) -> (usize, Vec<Vec<usize>>) {
    let height = map.len();
    let width = map[0].len();

    let mut visited = map.to_vec();
    let mut lengths = vec![vec![usize::MAX; width]; height];

    let mut q = VecDeque::new();
    q.push_back(start);

    let mut length = usize::MAX;

    while let Some([x, y, l]) = q.pop_front() {
        if l < lengths[y][x] {
            lengths[y][x] = l;
        }

        if [x, y] == end {
            if l < length {
                length = l;
            }
            continue;
        }

        for dir in DIRS {
            let [nx, ny] = [
                (x as i32 + dir[0]) as usize,
                (y as i32 + dir[1]) as usize,
            ];
            if !(nx >= width || ny >= height || visited[ny][nx]) {
                visited[ny][nx] = true;
                q.push_back([nx, ny, l + 1]);
            }
        }
    }

    (length, lengths)
}

fn taxicab(a: &[i32], b: &[i32]) -> u32 {
    a[0].abs_diff(b[0]) + a[1].abs_diff(b[1])
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    let (length, lengths) = bfs(&map, [start[0], start[1], 0], end);
    let (_, lengths2) = bfs(&map, [end[0], end[1], 0], start);
    let height = map.len();
    let width = map[0].len();

    let mut cheats = 0;

    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some([x, y]) = q.pop_front() {
        for dir in DIRS {
            let [mut nx, mut ny] = [
                (x as i32 + dir[0]) as usize,
                (y as i32 + dir[1]) as usize,
            ];

            if nx >= width || ny >= height {
                continue;
            }

            if lengths[ny][nx] == lengths[y][x] + 1 {
                q.push_back([nx, ny]);
            }

            [nx, ny] = [
                (nx as i32 + dir[0]) as usize,
                (ny as i32 + dir[1]) as usize,
            ];

            if nx >= width || ny >= height || map[ny][nx] {
                continue;
            }

            let cheat_len = length as i32 - lengths[y][x] as i32 - lengths2[ny][nx] as i32 - 2;

            if cheat_len >= 100 {
                cheats += 1;
            }
        }
    }
    
    Some(cheats)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    let (length, lengths) = bfs(&map, [start[0], start[1], 0], end);
    let (_, lengths2) = bfs(&map, [end[0], end[1], 0], start);

    let mut cheats = 0;

    let mut cheat_starts = vec![];

    for (y, v) in lengths.iter().enumerate() {
        for (x, n) in v.iter().enumerate() {
            if n != &usize::MAX {
                cheat_starts.push([x as i32, y as i32, *n as i32]);
            }
        }
    }

    let mut cheat_ends = vec![];

    for (y, v) in lengths2.iter().enumerate() {
        for (x, n) in v.iter().enumerate() {
            if n != &usize::MAX {
                cheat_ends.push([x as i32, y as i32, *n as i32]);
            }
        }
    }

    for start in cheat_starts {
        for end in &cheat_ends {
            let dist = taxicab(&start, end);
            if dist <= 20 {
                let cheat_len = length as i32 - start[2] - end[2] - dist as i32;
                if cheat_len >= 100 {
                    cheats += 1;
                }
            }
        }
    }

    Some(cheats)
}

// not useful to test this, won't be sharing inputs

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(0));
//     }

//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(0));
//     }
// }
