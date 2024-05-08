#[derive(Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn is_stronger_than(&self, other: &HandType) -> bool {
        self.get_strength() < other.get_strength()
    }

    pub fn get_strength(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 1,
            HandType::FourOfAKind => 2,
            HandType::FullHouse => 3,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 5,
            HandType::OnePair => 6,
            HandType::HighCard => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type_is_stronger() {
        let five_of_a_kind = HandType::FiveOfAKind;
        let four_of_a_kind = HandType::FourOfAKind;
        assert!(five_of_a_kind.is_stronger_than(&four_of_a_kind));
    }
}
