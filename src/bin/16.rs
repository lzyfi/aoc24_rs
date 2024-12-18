use std::collections::{BinaryHeap, VecDeque};

advent_of_code::solution!(16);

fn parse(input: &str) -> (Vec<Vec<u8>>, [usize; 2], [usize; 2]) {
    let mut temp = vec![];
    let mut start = [0; 2];
    let mut end = [0; 2];

    for (y, line) in input.lines().enumerate() {
        if let Some(i) = line.find("S") {
            start = [i, y];
        } else if let Some(i) = line.find("E") {
            end = [i, y]
        }
        temp.push(line.as_bytes().to_vec());
    }
    (temp, start, end)
}

static DIRS: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

#[derive(Debug)]
struct Node {
    v: [u32; 4],
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.v[3].cmp(&self.v[3])
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.v[3].cmp(&self.v[3]))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.v[3] == other.v[3]
    }
}

impl Eq for Node {}

fn dijkstra(
    start_x: usize,
    start_y: usize,
    end: [usize; 2],
    map: &[Vec<u8>],
) -> (u32, Vec<Vec<Vec<u32>>>) {
    let mut lowest = u32::MAX;

    let mut seen = vec![vec![vec![u32::MAX; 4]; map[0].len()]; map.len()];
    seen[start_y][start_x][0] = 0;

    let mut q = BinaryHeap::new();

    let s = Node {
        v: [start_x as u32, start_y as u32, 0, 0],
    };

    q.push(s);

    while lowest == u32::MAX {
        while let Some(node) = q.pop() {
            let [x, y, rot, c] = node.v;
            if [x as usize, y as usize] == end {
                lowest = c;
                break;
            }

            let left = (rot + 3) % 4;
            let right = (rot + 1) % 4;

            let next = [
                (
                    x as i32 + DIRS[rot as usize][0],
                    y as i32 + DIRS[rot as usize][1],
                    rot,
                    c + 1,
                ),
                (x as i32, y as i32, right, c + 1000),
                (x as i32, y as i32, left, c + 1000),
            ];

            for (n_x, n_y, n_rot, n_cost) in next {
                let (n_x, n_y) = (n_x as usize, n_y as usize);

                if map[n_y][n_x] != b'#' && n_cost < seen[n_y][n_x][n_rot as usize] {
                    q.push(Node {
                        v: [n_x as u32, n_y as u32, n_rot, n_cost],
                    });
                    seen[n_y][n_x][n_rot as usize] = n_cost;
                }
            }
        }
    }

    (lowest, seen)
}

fn bfs(
    map: &[Vec<u8>],
    start: [usize; 2],
    end: [usize; 2],
    lowest: u32,
    seen: Vec<Vec<Vec<u32>>>,
) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut seen = seen;

    let mut q = VecDeque::new();

    for rot in 0..4 {
        if seen[end[1]][end[0]][rot] == lowest {
            q.push_back((end, rot, lowest));
        }
    }

    while let Some(([x, y], rot, c)) = q.pop_front() {
        visited[y][x] = true;

        if [x, y] == start {
            continue;
        }

        let left = (rot + 3) % 4;
        let right = (rot + 1) % 4;

        let next = [
            (x as i32 - DIRS[rot][0], y as i32 - DIRS[rot][1], rot, c - 1),
            (x as i32, y as i32, right, c.wrapping_sub(1000)),
            (x as i32, y as i32, left, c.wrapping_sub(1000)),
        ];

        for (nx, ny, nrot, nc) in next {
            let (nx, ny) = (nx as usize, ny as usize);
            if nc == seen[ny][nx][nrot] {
                q.push_back(([nx, ny], nrot, nc));
                seen[ny][nx][nrot] = u32::MAX;
            }
        }
    }
    visited.iter().flatten().filter(|&&p| p).count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    let res = dijkstra(start[0], start[1], end, &map);
    Some(res.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, end) = parse(input);
    let (lowest, seen) = dijkstra(start[0], start[1], end, &map);
    let res = bfs(&map, start, end, lowest, seen);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
