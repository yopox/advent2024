use memoize::memoize;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones: Vec<usize> = input.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
    for _ in 0..25 { stones = blink(&stones); }
    Some(stones.len())
}

fn blink(stones: &Vec<usize>) -> Vec<usize> {
    let mut s = vec![];
    stones.iter()
        .for_each(|stone| {
            if *stone == 0 { s.push(1); }
            else if (stone.ilog10() + 1) % 2 == 0 {
                let (l, r) = split(*stone);
                s.push(l); s.push(r);
            }
            else { s.push(*stone * 2024); }
        });
    s
}

fn split(number: usize) -> (usize, usize) {
    let str = number.to_string();
    let (l, r) = str.split_at((number.ilog10() + 1) as usize / 2);
    (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones: Vec<usize> = input.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
    Some(stones.iter().map(|stone| count(0, *stone)).sum())
}

#[memoize]
fn count(step: usize, number: usize) -> usize {
    if step == 75 { return 1; }
    if number == 0 {
        count(step + 1, 1)
    } else if (number.ilog10() + 1) % 2 == 0 {
        let (l, r) = split(number);
        count(step + 1, l) + count(step + 1, r)
    } else {
        count(step + 1, number * 2024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
