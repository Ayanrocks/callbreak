use std::collections::HashMap;

use rand::distributions::uniform::SampleBorrow;

use crate::card::{Card, Suit};
use crate::deck::Deck;
use crate::player::Player;

pub struct Game<'a> {
    deck: Deck,
    players: Vec<Player>,
    default_calls: HashMap<u8, Call>,
    current_round: Trick<'a>,
}

struct Trick<'a> {
    rounds: Vec<Round<'a>>,
    winner: Winner<'a>,
}

struct Winner<'a> {
    player: &'a str,
    suit: Suit,
    priority: i32,
    value: String,
}

struct Round<'a> {
    player: &'a str,
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
    /// new_game initializes a new game with default calls and empty player vector
    /// then once the game is initialized, the players can be added individually
    /// ```rust
    ///  let game = Game::new_game()
    /// ```
    pub fn new_game() -> Self {
        let new_deck = Deck::new();
        let mut game = Game {
            deck: new_deck,
            players: vec![],
            default_calls: HashMap::new(),
            current_round: Trick::new(),
        };

        game.initiate_calls();
        game
    }

    /// initiate_calls initiate default calls for the game
    fn initiate_calls(&mut self) {
        self.default_calls.insert(2, Call::Two(2));
        self.default_calls.insert(3, Call::Three(3));
        self.default_calls.insert(4, Call::Four(4));
        self.default_calls.insert(5, Call::Five(5));
        self.default_calls.insert(6, Call::Six(6));
        self.default_calls.insert(7, Call::Seven(7));
        self.default_calls.insert(8, Call::Eight(8));
    }


    /// add_players adds individual player with a specific pin and a call value
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


    /// starts the game by shuffling and giving equal number of cards to the players
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

    /// get_player_index returns the player index with the player name
    fn get_player_index(&self, player_name: &str) -> usize {
        let mut idx = self.players.len() - 1;
        for (i, c) in self.players.iter().enumerate() {
            if c.get_name() == player_name {
                idx = i
            }
        }
        idx
    }

    /// throw takes player name and card the user want to throw
    /// and throw it in a single round in the game
    pub fn throw(&mut self, player_name: &'a str, card_idx: usize) {
        // check if this is a new round or an existing round
        let mut is_new_round = true;
        if self.current_round.rounds.len() != 0 {
            is_new_round = false
        }
        // throw the current players card to the
        let player_idx = self.get_player_index(player_name);
        // throw the card that the player passed to the round
        let throwable_card = self.players[player_idx].throw(card_idx);
        // get the current winner of the round
        if is_new_round {
            self.current_round.winner = Winner {
                player: player_name,
                suit: throwable_card.get_suit(),
                priority: throwable_card.get_priority(),
                value: throwable_card.get_value().to_string(),
            }
        } else {
            // get the current winner
            self.get_round_winner()
        }

        let new_round = Round {
            player: player_name,
            card: throwable_card,
        };
        // add the card to the round
        self.current_round.rounds.push(new_round);
    }

    /// clears current round and adds an empty round
    pub fn clear_round(&mut self) {
        self.current_round = Trick::new();
    }

    /// gets the current round winner
    pub fn get_round_winner(&mut self) {
        // get all the round cards
        // set rules for winner of the game
        // check if any spade is there in the card
        for s in self.current_round.rounds.iter() {
            if s.card.get_suit() == Suit::Spade {
                // get the highest suit if the current suit is 'spade'
                if self.current_round.winner.suit == Suit::Spade {
                    if s.card.get_priority() > self.current_round.winner.priority {
                        self.current_round.winner = Winner::new(s.player,
                                                                s.card.get_suit(),
                                                                s.card.get_priority(),
                                                                s.card.get_value().to_string());
                    }
                } else {
                    // if the current winner is not spade then by default spade wins
                    self.current_round.winner = Winner::new(s.player,
                                                            s.card.get_suit(),
                                                            s.card.get_priority(),
                                                            s.card.get_value().to_string());
                }
            } else {
                if self.current_round.winner.suit == Suit::Spade {
                    self.current_round.winner = Winner::new(s.player,
                                                            s.card.get_suit(),
                                                            s.card.get_priority(),
                                                            s.card.get_value().to_string());
                } else {
                    // check for a higher priority card
                    if s.card.get_priority() > self.current_round.winner.priority && s.card.get_suit() == self.current_round.winner.suit {
                        self.current_round.winner = Winner::new(s.player,
                                                                s.card.get_suit(),
                                                                s.card.get_priority(),
                                                                s.card.get_value().to_string());
                    }
                }
            }
        }
    }
}

impl<'a> Trick<'a> {
    pub fn new() -> Trick<'a> {
        Trick {
            rounds: vec![],
            winner: Winner {
                player: "",
                suit: (Suit::Spade),
                priority: 0,
                value: "".to_string(),
            },
        }
    }
}

impl<'a> Winner<'a> {
    pub fn new(player_name: &str, suit: Suit, priority: i32, value: String) -> Winner {
        Winner {
            player: player_name,
            suit,
            priority,
            value,
        }
    }
}





