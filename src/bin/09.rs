advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    Data(usize),
    Free,
}

fn parse(input: &str) -> Vec<Block> {
    let mut temp = vec![];
    let mut id = 0;

    let mut data = true;

    for char in input.chars() {
        if let Some(n) = char.to_digit(10) {
            if data {
                temp.append(&mut vec![Block::Data(id); n as usize]);
                id += 1;
            } else {
                temp.append(&mut vec![Block::Free; n as usize]);
            }
            data = !data;
        } else {
            println!("Invalid {}", char as u8);
            panic!();
        }
    }

    temp
}

pub fn part_one(input: &str) -> Option<usize> {
    let layout = parse(input);
    let mut temp = layout.clone();

    let free: Vec<usize> = layout
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Block::Free => Some(i),
            _ => None,
        })
        .collect();

    let len = free.len();
    let mut ind = 0;

    for i in (0..layout.len()).rev() {
        match layout[i] {
            Block::Data(_) => {
                if ind >= len {
                    break;
                }

                let f = free[ind];
                ind += 1;

                if f < i {
                    temp.swap(i, f);
                }
            }
            Block::Free => continue,
        }
    }

    Some(
        temp.iter()
            .enumerate()
            .map(|(i, b)| {
                i * match b {
                    Block::Data(id) => id,
                    Block::Free => &0,
                }
            })
            .sum::<usize>(),
    )
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
