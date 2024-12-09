advent_of_code::solution!(9);

#[derive(Debug)]
enum Block {
    File(u32),
    Empty,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = vec![];
    let mut empty_pos = vec![];
    let mut id = 0;

    let mut file = true;
    for char in input.chars() {
        let Some(i) = char.to_digit(10) else { continue };
        for _ in 0..i {
            if file {
                blocks.push(Block::File(id));
            } else {
                blocks.push(Block::Empty);
                empty_pos.push(blocks.len() - 1);
            }
        }
        if file { id += 1; }
        file = !file;
    }

    for i in (0..blocks.len()).rev() {
        if empty_pos.is_empty() || i < empty_pos[0] { break }
        let Block::File(n) = blocks[i] else { continue };
        let first_empty = empty_pos.remove(0);
        blocks[first_empty] = Block::File(n);
        blocks[i] = Block::Empty;
    }

    Some(checksum(blocks))
}

fn checksum(blocks: Vec<Block>) -> usize {
    blocks.iter().enumerate()
        .map(|(i, b)| match b {
            Block::File(n) => i * *n as usize,
            Block::Empty => 0,
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks = vec![];
    let mut empty = vec![];
    let mut index = 0;
    let mut id = 0;

    let mut file = true;
    for char in input.chars() {
        let Some(i) = char.to_digit(10) else { continue };
        if file {
            blocks.push((Block::File(id), i, index));
        } else {
            empty.push((i, index));
        }
        if file { id += 1; }
        file = !file;
        index += i;
    }

    'defrag: for i in (0..blocks.len()).rev() {
        let (_, len, pos) = blocks[i];
        for ((empty_len, empty_pos)) in empty.iter_mut() {
            if *empty_pos > pos { continue 'defrag }
            if *empty_len >= len {
                *empty_len -= len;
                blocks[i].2 = *empty_pos;
                *empty_pos += len;
                continue 'defrag
            }
        }
    }

    Some(checksum2(blocks))
}

fn checksum2(blocks: Vec<(Block, u32, u32)>) -> usize {
    blocks.iter()
        .map(|(b, len, pos)| match b {
            Block::File(n) => (*pos..*pos+len).map(|i| (i * n) as usize).sum(),
            Block::Empty => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
