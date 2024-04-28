use rand::distributions::uniform::SampleBorrow;

use crate::deck::Deck;
use crate::players::Players;

pub struct Game {
    deck: Deck,
    players: Vec<Players>,
}

impl Game {
    pub fn new_game() -> Self {
        let new_deck = Deck::new();
        Game {
            deck: new_deck,
            players: vec![],
        }
    }

    pub fn add_players(&mut self, name: &str, pin: &u16) {
        let new_player = Players::new(name, pin);
        self.players.push(new_player);
    }

    pub fn start(&mut self) {
        // check if atleast 2 players are added
        if self.players.len() < 2 {
            println!("Atleast 2 players is needed");
            return;
        }

        // shuffle the deck first
        self.deck.shuffle();

        // distribute the cards to each player
        let mut i = 0;
        loop {
            if self.deck.total_len() == 0 {
                break;
            }

            self.players[i].add_card(self.deck.distribute());
            i += 1;
            if i >= self.players.len() {
                i = 0
            }
        }

        self.players[0].reveal(1234.borrow());
        self.players[1].reveal(1234.borrow());
    }
}