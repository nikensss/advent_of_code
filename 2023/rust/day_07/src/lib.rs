mod card;
mod hand;
mod hand_type;

use std::cmp::Ordering;

use crate::card::Card;
use hand::Hand;
use nom::{
    character::complete::{alphanumeric1, line_ending, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn part_1(input: &str) -> usize {
    let (_, mut hands) = parse_input(input).unwrap();

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.get_bid())
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (_, mut hands) = parse_input(input).unwrap();

    hands.sort_by(|a, b| {
        if a.is_stronger_than_with_joker(&b) {
            return Ordering::Greater;
        }

        if b.is_stronger_than_with_joker(&a) {
            return Ordering::Less;
        }

        return Ordering::Equal;
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.get_bid())
        .sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(alphanumeric1, space1, u64)(input)?;

    let cards = cards
        .chars()
        .map(|c| c.to_string().parse::<Card>().unwrap())
        .collect();

    let bid = bid.try_into().unwrap();

    let hand = Hand::new(cards, bid);

    Ok((input, hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");
    const COMPLETE_INPUT: &str = include_str!("../complete_input.txt");

    #[test]
    fn test_parse_line() {
        let input = "32T3K 765";
        let hand = Hand::new(
            vec![
                Card::new("3").unwrap(),
                Card::new("2").unwrap(),
                Card::new("T").unwrap(),
                Card::new("3").unwrap(),
                Card::new("K").unwrap(),
            ],
            765,
        );

        assert_eq!(parse_line(input).unwrap().1, hand);
    }

    #[test]
    fn test_parse_input() {
        match parse_input(TEST_INPUT) {
            Ok((_, hands)) => {
                assert_eq!(hands.len(), 5);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), 247815719);
    }

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT), 5905);
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT), 248747492);
    }
}
