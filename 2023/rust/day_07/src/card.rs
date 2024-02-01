use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub strength: u8,
    label: String,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card {
                strength: 14,
                label: s.to_string(),
            }),
            "K" => Ok(Card {
                strength: 13,
                label: s.to_string(),
            }),
            "Q" => Ok(Card {
                strength: 12,
                label: s.to_string(),
            }),
            "J" => Ok(Card {
                strength: 11,
                label: s.to_string(),
            }),
            "T" => Ok(Card {
                strength: 10,
                label: s.to_string(),
            }),
            "9" => Ok(Card {
                strength: 9,
                label: s.to_string(),
            }),
            "8" => Ok(Card {
                strength: 8,
                label: s.to_string(),
            }),
            "7" => Ok(Card {
                strength: 7,
                label: s.to_string(),
            }),
            "6" => Ok(Card {
                strength: 6,
                label: s.to_string(),
            }),
            "5" => Ok(Card {
                strength: 5,
                label: s.to_string(),
            }),
            "4" => Ok(Card {
                strength: 4,
                label: s.to_string(),
            }),
            "3" => Ok(Card {
                strength: 3,
                label: s.to_string(),
            }),
            "2" => Ok(Card {
                strength: 2,
                label: s.to_string(),
            }),
            _ => Err(format!("Invalid card: {}", s)),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn new(label: &str) -> Result<Self, String> {
        label.parse::<Card>()
    }

    pub fn is_stronger_than(&self, other: &Card) -> bool {
        self.strength > other.strength
    }

    pub fn is_stronger_than_with_joker(&self, other: &Card) -> bool {
        if self.is_joker() {
            return false;
        }

        if other.is_joker() {
            return true;
        }

        self.is_stronger_than(other)
    }

    pub fn is_joker(&self) -> bool {
        self.label == "J"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_card() {
        let card = "A".parse::<Card>().unwrap();
        assert_eq!(
            card,
            Card {
                strength: 14,
                label: "A".to_string(),
            }
        );

        let card = "T".parse::<Card>().unwrap();
        assert_eq!(
            card,
            Card {
                strength: 10,
                label: "T".to_string(),
            }
        );

        let card = "2".parse::<Card>().unwrap();
        assert_eq!(
            card,
            Card {
                strength: 2,
                label: "2".to_string(),
            }
        );
    }

    #[test]
    fn test_is_stronger_than() {
        let stronger = "A".parse::<Card>().unwrap();
        let weaker = "T".parse::<Card>().unwrap();

        assert!(stronger.is_stronger_than(&weaker));
        assert!(!weaker.is_stronger_than(&stronger));
    }

    #[test]
    fn test_card_sorting() {
        let mut cards = vec![
            "4".parse::<Card>().unwrap(),
            "A".parse::<Card>().unwrap(),
            "T".parse::<Card>().unwrap(),
            "2".parse::<Card>().unwrap(),
        ];

        cards.sort();

        assert_eq!(cards[0].label, "2");
        assert_eq!(cards[1].label, "4");
        assert_eq!(cards[2].label, "T");
        assert_eq!(cards[3].label, "A");
    }

    #[test]
    fn test_is_stronger_than_with_joker() {
        let cards = vec![
            "4".parse::<Card>().unwrap(),
            "J".parse::<Card>().unwrap(),
            "T".parse::<Card>().unwrap(),
            "2".parse::<Card>().unwrap(),
        ];

        assert!(cards[0].is_stronger_than_with_joker(&cards[1]));
        assert!(cards[2].is_stronger_than_with_joker(&cards[1]));
        assert!(cards[3].is_stronger_than_with_joker(&cards[1]));
    }
}
