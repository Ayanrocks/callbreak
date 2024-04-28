use crate::card::Card;

pub struct Players {
    name: String,
    pin: u16,
    cards: Vec<Card>,
}

impl Players {
    pub fn new(name: &str, pin: &u16) -> Self {
        Players {
            name: name.to_string(),
            pin: *pin,
            cards: vec![],
        }
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
        // reveal the cards
        for r in self.cards.iter() {
            print!("{}", r.get_print_str())
        }
        println!()
    }
}