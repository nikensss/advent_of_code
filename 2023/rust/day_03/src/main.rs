use std::{fs, num::ParseIntError};

fn main() {
    let lines = fs::read_to_string("test-input-01.txt").unwrap();
    let lines = lines.split("\n").collect::<Vec<&str>>();

    let sum = part_01(&lines).unwrap();
    println!("Sum: {}", sum);

    let sum = part_02(&lines).unwrap();
    println!("Sum: {}", sum);
}

fn part_01(lines: &Vec<&str>) -> Result<isize, ParseIntError> {
    let blocks = get_blocks(lines);

    let number_blocks_adjacent_to_symbols = blocks
        .iter()
        .filter(|b| b.r#type == BlockType::Number)
        .filter(|b| b.touches_symbol(&blocks))
        .collect::<Vec<&Block>>();

    Ok(number_blocks_adjacent_to_symbols
        .iter()
        .map(|b| b.value.parse::<isize>().unwrap())
        .sum::<isize>())
}

fn part_02(lines: &Vec<&str>) -> Result<isize, ParseIntError> {
    let blocks = get_blocks(lines);
    Ok(blocks
        .iter()
        .filter(|b| b.is_gear(&blocks))
        .map(|b| b.get_adjacent_numbers(&blocks))
        .map(|b| {
            b.iter()
                .map(|b| b.value.parse::<isize>().unwrap())
                .product::<isize>()
        })
        .sum::<isize>())
}

fn get_blocks(lines: &Vec<&str>) -> Vec<Block> {
    let mut blocks = Vec::new();

    for (line_index, line_contents) in lines.iter().enumerate() {
        blocks.append(&mut get_line_blocks(line_contents, line_index));
    }

    blocks
}

fn get_line_blocks(line: &str, line_index: usize) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut i = 0;
    while i < line.len() {
        let character = line.chars().nth(i).unwrap();

        let block = if character.is_digit(10) {
            Block {
                line: line_index as isize,
                offset: i as isize,
                value: get_number(line, i).unwrap(),
                r#type: BlockType::Number,
            }
        } else if character == '.' {
            Block {
                line: line_index as isize,
                offset: i as isize,
                value: get_dots(line, i).unwrap(),
                r#type: BlockType::Dot,
            }
        } else {
            Block {
                line: line_index as isize,
                offset: i as isize,
                value: get_symbol(line, i).unwrap(),
                r#type: BlockType::Symbol,
            }
        };

        i = i + block.len() as usize;
        blocks.push(block);
    }

    blocks
}

fn get_number(line: &str, offset: usize) -> Option<String> {
    get_sequence(line, offset, |c| c.is_digit(10))
}

fn get_dots(line: &str, offset: usize) -> Option<String> {
    get_sequence(line, offset, |c| c == '.')
}

fn get_symbol(line: &str, offset: usize) -> Option<String> {
    get_sequence(line, offset, |c| !c.is_digit(10) && c != '.')
}

fn get_sequence<F>(line: &str, offset: usize, predicate: F) -> Option<String>
where
    F: Fn(char) -> bool,
{
    let mut sequence = String::from("");

    for character in line[offset..].chars() {
        if predicate(character) {
            sequence.push(character);
        } else {
            break;
        }
    }

    if sequence.len() == 0 {
        return None;
    }

    Some(sequence)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum BlockType {
    Number,
    Dot,
    Symbol,
}

#[derive(Debug, Clone)]
struct Block {
    value: String,
    line: isize,
    offset: isize,
    r#type: BlockType,
}

impl Block {
    fn len(&self) -> isize {
        self.value.len() as isize
    }

    fn is_gear(&self, blocks: &Vec<Block>) -> bool {
        if self.value != "*" {
            return false;
        }

        self.has_exactly_two_adjacent_numbers(blocks)
    }

    fn get_adjacent_numbers(&self, blocks: &Vec<Block>) -> Vec<Block> {
        blocks
            .iter()
            .filter(|b| b.r#type == BlockType::Number)
            .filter(|b| b.line == self.line - 1 || b.line == self.line || b.line == self.line + 1)
            .filter(|b| b.offset + b.len() >= self.offset && b.offset <= self.offset + self.len())
            .map(|b| b.clone())
            .collect::<Vec<Block>>()
    }

    fn count_adjacent_numbers(&self, blocks: &Vec<Block>) -> usize {
        self.get_adjacent_numbers(blocks).len()
    }

    fn has_exactly_two_adjacent_numbers(&self, blocks: &Vec<Block>) -> bool {
        self.count_adjacent_numbers(blocks) == 2
    }

    fn touches_symbol(&self, blocks: &Vec<Block>) -> bool {
        blocks
            .iter()
            .filter(|b| b.r#type == BlockType::Symbol)
            .filter(|b| b.line == self.line - 1 || b.line == self.line || b.line == self.line + 1)
            .any(|b| b.offset + b.len() >= self.offset && b.offset <= self.offset + self.len())
    }
}

#[test]
fn test_part_01_test_input() {
    let lines = fs::read_to_string("test-input-01.txt").unwrap();
    let lines = lines.split("\n").collect::<Vec<&str>>();

    let sum = part_01(&lines).unwrap();
    assert_eq!(sum, 4361);
}

#[test]
fn test_part_01_complete_input() {
    let lines = fs::read_to_string("input-01.txt").unwrap();
    let lines = lines.split("\n").collect::<Vec<&str>>();

    let sum = part_01(&lines).unwrap();

    assert_eq!(sum, 556057);
}

#[test]
fn test_part_02_test_input() {
    let lines = fs::read_to_string("test-input-01.txt").unwrap();
    let lines = lines.split("\n").collect::<Vec<&str>>();

    let sum = part_02(&lines).unwrap();
    assert_eq!(sum, 467835);
}

#[test]
fn test_part_02_complete_input() {
    let lines = fs::read_to_string("input-01.txt").unwrap();
    let lines = lines.split("\n").collect::<Vec<&str>>();

    let sum = part_02(&lines).unwrap();

    assert_eq!(sum, 82824352);
}
