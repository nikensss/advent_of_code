use std::{cell::RefCell, rc::Rc};

use instruction::Instruction;
use node::Node;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, newline},
    combinator::eof,
    multi::many1,
    sequence::pair,
    IResult,
};
use num::integer::lcm;

mod instruction;
mod node;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn part_1(input: &str) -> usize {
    let (_, (instructions, nodes)) = match parse_input(input) {
        Ok((input, instructions)) => (input, instructions),
        Err(e) => panic!("Error: {:?}", e),
    };

    follow_instructions(instructions, connect_nodes(nodes))
}

pub fn part_2(input: &str) -> usize {
    let (_, (instructions, nodes)) = match parse_input(input) {
        Ok((input, instructions)) => (input, instructions),
        Err(e) => panic!("Error: {:?}", e),
    };

    follow_instructions_as_ghost(instructions, connect_nodes(nodes))
}

fn connect_nodes(nodes: Vec<Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>> {
    for node in nodes.iter() {
        let left = node.borrow().get_left();
        let right = node.borrow().get_right();

        if let Some(left) = left {
            let left_id = left.borrow().get_id().to_string();
            let left_node = nodes
                .iter()
                .find(|n| n.borrow().get_id() == left_id)
                .unwrap();
            node.borrow_mut().set_left(left_node.clone());
        }

        if let Some(right) = right {
            let right_id = right.borrow().get_id().to_string();
            let right_node = nodes
                .iter()
                .find(|n| n.borrow().get_id() == right_id)
                .unwrap();
            node.borrow_mut().set_right(right_node.clone());
        }
    }

    nodes
}

fn follow_instructions(instructions: Vec<Instruction>, nodes: Vec<Rc<RefCell<Node>>>) -> usize {
    let mut node = nodes
        .iter()
        .find(|n| n.borrow().is_start())
        .unwrap()
        .clone();
    let mut steps = 0;

    loop {
        if node.borrow().is_end() {
            break;
        }

        let instruction = &instructions[steps % instructions.len()];
        let next_node = node.borrow().take(instruction);

        if next_node.is_none() {
            // being none it means that it is connected to itself, which we avoid in the data
            // structure to avoid cyclic references and we emulate connecting to itself by
            // not updating the node
            continue;
        }

        node = next_node.unwrap();
        steps += 1;
    }

    steps
}

fn follow_instructions_as_ghost(
    instructions: Vec<Instruction>,
    nodes: Vec<Rc<RefCell<Node>>>,
) -> usize {
    nodes
        .iter()
        .filter(|n| n.borrow().is_start_for_ghost())
        .map(|n| follow_instructions_as_ghost_for_node(n.clone(), &instructions))
        .fold(1, |acc, x| lcm(acc, x))
}

fn follow_instructions_as_ghost_for_node(
    node: Rc<RefCell<Node>>,
    instructions: &Vec<Instruction>,
) -> usize {
    let mut current_node = node;
    let mut steps = 0;

    loop {
        if current_node.borrow().is_end_for_ghost() {
            break;
        }
        let instruction = &instructions[steps % instructions.len()];
        let next_node = current_node.borrow().take(instruction);

        if next_node.is_none() {
            break;
        }

        current_node = next_node.unwrap();
        steps += 1;
    }

    steps
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, Vec<Rc<RefCell<Node>>>)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, nodes) = parse_node_lines(input)?;

    Ok((input, (instructions, nodes)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = take_until("\n\n")(input)?;
    let (input, _) = pair(newline, newline)(input)?;

    Ok((
        input,
        instructions
            .chars()
            .map(|c| c.to_string().parse::<Instruction>().unwrap())
            .collect(),
    ))
}

fn parse_node_lines(input: &str) -> IResult<&str, Vec<Rc<RefCell<Node>>>> {
    let (input, nodes) = many1(parse_node_line)(input)?;

    Ok((input, nodes))
}

fn parse_node_line(line: &str) -> IResult<&str, Rc<RefCell<Node>>> {
    let (input, main_node) = take_until(" ")(line)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left_node) = take_until(", ")(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right_node) = take_until(")")(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;

    let main_node = Node::new(main_node);
    let left_node = Node::new(left_node);
    let right_node = Node::new(right_node);

    main_node.borrow_mut().set_left(left_node);
    main_node.borrow_mut().set_right(right_node);

    Ok((input, main_node))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../test_input_1.txt");
    const TEST_INPUT_2: &str = include_str!("../test_input_2.txt");
    const TEST_INPUT_3: &str = include_str!("../test_input_3.txt");
    const COMPLETE_INPUT: &str = include_str!("../complete_input.txt");

    #[test]
    fn test_parse_instructions() {
        let input = "RL\n\n";
        let (input, instructions) = parse_instructions(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(instructions, vec![Instruction::Right, Instruction::Left]);

        let input = "LLR\n\n";
        let (input, instructions) = parse_instructions(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            instructions,
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );
    }

    #[test]
    fn test_parse_line() {
        let line = "AAA = (BBB, CCC)";
        let (input, node) = parse_node_line(line).unwrap();
        assert_eq!(input, "");
        assert_eq!(node.borrow().get_id(), "AAA");
        assert_eq!(node.borrow().get_left().unwrap().borrow().get_id(), "BBB");
        assert_eq!(node.borrow().get_right().unwrap().borrow().get_id(), "CCC");

        let line = "BBB = (DDD, BBB)";
        let (input, node) = parse_node_line(line).unwrap();
        assert_eq!(input, "");
        assert_eq!(node.borrow().get_id(), "BBB");
        assert_eq!(node.borrow().get_left().unwrap().borrow().get_id(), "DDD");
        assert!(node.borrow().get_right().is_none());
    }

    #[test]
    fn test_parse_node_lines() {
        let input = "AAA = (BBB, CCC)\nBBB = (DDD, BBB)\n";
        let (input, nodes) = parse_node_lines(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].borrow().get_id(), "AAA");
        assert_eq!(
            nodes[0]
                .borrow()
                .take(&Instruction::Left)
                .unwrap()
                .borrow()
                .get_id(),
            "BBB"
        );
        assert_eq!(
            nodes[0]
                .borrow()
                .take(&Instruction::Right)
                .unwrap()
                .borrow()
                .get_id(),
            "CCC"
        );
        assert_eq!(nodes[1].borrow().get_id(), "BBB");
        assert_eq!(
            nodes[1].borrow().get_left().unwrap().borrow().get_id(),
            "DDD"
        );
        assert!(nodes[1].borrow().get_right().is_none());
    }

    #[test]
    fn test_parse_input() {
        let (input, (instructions, nodes)) = parse_input(TEST_INPUT_1).unwrap();
        assert_eq!(input, "");
        assert_eq!(instructions, vec![Instruction::Right, Instruction::Left]);
        assert_eq!(nodes.len(), 7);
        assert_eq!(nodes[0].borrow().get_id(), "AAA");
        assert_eq!(
            nodes[0].borrow().get_left().unwrap().borrow().get_id(),
            "BBB"
        );
        assert_eq!(
            nodes[0].borrow().get_right().unwrap().borrow().get_id(),
            "CCC"
        );
        assert_eq!(nodes[1].borrow().get_id(), "BBB");
        assert_eq!(
            nodes[1].borrow().get_left().unwrap().borrow().get_id(),
            "DDD"
        );
        assert_eq!(
            nodes[1].borrow().get_right().unwrap().borrow().get_id(),
            "EEE"
        );
        assert_eq!(nodes[6].borrow().get_id(), "ZZZ");
        assert!(nodes[6].borrow().get_left().is_none());
        assert!(nodes[6].borrow().get_right().is_none());

        let (input, (instructions, nodes)) = parse_input(TEST_INPUT_2).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            instructions,
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].borrow().get_id(), "AAA");
        assert_eq!(
            nodes[0].borrow().get_left().unwrap().borrow().get_id(),
            "BBB"
        );
        assert_eq!(
            nodes[0].borrow().get_right().unwrap().borrow().get_id(),
            "BBB"
        );
        assert_eq!(nodes[1].borrow().get_id(), "BBB");
        assert_eq!(
            nodes[1].borrow().get_left().unwrap().borrow().get_id(),
            "AAA"
        );
        assert_eq!(
            nodes[1].borrow().get_right().unwrap().borrow().get_id(),
            "ZZZ"
        );
        assert_eq!(nodes[2].borrow().get_id(), "ZZZ");
        assert!(nodes[2].borrow().get_left().is_none());
        assert!(nodes[2].borrow().get_right().is_none());
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT_1), 2);
        assert_eq!(part_1(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), 18673);
    }

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT_3), 6);
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT), 17_972_669_116_327);
    }
}
