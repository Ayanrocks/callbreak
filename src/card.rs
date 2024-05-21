use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    Club,
    Spade,
    Hearts,
    Diamonds,
}

// Default trump suit is spade
pub const TRUMP_SUIT: Suit = Suit::Spade;

pub struct Card {
    value: String,
    suit: Suit,
    priority: i32,
}

impl Card {
    pub fn get_initial_priority(&self, value: &str) -> i32 {
        let mut hash_map: HashMap<&str, i32> = HashMap::new();
        hash_map.insert("2", 2);
        hash_map.insert("3", 3);
        hash_map.insert("4", 4);
        hash_map.insert("5", 5);
        hash_map.insert("6", 6);
        hash_map.insert("7", 7);
        hash_map.insert("8", 8);
        hash_map.insert("9", 9);
        hash_map.insert("10", 10);
        hash_map.insert("J", 11);
        hash_map.insert("Q", 12);
        hash_map.insert("K", 13);
        hash_map.insert("A", 14);

        if let Some(v) = hash_map.get(value) {
            *v
        } else {
            -1
        }
    }

    pub fn new(suit: Suit, value: String) -> Self {
        let mut card = Card {
            value,
            suit,
            priority: -1,
        };
        card.priority = card.get_initial_priority(card.value.as_str());
        card
    }

    pub fn get_print_str(&self) -> String {
        let parsed_suit = match self.suit {
            Suit::Club => "♣️",
            Suit::Spade => "♠️",
            Suit::Diamonds => "♦️",
            Suit::Hearts => "♥️"
        };

        format!("[ {} {} ]", self.value, parsed_suit)
    }

    pub fn print(&self) {
        println!("{}", self.get_print_str());
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_priority(&self) -> i32 {
        self.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_card() {
        let card = Card::new(Suit::Club, "2".to_string());

        assert_eq!(card.value, "2");
        assert_eq!(card.priority, 2);
        assert_eq!(card.suit, Suit::Club);

        let new_card = Card::new(Suit::Diamonds, "K".to_string());

        assert_eq!(new_card.value, "K");
        assert_eq!(new_card.priority, 13);
        assert_eq!(new_card.suit, Suit::Diamonds);

        let new_card_2 = Card::new(Suit::Hearts, "Q".to_string());

        assert_eq!(new_card_2.value, "Q");
        assert_eq!(new_card_2.priority, 12);
        assert_eq!(new_card_2.suit, Suit::Hearts);

        let new_card_3 = Card::new(Suit::Spade, "A".to_string());

        assert_eq!(new_card_3.value, "A");
        assert_eq!(new_card_3.priority, 14);
        assert_eq!(new_card_3.suit, Suit::Spade);
    }

    #[test]
    fn test_get_initial_priority() {
        let card = Card::new(Suit::Club, "2".to_string());
        assert_eq!(card.get_initial_priority("2"), 2)
    }

    #[test]
    fn test_get_print_str() {
        let card = Card::new(Suit::Club, "5".to_string());
        assert_eq!(card.get_print_str(), "[ 5 ♣\u{fe0f} ]");
    }

    #[test]
    fn test_get_value() {
        let card = Card::new(Suit::Diamonds, "9".to_string());
        assert_eq!(card.get_value(), "9");
    }

    #[test]
    fn test_get_suit() {
        let card = Card::new(Suit::Hearts, "9".to_string());
        assert_eq!(card.get_suit(), Suit::Hearts);
    }

    #[test]
    fn test_get_priority() {
        let card = Card::new(Suit::Hearts, "J".to_string());
        assert_eq!(card.get_priority(), 11);
    }
}

