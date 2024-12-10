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

fn parse2(input: &str) -> Vec<isize> {
    let mut temp = vec![];
    let mut id = 0;

    let mut data = true;

    for char in input.chars() {
        if let Some(n) = char.to_digit(10) {
            if data {
                temp.append(&mut vec![id; n as usize]);
                id += 1;
            } else {
                temp.append(&mut vec![-1; n as usize]);
            }
            data = !data;
        } else {
            println!("Invalid {}", char as u8);
            panic!();
        }
    }

    temp
}

fn checksum2(input: &[isize]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, n)| match n {
            -1 => 0,
            id => i * *id as usize,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let layout = parse(input);
    let mut temp = layout.clone();

    let free: Vec<usize> = layout
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Block::Free => Some(i),
            Block::Data(_) => None,
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

pub fn part_two(input: &str) -> Option<usize> {
    let layout = parse2(input);
    let mut temp = layout.clone();

    let max = *layout.iter().max().unwrap();

    let mut free = temp
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .split(|n| *n.1 != -1)
        .filter_map(|v| {
            if v.is_empty() {
                None
            } else {
                Some((v[0].0, v.len()))
            }
        })
        .collect::<Vec<_>>();

    let mut inds = vec![0; max as usize + 1];
    inds[0] = 0;
    let mut count = 0;
    for (ind, val) in temp.iter().enumerate() {
        if *val == count + 1 {
            inds[*val as usize] = ind;
            count = *val;
        }
    }

    for id in (0..=max).rev() {
        let size = layout.iter().filter(|n| n == &&id).count();
        for (f_ind, (i, s)) in free.clone().iter().enumerate() {
            if *s < size {
                continue;
            }

            if *i > inds[id as usize] {
                break;
            }

            if *s == size {
                for j in 0..size {
                    temp.swap(i + j, inds[id as usize] + j);
                }
                free.remove(f_ind);
                break;
            }

            if *s > size {
                for j in 0..size {
                    temp.swap(i + j, inds[id as usize] + j);
                }
                free[f_ind] = (free[f_ind].0 + size, free[f_ind].1 - size);
                break;
            }
        }
    }

    Some(checksum2(&temp))
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
        assert_eq!(result, Some(2858));
    }
}
