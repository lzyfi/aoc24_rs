advent_of_code::solution!(15);

fn parse(input: &str) -> (Vec<u8>, Vec<Vec<u8>>, [i32; 2]) {
    let mut change = false;

    let mut moves = vec![];
    let mut map = vec![];
    let mut pos = [0, 0];

    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            change = true;
        }

        if change {
            moves.append(&mut line.as_bytes().to_vec());
        } else {
            if let Some(x) = line.bytes().position(|b| b == 64) {
                pos = [x as i32, i as i32];
            }
            map.push(line.replace("@", ".").as_bytes().to_vec());
        }
    }
    (moves, map, pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (moves, mut map, mut pos) = parse(input);
    for m in moves {
        let dir = match m {
            b'v' => [0, 1],
            b'>' => [1, 0],
            b'^' => [0, -1],
            b'<' => [-1, 0],
            _ => panic!("Invalid move {m}"),
        };
        let [nx, ny] = [pos[0] + dir[0], pos[1] + dir[1]];
        if map[ny as usize][nx as usize] == b'#'
        {
            continue;
        }

        if map[ny as usize][nx as usize] == b'.' {
            pos = [nx, ny];
            continue;
        }

        let mut moved = false;
        let (mut cx, mut cy) = (nx, ny);

        loop {
            (cx, cy) = (cx + dir[0], cy + dir[1]);

            if map[cy as usize][cx as usize] == b'#' {
                break;
            }

            if map[cy as usize][cx as usize] == b'.' {
                map[cy as usize][cx as usize] = b'O';
                moved = true;
                break;
            }
        }

        if moved {
            map[ny as usize][nx as usize] = b'.';
            pos = [nx, ny];
        }
    }
    let mut sum = 0;

    for (y, v) in map.iter().enumerate() {
        for (x, b) in v.iter().enumerate() {
            if b == &b'O' {
                sum += 100 * y + x;
            }
        }
    }

    Some(sum as u32)
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
