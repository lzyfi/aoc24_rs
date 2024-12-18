use std::fmt::Write;

advent_of_code::solution!(17);

fn parse(input: &str) -> ([usize; 3], Vec<u8>) {
    let mut registers = [0; 3];
    let mut ops = vec![];

    let mut c = 0;

    for line in input.lines() {
        if line.starts_with("Register") {
            registers[c] = line.split(": ").nth(1).unwrap().parse().unwrap();
            c += 1;
        } else if !line.is_empty() {
            ops = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|c| c.parse().unwrap())
                .collect();
        }
    }

    (registers, ops)
}

fn run(registers: [usize; 3], ops: &[u8]) -> Vec<u8> {
    let mut registers = registers;
    let mut ip = 0;

    let mut output = Vec::new();

    loop {
        if ip >= ops.len() - 1 {
            break;
        }
        let combo = match ops[ip + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => 0,
        };
        match ops[ip] {
            0 => {
                registers[0] /= 2_usize.pow(combo as u32);
                ip += 2;
            }
            1 => {
                registers[1] ^= ops[ip + 1] as usize;
                ip += 2;
            }
            2 => {
                registers[1] = combo % 8;
                ip += 2;
            }
            3 => {
                if registers[0] != 0 {
                    ip = ops[ip + 1] as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                registers[1] ^= registers[2];
                ip += 2;
            }
            5 => {
                output.push((combo % 8) as u8);
                ip += 2;
            }
            6 => {
                registers[1] = registers[0] / 2_usize.pow(combo as u32);
                ip += 2;
            }
            7 => {
                registers[2] = registers[0] / 2_usize.pow(combo as u32);
                ip += 2;
            }
            _ => panic!(),
        }
    }
    output
}

pub fn part_one(input: &str) -> Option<String> {
    let (registers, ops) = parse(input);
    Some(
        run(registers, &ops)
            .iter()
            .fold(String::new(), |mut acc, n| {
                if acc.is_empty() {
                    let _ = write!(acc, "{n}");
                } else {
                    let _ = write!(acc, ",{n}");
                }
                acc
            }),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut registers, ops) = parse(input);

    let mut candidates = vec![0];
    let max = ops.len();

    for iter in 0..max {
        let mut cands_temp = vec![];
        for c in candidates.clone() {
            for i in 0..8 {
                let a = (c << 3) | i;
                registers[0] = a;
                let out = run(registers, &ops);

                if out == ops {
                    return Some(a);
                }
                if out.get(out.len().wrapping_sub(iter).wrapping_sub(1)) == ops.get(max - iter - 1)
                {
                    cands_temp.push(a);
                }
            }
        }
        candidates = cands_temp;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0");
        assert_eq!(result, Some(117440));
    }
}
