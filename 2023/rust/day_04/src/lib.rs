use std::collections::{HashMap, HashSet};

use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1, u32};
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::IResult;

pub fn part_1(input: &str) -> usize {
    let cards = get_cards(input);
    let points: usize = cards.iter().map(|c| c.get_points()).sum();

    points
}

pub fn part_2(input: &str) -> usize {
    let cards = get_cards(input);
    let mut card_counter = CardCounter::new(&cards);

    for card in &cards {
        card_counter.process_card(card);
    }

    card_counter.get_total_count()
}

fn get_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let parse_line_result = parse_line(line);

            match parse_line_result {
                Ok((_, card)) => match card {
                    Ok(card) => card,
                    Err(message) => panic!("could not build card: {}", message),
                },
                Err(message) => panic!("could not parse line: {}", message),
            }
        })
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Result<Card, String>> {
    let (input, id) = pase_card_id(input)?;
    let mut card_builder = CardBuilder::new(id as usize);
    let (input, _) = tag(": ")(input)?;
    let (input, winning_numbers) = parse_winning_numbers(input)?;
    card_builder.add_winning_numbers(winning_numbers);
    let (input, received_numbers) = parse_received_numbers(input)?;
    card_builder.add_received_numbers(received_numbers);

    Ok((input, card_builder.build()))
}

fn pase_card_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = preceded(tag("Card"), multispace1)(input)?;
    let (input, id) = u32(input)?;

    Ok((input, id as usize))
}

fn parse_winning_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, winning_numbers) = parse_numbers(input)?;

    Ok((input, winning_numbers))
}

fn parse_received_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag(" | ")(input)?;
    let (input, received_numbers) = parse_numbers(input)?;

    Ok((input, received_numbers))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, numbers) = preceded(multispace0, separated_list0(multispace1, u32))(input)?;
    let numbers = numbers.into_iter().map(|x| x as usize).collect();

    Ok((input, numbers))
}

#[derive(Debug)]
pub struct CardBuilder {
    id: usize,
    winning_numbers: HashSet<usize>,
    received_numbers: HashSet<usize>,
}

impl CardBuilder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            winning_numbers: HashSet::new(),
            received_numbers: HashSet::new(),
        }
    }

    pub fn add_winning_number(&mut self, number: usize) {
        self.winning_numbers.insert(number);
    }

    pub fn add_winning_numbers(&mut self, numbers: Vec<usize>) {
        self.winning_numbers.extend(numbers);
    }

    pub fn add_received_number(&mut self, number: usize) {
        self.received_numbers.insert(number);
    }

    pub fn add_received_numbers(&mut self, numbers: Vec<usize>) {
        self.received_numbers.extend(numbers);
    }

    pub fn build(self) -> Result<Card, String> {
        if self.winning_numbers.len() != 5 && self.winning_numbers.len() != 10 {
            return Err(format!(
                "there should be 5 or 10 winning numbers but there are {}",
                self.winning_numbers.len()
            ));
        }

        if self.received_numbers.len() != 8 && self.received_numbers.len() != 25 {
            return Err(format!(
                "there should be 8 or 25 received numbers but there are {}",
                self.received_numbers.len(),
            ));
        }

        Ok(Card {
            id: self.id,
            winning_numbers: self.winning_numbers,
            received_numbers: self.received_numbers,
        })
    }
}

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    received_numbers: HashSet<usize>,
}

impl Card {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_total_matching_numbers(&self) -> usize {
        let owned_winning_nubmers = self.winning_numbers.intersection(&self.received_numbers);
        owned_winning_nubmers.count()
    }

    pub fn get_points(&self) -> usize {
        let owned_winning_nubmers_count = self.get_total_matching_numbers();

        if owned_winning_nubmers_count == 0 {
            return 0;
        }

        let base: usize = 2;
        base.pow(owned_winning_nubmers_count as u32 - 1)
    }
}

struct CardCount {
    count: usize,
}

