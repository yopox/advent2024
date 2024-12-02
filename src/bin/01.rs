use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = (vec![], vec![]);
    for line in input.lines() {
        let Some((a, b)) = line.split_once("   ") else { continue; };
        let (Ok(n1), Ok(n2)) = (a.parse::<u32>(), b.parse::<u32>()) else { continue; };
        l1.push(n1);
        l2.push(n2);
    }
    l1.sort(); l2.sort();
    Some((0..l1.len()).map(|i| l1[i].abs_diff(l2[i])).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = (vec![], HashMap::new());
    for line in input.lines() {
        let Some((a, b)) = line.split_once("   ") else { continue; };
        let (Ok(n1), Ok(n2)) = (a.parse::<u32>(), b.parse::<u32>()) else { continue; };
        l1.push(n1);
        *l2.entry(n2).or_insert(0) += 1;
    }
    Some(l1.iter().map(|i| *l2.entry(*i).or_default() * *i).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
