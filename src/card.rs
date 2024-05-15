use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    Club,
    Spade,
    Hearts,
    Diamonds,
}

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

    pub fn print(self) {
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

