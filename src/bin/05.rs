use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut before = HashMap::new();
    let mut after = HashMap::new();
    let mut pages_section = false;
    let mut count = 0;

    'l : for line in input.lines() {
        if line == "" { pages_section = true; continue }
        if !pages_section {
            let Some((b, a)) = line.split_once("|") else { continue };
            let (Ok(b), Ok(a)) = (a.parse::<u32>(), b.parse::<u32>()) else { continue };
            before.entry(b).or_insert(vec![]).push(a);
            after.entry(a).or_insert(vec![]).push(b);
        }
        else {
            let pages: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            for i in 0..pages.len() {
                let page = pages[i];
                let pages_before = before.entry(page).or_default();
                let pages_after = after.entry(page).or_default();
                for j in 0..i {
                    if pages_after.contains(&pages[j]) { continue 'l  }
                }
                for j in i+1..pages.len() {
                    if pages_before.contains(&pages[j]) { continue 'l  }
                }
            }
            count += pages[pages.len() / 2];
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut before = HashMap::new();
    let mut after = HashMap::new();
    let mut pages_section = false;
    let mut count = 0;

    'l : for line in input.lines() {
        if line == "" { pages_section = true; continue }
        if !pages_section {
            let Some((b, a)) = line.split_once("|") else { continue };
            let (Ok(b), Ok(a)) = (a.parse::<u32>(), b.parse::<u32>()) else { continue };
            before.entry(b).or_insert(vec![]).push(a);
            after.entry(a).or_insert(vec![]).push(b);
        }
        else {
            let mut pages: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            let mut add = false;

            'p : for i in 0..pages.len() {
                let page = pages[i];
                let pages_before = before.entry(page).or_default();
                let pages_after = after.entry(page).or_default();
                for j in 0..i {
                    if pages_after.contains(&pages[j]) { add = true; break 'p  }
                }
                for j in i+1..pages.len() {
                    if pages_before.contains(&pages[j]) { add = true; break 'p  }
                }
            }

            if add {
                pages.sort_by(|p1, p2| {
                    if before.entry(*p1).or_default().contains(p2) { Ordering::Greater }
                    else if before.entry(*p2).or_default().contains(p1) { Ordering::Less }
                    else if after.entry(*p1).or_default().contains(p2) { Ordering::Less }
                    else if after.entry(*p2).or_default().contains(p1) { Ordering::Greater }
                    else { Ordering::Equal }
                });
                count += pages[pages.len() / 2];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
