use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut scores = HashMap::new();

    for (&(y, x), &height) in map.iter() {
        if height == 9 {
            let mut reachable = HashSet::new();
            descend(x, y, None, &map, &mut reachable);
            for &(y2, x2) in reachable.iter() {
                *scores.entry((y2, x2)).or_insert(0) += 1;
            }
        }
    }

    Some(scores.values().sum())
}

fn parse_input(input: &str) -> HashMap<(usize, usize), u8> {
    let mut map = HashMap::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for char in line.chars() {
            if let Some(n) = char.to_digit(10) {
                map.insert((y, x), n as u8);
            }
            x += 1;
        }
        y += 1;
    }

    map
}

fn descend(x: usize, y: usize, previous: Option<u8>, map: &HashMap<(usize, usize), u8>, reachable: &mut HashSet<(usize, usize)>) {
    let Some(height) = map.get(&(y, x)) else { return };
    if let Some(previous) = previous {
        if *height != previous - 1 { return; }
    }
    if *height == 0 {
        reachable.insert((y, x));
        return;
    }

    if x > 0 { descend(x - 1, y, Some(*height), map, reachable); }
    if y > 0 { descend(x, y - 1, Some(*height), map, reachable); }
    descend(x + 1, y, Some(*height), map, reachable);
    descend(x, y + 1, Some(*height), map, reachable);
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut scores = HashMap::new();

    for (&(y, x), &height) in map.iter() {
        if height == 9 { descend2(x, y, None, &map, &mut scores); }
    }

    Some(scores.values().sum())
}

fn descend2(x: usize, y: usize, previous: Option<u8>, map: &HashMap<(usize, usize), u8>, scores: &mut HashMap<(usize, usize), u32>) {
    let Some(height) = map.get(&(y, x)) else { return };
    if let Some(previous) = previous {
        if *height != previous - 1 { return; }
    }
    if *height == 0 {
        *scores.entry((y, x)).or_default() += 1;
        return;
    }

    if x > 0 { descend2(x - 1, y, Some(*height), map, scores); }
    if y > 0 { descend2(x, y - 1, Some(*height), map, scores); }
    descend2(x + 1, y, Some(*height), map, scores);
    descend2(x, y + 1, Some(*height), map, scores);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