impl CardCount {
    fn new() -> Self {
        Self { count: 1 }
    }

    fn increase_count_by(&mut self, amount: usize) {
        self.count += amount;
    }
}

struct CardCounter {
    counts: HashMap<usize, CardCount>,
}

impl CardCounter {
    fn new(cards: &Vec<Card>) -> Self {
        let counts = cards
            .iter()
            .map(|card| (card.get_id(), CardCount::new()))
            .collect();

        Self { counts }
    }

    fn get_total_count(&self) -> usize {
        let mut total = 0;
        for card_count in self.counts.values() {
            total += card_count.count;
        }
        total
    }

    fn get_count(&self, id: usize) -> usize {
        if let Some(card_count) = self.counts.get(&id) {
            card_count.count
        } else {
            0
        }
    }

    fn process_card(&mut self, card: &Card) {
        let id = card.get_id();
        let total_matching_numbers = card.get_total_matching_numbers();

        if total_matching_numbers == 0 {
            return;
        }

        for n in (id + 1)..=(id + total_matching_numbers) {
            self.increase_count(n, self.get_count(id));
        }
    }

    fn increase_count(&mut self, id: usize, amount: usize) {
        if let Some(card_count) = self.counts.get_mut(&id) {
            card_count.increase_count_by(amount);
        } else {
            self.counts.insert(id, CardCount::new());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUT_01: &str = include_str!("../test-input-01.txt");
    const COMPLETE_INPUT_01: &str = include_str!("../input-01.txt");

    #[test]
    fn test_card_builder_adds_one_winning_number() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_number(1);

        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![1].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 1"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_adds_multiple_winning_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3]);

        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 3"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_does_not_add_duplicate_winning_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3]);

        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        card_builder.add_winning_number(1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        card_builder.add_winning_numbers(vec![2, 3]);
        assert_eq!(
            card_builder.winning_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 3"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_adds_one_received_number() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_received_number(1);
        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![1].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 0"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_adds_multiple_received_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_received_numbers(vec![1, 2, 3]);
        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 0"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_does_not_add_duplicate_received_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_received_numbers(vec![1, 2, 3]);
        assert_eq!(card_builder.id, 1);
        assert_eq!(
            card_builder.winning_numbers,
            vec![].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card_builder.received_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        card_builder.add_received_number(1);
        assert_eq!(
            card_builder.received_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );
        card_builder.add_received_numbers(vec![2, 3]);
        assert_eq!(
            card_builder.received_numbers,
            vec![1, 2, 3].into_iter().collect::<HashSet<usize>>()
        );

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 0"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_does_not_build_card_with_wrong_winning_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4]);
        card_builder.add_received_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 5 or 10 winning numbers but there are 4"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_does_not_build_card_with_wrong_received_numbers() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![1, 2, 3, 4, 5, 6, 7]);

        let card = card_builder.build();
        match card {
            Ok(_) => panic!("card should not be built"),
            Err(message) => {
                assert_eq!(
                    message,
                    "there should be 8 or 25 received numbers but there are 7"
                )
            }
        }
    }

    #[test]
    fn test_card_builder_builds_card() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.id, 1);
        assert_eq!(
            card.winning_numbers,
            vec![1, 2, 3, 4, 5].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(
            card.received_numbers,
            vec![1, 2, 3, 4, 5, 6, 7, 8]
                .into_iter()
                .collect::<HashSet<usize>>()
        );
    }

    #[test]
    fn test_card_points() {
        let mut card_builder = CardBuilder::new(1);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 16);

        let mut card_builder = CardBuilder::new(2);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![2, 3, 4, 5, 6, 7, 8, 9]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 8);

        let mut card_builder = CardBuilder::new(3);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![3, 4, 5, 6, 7, 8, 9, 10]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 4);

        let mut card_builder = CardBuilder::new(4);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![4, 5, 6, 7, 8, 9, 10, 11]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 2);

