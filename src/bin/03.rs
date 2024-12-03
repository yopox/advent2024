use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new("mul\\((\\d+,\\d+)\\)").unwrap();
    let sum = input.lines().map(|line| {
        let mut i = 0;
        for m in pattern.find_iter(line) {
            let s = &m.as_str()[4..m.as_str().len()-1];
            let Some((n1, n2)) = s.split_once(",") else { continue };
            let (Ok(n1), Ok(n2)) = (n1.parse::<u32>(), n2.parse::<u32>()) else { continue };
            i += n1 * n2;
        }
        i
    })
        .sum();
    Some(sum)
}

enum Command {
    Mul(usize, String),
    Do(usize),
    Dont(usize),
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut commands = vec![];

    let mul_p = Regex::new("mul\\((\\d+,\\d+)\\)").unwrap();
    let do_p = Regex::new("do\\(\\)").unwrap();
    let dont_p = Regex::new("don't\\(\\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    input.lines().for_each(|line| {
        mul_p.find_iter(line).for_each(|m| {
            commands.push(Command::Mul(m.start(), m.as_str()[4..m.as_str().len()-1].to_string()));
        });
        do_p.find_iter(line).for_each(|m| {
            commands.push(Command::Do(m.start()));
        });
        dont_p.find_iter(line).for_each(|m| {
            commands.push(Command::Dont(m.start()));
        });

        commands.sort_by_key(|command| {
            match command {
                &Command::Mul(o, _) => o,
                &Command::Do(o) => o,
                &Command::Dont(o) => o,
            }
        });

        for command in &commands {
            match command {
                Command::Mul(_, s) => { if enabled {
                    let Some((n1, n2)) = s.split_once(",") else { continue };
                    let (Ok(n1), Ok(n2)) = (n1.parse::<u32>(), n2.parse::<u32>()) else { continue };
                    sum += n1 * n2;
                }}
                Command::Do(_) => { enabled = true; }
                Command::Dont(_) => { enabled = false; }
            }
        }

        commands.clear();
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
