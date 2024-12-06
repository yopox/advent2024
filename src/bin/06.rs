use std::cmp::{max, PartialEq};
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir { UP, RIGHT, DOWN, LEFT }

impl Dir {
    pub fn right(&self) -> Self {
        match self {
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
        }
    }

    pub fn diff(&self) -> (i32, i32) {
        match self {
            Dir::UP => (-1, 0),
            Dir::RIGHT => (0, 1),
            Dir::DOWN => (1, 0),
            Dir::LEFT => (0, -1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut y_max, mut x_max) = (0, 0);
    let mut pos = (0, 0);
    let mut obstacles = HashSet::new();

    parse_input(input, &mut y_max, &mut x_max, &mut pos, &mut obstacles);

    let mut visited = HashSet::new();
    visited.insert(pos);
    get_path(&mut y_max, &mut x_max, pos, &mut obstacles, &mut visited);

    Some(visited.len() as u32)
}

fn parse_input(input: &str, y_max: &mut i32, x_max: &mut i32, pos: &mut (i32, i32), obstacles: &mut HashSet<(i32, i32)>) {
    for (y, line) in input.lines().enumerate() {
        *y_max = max(*y_max, y as i32);
        for (x, char) in line.chars().enumerate() {
            *x_max = max(*x_max, x as i32);
            match char {
                '#' => { obstacles.insert((y as i32, x as i32)); }
                '^' => { *pos = (y as i32, x as i32); }
                '.' | _ => {}
            }
        }
    }
}

fn get_path(y_max: &mut i32, x_max: &mut i32, start_pos: (i32, i32), obstacles: &mut HashSet<(i32, i32)>, visited: &mut HashSet<(i32, i32)>) {
    let mut dir = Dir::UP;
    let mut pos = (start_pos.0, start_pos.1);

    while !(
        dir == Dir::UP && pos.0 == 0 ||
            dir == Dir::RIGHT && pos.1 == *x_max ||
            dir == Dir::DOWN && pos.0 == *y_max ||
            dir == Dir::LEFT && pos.1 == 0
    ) {
        let diff = dir.diff();
        let new_pos = (pos.0 + diff.0, pos.1 + diff.1);
        if obstacles.contains(&new_pos) {
            dir = dir.right();
        } else {
            pos = new_pos;
            visited.insert(new_pos);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut y_max, mut x_max) = (0, 0);
    let mut start_pos = (0, 0);
    let mut obstacles = HashSet::new();

    parse_input(input, &mut y_max, &mut x_max, &mut start_pos, &mut obstacles);

    let mut visited = HashSet::new();
    visited.insert(start_pos);
    get_path(&mut y_max, &mut x_max, start_pos, &mut obstacles, &mut visited);

    Some(visited.iter().filter(|&p| {
        if *p == start_pos { false }
        else {
            let mut o2 = obstacles.clone();
            o2.insert(*p);
            check_loop(start_pos, x_max, y_max, &o2)
        }
    }).count() as u32)
}

fn check_loop(start: (i32, i32), x_max: i32, y_max: i32, obstacles: &HashSet<(i32, i32)>) -> bool {
    let mut pos = start;
    let mut visited = HashSet::new();
    let mut dir = Dir::UP;

    while !(
        dir == Dir::UP && pos.0 == 0 ||
        dir == Dir::RIGHT && pos.1 == x_max ||
        dir == Dir::DOWN && pos.0 == y_max ||
        dir == Dir::LEFT && pos.1 == 0
    ) {
        if visited.contains(&(pos, dir)) { return true }
        visited.insert((pos, dir));
        let diff = dir.diff();
        let new_pos = (pos.0 + diff.0, pos.1 + diff.1);
        if obstacles.contains(&new_pos) {
            dir = dir.right();
        } else {
            pos = new_pos;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
