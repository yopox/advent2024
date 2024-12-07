advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut s = 0;

    for line in input.lines() {
        let Some((target, numbers)) = line.split_once(": ") else { continue };
        let Ok(target) = target.parse::<usize>() else { continue };
        let mut numbers = numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        if solve(numbers[0], &numbers[1..], target) { s += target };
    }

    Some(s)
}

fn solve(acc: usize, numbers: &[usize], target: usize) -> bool {
    if acc > target { return false }
    match numbers {
        [] if { acc == target } => true,
        [] => false,
        [n, rest @ ..] => solve(acc + *n, rest, target) || solve(acc * *n, rest, target)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut s = 0;

    for line in input.lines() {
        let Some((target, numbers)) = line.split_once(": ") else { continue };
        let Ok(target) = target.parse::<usize>() else { continue };
        let mut numbers = numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        if solve2(numbers[0], &numbers[1..], target) { s += target };
    }

    Some(s)
}

fn solve2(acc: usize, numbers: &[usize], target: usize) -> bool {
    if acc > target { return false }
    match numbers {
        [] if { acc == target } => true,
        [] => false,
        [n, rest @ ..] => solve2(acc + *n, rest, target)
            || solve2(acc * *n, rest, target)
            || solve2(acc * 10_usize.pow(n.ilog10() + 1) + *n, rest, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
