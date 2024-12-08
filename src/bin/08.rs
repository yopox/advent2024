use std::cmp::max;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let (mut y_max, mut x_max) = (0, 0);
    let mut antennas = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        y_max = max(y_max, y as i32);
        for (x, char) in line.chars().enumerate() {
            x_max = max(x_max, x as i32);
            match char {
                '.' => {}
                c => {
                    antennas.entry(c).or_insert_with(Vec::new).push((y as i32, x as i32));
                }
            }
        }
    }

    (antennas, y_max, x_max)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas, y_max, x_max) = parse_input(input);
    let mut antinodes = HashSet::new();

    for freq_antennas in antennas.values() {
        for i in 0..freq_antennas.len() {
            for j in i+1..freq_antennas.len() {
                let (a1_y, a1_x) = freq_antennas[i];
                let (a2_y, a2_x) = freq_antennas[j];
                let dx = a2_x - a1_x;
                let dy = a2_y - a1_y;
                let mut anti = vec![];
                if dy % 3 == 0 && dx % 3 == 0 {
                    anti.push((a1_y + dy / 3, a1_x + dx / 3));
                    anti.push((a1_y + 2 * dy / 3, a1_x + 2 * dx / 3));
                }
                anti.push((a1_y - dy, a1_x - dx));
                anti.push((a2_y + dy, a2_x + dx));
                for (anti_pos_y, anti_pos_x) in anti.into_iter() {
                    if anti_pos_y >= 0 && anti_pos_y <= y_max && anti_pos_x >= 0 && anti_pos_x <= x_max {
                        antinodes.insert((anti_pos_y as usize, anti_pos_x as usize));
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas, y_max, x_max) = parse_input(input);
    let mut antinodes = HashSet::new();

    for freq_antennas in antennas.values() {
        for i in 0..freq_antennas.len() {
            for j in i + 1..freq_antennas.len() {
                let (a1_y, a1_x) = freq_antennas[i];
                let (a2_y, a2_x) = freq_antennas[j];
                
                let dx = a2_x - a1_x;
                let dy = a2_y - a1_y;
                let dist = max(dx.abs(), dy.abs());
                
                for s in (-2 * x_max / dist)..(2 * x_max / dist) {
                    for i in 0..dist {
                        if (i * dx) % dist == 0 && (i * dy) % dist == 0 {
                            let anti_y = a1_y + dy * s + i * dy / dist;
                            let anti_x = a1_x + dx * s + i * dx / dist;
                            if anti_y >= 0 && anti_y <= y_max && anti_x >= 0 && anti_x <= x_max {
                                antinodes.insert((anti_y as usize, anti_x as usize));
                            }
                        }
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
