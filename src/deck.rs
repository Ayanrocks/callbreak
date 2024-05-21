use rand::Rng;

use crate::card::{Card, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Initializes a new deck with all cards
    /// ```rust
    /// let my_deck = Deck::new();
    /// ````
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: vec![],
        };
        deck.cards = deck.create_deck();
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

    /// Prints current deck stored in Deck.cards field
    ///  ```rust
    ///     let my_deck = Deck::new();
    ///     my_deck.print_deck();
    ///
    ///     // Output
    ///     [ 2 ♥️ ][ 3 ♥️ ][ 4 ♥️ ][ 5 ♥️ ][ 6 ♥️ ][ 7 ♥️ ][ 8 ♥️ ][ 9 ♥️ ][ 10 ♥️ ][ J ♥️ ][ Q ♥️ ][ K ♥️ ][ A ♥️ ]
    ///     [ 2 ♠️ ][ 3 ♠️ ][ 4 ♠️ ][ 5 ♠️ ][ 6 ♠️ ][ 7 ♠️ ][ 8 ♠️ ][ 9 ♠️ ][ 10 ♠️ ][ J ♠️ ][ Q ♠️ ][ K ♠️ ][ A ♠️ ]
    ///     [ 2 ♣️ ][ 3 ♣️ ][ 4 ♣️ ][ 5 ♣️ ][ 6 ♣️ ][ 7 ♣️ ][ 8 ♣️ ][ 9 ♣️ ][ 10 ♣️ ][ J ♣️ ][ Q ♣️ ][ K ♣️ ][ A ♣️ ]
    ///     [ 2 ♦️ ][ 3 ♦️ ][ 4 ♦️ ][ 5 ♦️ ][ 6 ♦️ ][ 7 ♦️ ][ 8 ♦️ ][ 9 ♦️ ][ 10 ♦️ ][ J ♦️ ][ Q ♦️ ][ K ♦️ ][ A ♦️ ]
    ///  ```
    pub fn print_deck(&self) {
        println!("\nDeck: 👇");
        for (i, c) in self.cards.iter().enumerate() {
            if i % 13 == 0 {
                println!();
            }
            print!("{}", c.get_print_str());
        }
        println!();
    }

    pub fn total_len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        for i in 0..self.cards.len() {
            let r = i + (rand::thread_rng().gen_range(0..self.total_len()) % (self.total_len() - i));
            self.cards.swap(i, r)
        }
    }

    pub fn distribute(&mut self) -> Card {
        self.cards.swap_remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.total_len(), 52);
        assert_eq!(deck.cards[0].get_value(), "2");
        assert_eq!(deck.cards[0].get_suit(), Suit::Hearts);
        assert_eq!(deck.cards[0].get_priority(), 2);
    }

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.total_len(), 52);
        assert_eq!(deck.cards[0].get_value(), "2");
        assert_eq!(deck.cards[0].get_suit(), Suit::Hearts);
        assert_eq!(deck.cards[0].get_priority(), 2);

        deck.shuffle();

        assert_ne!(deck.cards[0].get_value(), "2");
        assert_ne!(deck.cards[0].get_suit(), Suit::Hearts);
        assert_ne!(deck.cards[0].get_priority(), 2);
    }

    #[test]
    fn test_distribute() {
        let mut deck = Deck::new();
        let card = deck.distribute();

        assert_eq!(deck.cards.len(), 51);
        assert_eq!(deck.total_len(), 51);
        assert_eq!(card.get_value(), "2");
        assert_eq!(card.get_suit(), Suit::Hearts);
        assert_eq!(card.get_priority(), 2);
        assert_eq!(deck.cards[0].get_value(), "A");
        assert_eq!(deck.cards[0].get_suit(), Suit::Diamonds);
        assert_eq!(deck.cards[0].get_priority(), 14);
    }
}
