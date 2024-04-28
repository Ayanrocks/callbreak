use std::collections::HashMap;

use rand::distributions::uniform::SampleBorrow;

use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;

pub struct Game<'a> {
    deck: Deck,
    players: Vec<Player>,
    default_calls: HashMap<u8, Call>,
    current_round: Vec<Round<'a>>,
}

struct Round<'a> {
    player: &'a Player,
    card: Card,
}

#[derive(Clone, Copy)]
pub enum Call {
    Two(u8),
    Three(u8),
    Four(u8),
    Five(u8),
    Six(u8),
    Seven(u8),
    Eight(u8),
}

impl<'a> Game<'a> {
    pub fn new_game() -> Self {
        let new_deck = Deck::new();
        let mut game = Game {
            deck: new_deck,
            players: vec![],
            default_calls: HashMap::new(),
            current_round: vec![],
        };

        game.initiate_calls();
        game
    }
    fn initiate_calls(&mut self) {
        self.default_calls.insert(2, Call::Two(2));
        self.default_calls.insert(3, Call::Three(3));
        self.default_calls.insert(4, Call::Four(4));
        self.default_calls.insert(5, Call::Five(5));
        self.default_calls.insert(6, Call::Six(6));
        self.default_calls.insert(7, Call::Seven(7));
        self.default_calls.insert(8, Call::Eight(8));
    }

    // TODO: return Result or error from this function
    pub fn add_players(&mut self, name: &str, pin: &u16, call: u8) {
        // check if the player name is already there
        // we can use a hashmap as well here, but for the short number of players it works for now
        for p in self.players.iter() {
            if p.get_name() == name {
                println!("Player already added in the game");
                return;
            }
        }

        let player_call = self.default_calls.get(&call).expect("invalid call supplied");
        let new_player = Player::new(name, pin, *player_call);
        self.players.push(new_player);
    }

    pub fn start(&mut self) {
        // check if at least 2 players are added
        if self.players.len() < 2 {
            println!("At least 2 players is needed");
            return;
        }


        // distribute the cards to each player
        let mut i = 0;
        loop {
            if self.deck.total_len() == 0 {
                break;
            }
            // shuffle the deck first
            self.deck.shuffle();
            self.players[i].add_card(self.deck.distribute());
            i += 1;
            if i >= self.players.len() {
                i = 0
            }
        }

        self.players[0].reveal(1234.borrow());
        self.players[1].reveal(1234.borrow());
        self.players[2].reveal(1234.borrow());
        self.players[3].reveal(1234.borrow());
    }

    fn get_player_index(&self, player_name: &str) -> usize {
        let mut idx = self.players.len() - 1;
        for (i, c) in self.players.iter().enumerate() {
            if c.get_name() == player_name {
                idx = i
            }
        }
        idx
    }


    pub fn throw(&mut self, player_name: &str, card_idx: usize) {
        // throw the current players card to the
        let player_idx = self.get_player_index(player_name);

        // throw the card that the player passed to the round
        let throwable_card = self.players[player_idx].throw(card_idx);
        // add the card to the round
        // self.current_round.push(Round {
        //     player: &self.players[player_idx],
        //     card: throwable_card,
        // });
    }
}