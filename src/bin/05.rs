use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let rules_end = input.lines().position(|l| l.is_empty()).unwrap();
    input.lines().take(rules_end).for_each(|l| {
        let mut iter = l.split("|");
        let v: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        rules.entry(k).and_modify(|f| f.push(v)).or_insert(vec![v]);
    });
    let updates = input
        .lines()
        .skip(rules_end + 1)
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    let mut updated: Vec<i32> = vec![];

    let mut count = 0;
    'main: for update in updates {
        updated.clear();
        for page in &update {
            if rules.contains_key(page)
                && rules
                    .get(page)
                    .unwrap()
                    .iter()
                    .any(|dep| update.contains(dep) && !updated.contains(dep))
            {
                continue 'main;
            }
            updated.push(*page);
        }
        count += update[update.len() / 2];
    }
    Some(count as u32)
}

fn check(i: &[i32], r: (&i32, &Vec<i32>)) -> bool {
    let pos = i.iter().position(|n| n == r.0).unwrap();
    let c = &i[0..pos];
    !r.1.iter().any(|dep| !c.contains(dep) && i.contains(dep))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    let mut count = 0;

    for update in updates {
        let mut incl = false;
        let mut temp = update.clone();
        for page in &update {
            if !rules.contains_key(page) {
                continue;
            }

            if !check(&temp, rules.get_key_value(page).unwrap()) {
                let last_dep_pos = rules
                    .get(page)
                    .unwrap()
                    .iter()
                    .filter_map(|d| temp.iter().position(|p| d == p))
                    .max()
                    .unwrap();
                let pos = temp.iter().position(|n| n == page).unwrap();
                temp.remove(pos);
                temp.insert(last_dep_pos, *page);
                incl = true;
            }
        }
        if incl {
            count += temp[temp.len() / 2];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
