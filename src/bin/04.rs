advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];
    let board: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let word = b"XMAS";
    let mut count = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            for (dx, dy) in dirs {
                let mut m = true;
                for (iter, char) in word.iter().enumerate() {
                    let y_n = (y as i32 + dy * iter as i32) as usize;
                    let x_n = (x as i32 + dx * iter as i32) as usize;
                    if y_n >= board.len() || x_n >= board[1].len() {
                        m = false;
                        break;
                    }

                    if board[y_n][x_n] != *char {
                        m = false;
                        break;
                    }
                }
                if m {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;
    for y in 1..board.len() - 1 {
        for x in 1..board[0].len() - 1 {
            if board[y][x] != 'A' {
                continue;
            }

            if (board[y - 1][x - 1] == 'M' && board[y + 1][x + 1] == 'S'
                || board[y - 1][x - 1] == 'S' && board[y + 1][x + 1] == 'M')
                && (board[y - 1][x + 1] == 'M' && board[y + 1][x - 1] == 'S'
                    || board[y - 1][x + 1] == 'S' && board[y + 1][x - 1] == 'M')
            {
                count += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
