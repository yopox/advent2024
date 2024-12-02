advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let values: Vec<u32> = line.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect();
            let asc = check_asc(&values, None);
            let desc = !asc && check_desc(&values, None);
            if asc || desc { 1 } else { 0 }
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            let values: Vec<u32> = line.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect();
            let asc = (0..values.len()).any(|i| check_asc(&values, Some(i)));
            let desc = !asc && (0..values.len()).any(|i| check_desc(&values, Some(i)));
            if asc || desc { 1 } else { 0 }
        })
        .sum())
}

fn check_asc(values: &[u32], skip: Option<usize>) -> bool {
    let mut v = values.to_vec();
    if let Some(s) = skip { v.remove(s); }
    let mut asc = true;
    v.iter().reduce(|e1, e2| {
        asc = asc && e1 < e2 && e2 - e1 <= 3;
        e2
    });
    asc
}

fn check_desc(values: &[u32], skip: Option<usize>) -> bool {
    let mut v = values.to_vec();
    if let Some(s) = skip { v.remove(s); }
    let mut desc = true;
    v.iter().reduce(|e1, e2| {
        desc = desc && e1 > e2 && e1 - e2 <= 3;
        e2
    });
    desc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
