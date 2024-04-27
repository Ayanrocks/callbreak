use crate::cards::card::{Card, Suit};

pub struct Deck {
    Cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck {
            Cards: vec![],
        };
        deck.Cards = deck.create_deck();
        deck
    }

    fn create_deck(&self) -> Vec<Card> {
        let mut card_deck: Vec<Card> = vec![];
        let hearts: Vec<&str> = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let spades: Vec<&str> = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let clubs: Vec<&str> = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let diamonds: Vec<&str> = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

        for c in hearts {
            card_deck.push(Card::new(Suit::Hearts, c.to_string()))
        }

        for c in spades {
            card_deck.push(Card::new(Suit::Spade, c.to_string()))
        }

        for c in clubs {
            card_deck.push(Card::new(Suit::Club, c.to_string()))
        }

        for c in diamonds {
            card_deck.push(Card::new(Suit::Diamonds, c.to_string()))
        }

        card_deck
    }

    pub fn print_deck(&self) {
        println!("Deck: 👇");
        for (i, c) in self.Cards.iter().enumerate() {
            if i % 13 == 0 {
                println!();
            }
            print!("{}", c.get_print_str());
        }
        println!();
    }

    pub fn total_len(&self) -> usize {
        self.Cards.len()
    }
}