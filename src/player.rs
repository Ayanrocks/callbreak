use crate::card::{Card, TRUMP_SUIT};
use crate::game::Call;

pub struct Player {
    name: String,
    pin: u16,
    cards: Vec<Card>,
    call: Call,
}

impl Player {
    pub fn new(name: &str, pin: &u16, call: Call) -> Self {
        Player {
            name: name.to_string(),
            pin: *pin,
            cards: vec![],
            call,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }


    pub fn reveal(&self, pin_entered: &u16) {
        // check if pin is added or not
        if *pin_entered != self.pin {
            println!("Incorrect Pin Entered");
            return;
        }

        println!("\nPlayer: {} 👇", self.name);
        // reveal the cards
        for r in self.cards.iter() {
            print!("{}", r.get_print_str())
        }
        println!()
    }

    pub fn get_card_idx(&self, card_value: &str) -> usize {
        let mut idx = 0;
        for (i, c) in self.cards.iter().enumerate() {
            if card_value == c.get_value() {
                idx = i
            }
        }
        idx
    }

    /// throw throws the card and removes from the card for the player
    pub fn throw(&mut self, card_idx: usize) -> Card {
        self.cards.swap_remove(card_idx)
    }

    /// points to the eligible list of cards that the user needs to throw
    pub fn get_eligible_cards(&self, lead_thrower: &Card) {
        // check the lead thrower first
        let mut eligible_cards: Vec<(&Card, usize)> = vec![];

        for (i, c) in self.cards.iter().enumerate() {
            if c.get_suit() == lead_thrower.get_suit() {
                eligible_cards.push((&c, i))
            }
            if c.get_suit() == TRUMP_SUIT {
                eligible_cards.push((&c, i))
            }
        }

        // if empty then entire deck can be used for
        if eligible_cards.is_empty() {
            for (i, c) in self.cards.iter().enumerate() {
                eligible_cards.push((&c, i))
            }
        }

        println!();
        for t in eligible_cards.iter() {
            print!("{}", t.0.get_print_str())
        }
        println!();
        for t in eligible_cards.iter() {
            print!(" {}    ", t.1)
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let player = Player::new("test", &1234, Call::Two(2));

        assert_eq!(player.cards.len(), 0);
        assert_eq!(player.name, "test");
    }
}