        let mut card_builder = CardBuilder::new(5);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![5, 6, 7, 8, 9, 10, 11, 12]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 1);

        let mut card_builder = CardBuilder::new(6);
        card_builder.add_winning_numbers(vec![1, 2, 3, 4, 5]);
        card_builder.add_received_numbers(vec![6, 7, 8, 9, 10, 11, 12, 13]);
        let card = card_builder.build();
        assert!(card.is_ok());
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not build card: {}", message),
        };
        assert_eq!(card.get_points(), 0);
    }

    #[test]
    fn test_parse_card_id_1() {
        let input = "Card 1: ";
        let (_, card_id) = pase_card_id(input).unwrap();
        assert_eq!(card_id, 1);
    }

    #[test]
    fn test_parse_card_id_2() {
        let input = "Card  1: ";
        let (_, card_id) = pase_card_id(input).unwrap();
        assert_eq!(card_id, 1);
    }

    #[test]
    fn test_parse_winning_numbers_1() {
        let input = "1 2 3 4 5 | 1 2 3 4 5 6 7 8";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " | 1 2 3 4 5 6 7 8");
        assert_eq!(winning_numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_winning_numbers_2() {
        let input = " 1 2 3 4 5 | 1 2 3 4 5 6 7 8";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " | 1 2 3 4 5 6 7 8");
        assert_eq!(winning_numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_winning_numbers_3() {
        let input = " 1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " | 69 82 63 72 16 21 14  1");
        assert_eq!(winning_numbers, vec![1, 21, 53, 59, 44]);
    }

    #[test]
    fn test_parse_winning_numbers_and_received_numbers_1() {
        let input = "1 2 3 4 5 | 1 2 3 4 5 6 7 8";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " | 1 2 3 4 5 6 7 8");
        assert_eq!(winning_numbers, vec![1, 2, 3, 4, 5]);
        let (input, received_numbers) = parse_received_numbers(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(received_numbers, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_parse_winning_numbers_and_received_numbers_2() {
        let input = " 1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " | 69 82 63 72 16 21 14  1");
        assert_eq!(winning_numbers, vec![1, 21, 53, 59, 44]);
        let (input, received_numbers) = parse_received_numbers(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(received_numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[test]
    fn test_parse_winning_numbers_and_received_numbers_3() {
        let input = " 1 21 53 59 44 |  9 82 63 72 16 21 14  1";
        let (input, winning_numbers) = parse_winning_numbers(input).unwrap();
        assert_eq!(input, " |  9 82 63 72 16 21 14  1");
        assert_eq!(winning_numbers, vec![1, 21, 53, 59, 44]);
        let (input, received_numbers) = parse_received_numbers(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(received_numbers, vec![9, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[test]
    fn test_parse_line_1() {
        let input = "Card 1: 1 2 3 4 5 | 1 2 3 4 5 6 7 8";
        let (_, card) = parse_line(input).unwrap();
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not parse line: {}", message),
        };
        assert_eq!(card.get_id(), 1);
        assert_eq!(card.get_points(), 16);
    }

    #[test]
    fn test_parse_line_2() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14 22";
        let (_, card) = parse_line(input).unwrap();
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not parse line: {}", message),
        };
        assert_eq!(card.get_id(), 3);
        assert_eq!(card.get_points(), 1);
    }

    #[test]
    fn test_parse_line_3() {
        let input = "Card 3: 11 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (_, card) = parse_line(input).unwrap();
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not parse line: {}", message),
        };
        assert_eq!(card.get_id(), 3);
        assert_eq!(card.get_points(), 1);
    }

    #[test]
    fn test_parse_line_4() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (_, card) = parse_line(input).unwrap();
        let card = match card {
            Ok(card) => card,
            Err(message) => panic!("could not parse line: {}", message),
        };
        assert_eq!(card.get_id(), 3);
        assert_eq!(card.get_points(), 2);
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT_01), 13);
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT_01), 23028);
    }

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT_01), 30);
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT_01), 9236992);
    }
}
