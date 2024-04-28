use crate::card::Card;
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

    pub fn throw(&mut self, card_idx: usize) -> Card {
        self.cards.swap_remove(card_idx)
    }
}