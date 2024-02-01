use crate::{card::Card, hand_type::HandType};

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        Hand { cards, bid }
    }

    pub fn is_stronger_than(&self, other: &Hand) -> bool {
        let self_hand_type = self.get_hand_type();
        let other_hand_type = other.get_hand_type();

        if self_hand_type.is_stronger_than(&other_hand_type) {
            return true;
        }

        if other_hand_type.is_stronger_than(&self_hand_type) {
            return false;
        }

        let cards = self.cards.iter().zip(other.cards.iter());
        for (self_card, other_card) in cards {
            if self_card.is_stronger_than(other_card) {
                return true;
            }
            if other_card.is_stronger_than(self_card) {
                return false;
            }
        }

        false
    }

    pub fn get_hand_type(&self) -> HandType {
        let mut card_type_count: HashMap<&Card, usize> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_type_count.entry(card).or_insert(0);
            *count += 1;
        }

        let keys = card_type_count.keys().len();
        let mut values = card_type_count.into_values().collect::<Vec<usize>>();
        values.sort_by(|a, b| b.cmp(a));

        if keys == 1 && values.len() == 1 && values[0] == 5 {
            return HandType::FiveOfAKind;
        }

        if keys == 2 && values.len() == 2 && values[0] == 4 {
            return HandType::FourOfAKind;
        }

        if keys == 2 && values.len() == 2 && values[0] == 3 && values[1] == 2 {
            return HandType::FullHouse;
        }

        if keys == 3 && values.len() == 3 && values[0] == 3 && values[1] == 1 && values[2] == 1 {
            return HandType::ThreeOfAKind;
        }

        if keys == 3 && values.len() == 3 && values[0] == 2 && values[1] == 2 && values[2] == 1 {
            return HandType::TwoPair;
        }

        if keys == 4 && values[0] == 2 && values[1] == 1 && values[2] == 1 && values[3] == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    pub fn is_stronger_than_with_joker(&self, other: &Hand) -> bool {
        let self_hand_type = self.get_hand_type_with_joker();
        let other_hand_type = other.get_hand_type_with_joker();

        if self_hand_type.is_stronger_than(&other_hand_type) {
            return true;
        }

        if other_hand_type.is_stronger_than(&self_hand_type) {
            return false;
        }

        let cards = self.cards.iter().zip(other.cards.iter());
        for (self_card, other_card) in cards {
            if self_card.is_stronger_than_with_joker(other_card) {
                return true;
            }
            if other_card.is_stronger_than_with_joker(self_card) {
                return false;
            }
        }

        false
    }

    pub fn get_hand_type_with_joker(&self) -> HandType {
        let mut card_type_count: HashMap<&Card, usize> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_type_count.entry(card).or_insert(0);
            *count += 1;
        }

        let mut key_value_tuples = card_type_count
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();

        // get the amount of jokers
        let jokers = match key_value_tuples.iter().find(|(k, _)| k.is_joker()) {
            Some((_, v)) => *v,
            None => 0,
        };

        // remove the jokers from the list
        key_value_tuples = key_value_tuples
            .into_iter()
            .filter(|(k, _)| !k.is_joker())
            .collect::<Vec<_>>();

        if key_value_tuples.len() == 0 {
            return HandType::FiveOfAKind;
        }

        key_value_tuples.sort_by(|a, b| b.1.cmp(&a.1));

        key_value_tuples[0].1 += jokers;

        let mut values = key_value_tuples
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<usize>>();

        values.sort_by(|a, b| b.cmp(a));

        if values[0] == 5 {
            return HandType::FiveOfAKind;
        }

        if values[0] == 4 {
            return HandType::FourOfAKind;
        }

        if values[0] == 3 && values[1] == 2 {
            return HandType::FullHouse;
        }

        if values[0] == 3 && values[1] == 1 && values[2] == 1 {
            return HandType::ThreeOfAKind;
        }

        if values[0] == 2 && values[1] == 2 && values[2] == 1 {
            return HandType::TwoPair;
        }

        if values[0] == 2 && values[1] == 1 && values[2] == 1 && values[3] == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    pub fn get_bid(&self) -> usize {
        self.bid
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_stronger_than(other) {
            return Ordering::Greater;
        }

        if other.is_stronger_than(self) {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        let hand = Hand {
            cards: "AKQJT"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::HighCard);

        let hand = Hand {
            cards: "32TK3"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::OnePair);

        let hand = Hand {
            cards: "KK677"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::TwoPair);

        let hand = Hand {
            cards: "KTJJT"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::TwoPair);

        let hand = Hand {
            cards: "T55J5"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: "QQQJA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: "QQQAA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::FullHouse);

        let hand = Hand {
            cards: "KQKQK"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::FullHouse);

        let hand = Hand {
            cards: "QQKQQ"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "22222"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_is_stronger_than() {
        let first = Hand {
            cards: "32TK3"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };

        let second = Hand {
            cards: "KK677"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };

        let third = Hand {
            cards: "KTJJT"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };

        let fourth = Hand {
            cards: "T55J5"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };

        let fifth = Hand {
            cards: "QQQJA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 2,
        };

        assert!(second.is_stronger_than(&first));
        assert!(second.is_stronger_than(&third));
        assert!(fourth.is_stronger_than(&first));
        assert!(fourth.is_stronger_than(&second));
        assert!(fourth.is_stronger_than(&third));
        assert!(fifth.is_stronger_than(&fourth));
    }

    #[test]
    fn test_ordering_hands() {
        let mut hands = vec![
            Hand {
                cards: "KK677"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "QQQJA"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 2,
            },
            Hand {
                cards: "32TK3"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "KTJJT"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "T55J5"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
        ];

        assert_eq!(hands[0].cmp(&hands[1]), Ordering::Less);
        assert_eq!(hands[1].cmp(&hands[0]), Ordering::Greater);
        assert_eq!(hands[0].cmp(&hands[2]), Ordering::Greater);
        assert_eq!(hands[2].cmp(&hands[1]), Ordering::Less);

        hands.sort();

        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: "32TK3"
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<Card>().unwrap())
                        .collect(),
                    bid: 1,
                },
                Hand {
                    cards: "KTJJT"
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<Card>().unwrap())
                        .collect(),
                    bid: 1,
                },
                Hand {
                    cards: "KK677"
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<Card>().unwrap())
                        .collect(),
                    bid: 1,
                },
                Hand {
                    cards: "T55J5"
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<Card>().unwrap())
                        .collect(),
                    bid: 1,
                },
                Hand {
                    cards: "QQQJA"
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<Card>().unwrap())
                        .collect(),
                    bid: 2,
                },
            ]
        );
    }

    #[test]
    fn test_hand_type_with_joker() {
        let hand = Hand {
            cards: "QQQJA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 1,
        };
        assert_eq!(hand.get_hand_type_with_joker(), HandType::FourOfAKind);

        let hand = Hand {
            cards: "KQQJA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 2,
        };
        assert_eq!(hand.get_hand_type_with_joker(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: "QQAJA"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 3,
        };
        assert_eq!(hand.get_hand_type_with_joker(), HandType::FullHouse);

        let hand = Hand {
            cards: "JJJJJ"
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: 4,
        };
        assert_eq!(hand.get_hand_type_with_joker(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: vec![
                "K".parse::<Card>().unwrap(),
                "T".parse::<Card>().unwrap(),
                "J".parse::<Card>().unwrap(),
                "J".parse::<Card>().unwrap(),
                "T".parse::<Card>().unwrap(),
            ],
            bid: 5,
        };
        assert_eq!(hand.get_hand_type_with_joker(), HandType::FourOfAKind);

        let hands = vec![
            Hand {
                cards: "32TK3"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "T55J5"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "KK677"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "KTJJT"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 1,
            },
            Hand {
                cards: "QQQJA"
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect(),
                bid: 2,
            },
        ];

        assert_eq!(
            hands
                .iter()
                .map(|h| h.get_hand_type_with_joker())
                .collect::<Vec<_>>(),
            vec![
                HandType::OnePair,
                HandType::FourOfAKind,
                HandType::TwoPair,
                HandType::FourOfAKind,
                HandType::FourOfAKind,
            ]
        );
    }
}
