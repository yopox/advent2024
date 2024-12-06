use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut m = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            m.insert((x, y), char);
            x += 1;
        }
        y += 1;
    }

    let mut count = 0;

    for (&(x, y), &v) in m.iter() {
        if v != 'X' { continue }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x+1, y)), m.get(&(x+2, y)), m.get(&(x+3, y))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x-1, y)), m.get(&(x-2, y)), m.get(&(x-3, y))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x, y+1)), m.get(&(x, y+2)), m.get(&(x, y+3))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x, y-1)), m.get(&(x, y-2)), m.get(&(x, y-3))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x+1, y+1)), m.get(&(x+2, y+2)), m.get(&(x+3, y+3))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x+1, y-1)), m.get(&(x+2, y-2)), m.get(&(x+3, y-3))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x-1, y+1)), m.get(&(x-2, y+2)), m.get(&(x-3, y+3))) { count += 1; }
        if let (Some(&'M'), Some(&'A'), Some(&'S')) = (m.get(&(x-1, y-1)), m.get(&(x-2, y-2)), m.get(&(x-3, y-3))) { count += 1; }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut m = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            m.insert((x, y), char);
            x += 1;
        }
        y += 1;
    }

    let mut count = 0;

    for (&(x, y), &v) in m.iter() {
        if v != 'A' { continue }

        let x1 = m.get(&(x-1, y-1));
        let x2 = m.get(&(x+1, y+1));
        let y1 = m.get(&(x+1, y-1));
        let y2 = m.get(&(x-1, y+1));

        match ((x1, x2), (y1, y2)) {
            ((Some(&'M'), Some(&'S')), (Some(&'M'), Some(&'S')))
            | ((Some(&'S'), Some(&'M')), (Some(&'M'), Some(&'S')))
            | ((Some(&'M'), Some(&'S')), (Some(&'S'), Some(&'M')))
            | ((Some(&'S'), Some(&'M')), (Some(&'S'), Some(&'M')))
            => count += 1,
            _ => {},
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
