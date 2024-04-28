use rand::distributions::uniform::SampleBorrow;

use card::{Card, Suit};
use deck::Deck;
use player::Player;

use crate::game::{Call, Game};

mod card;
mod deck;
mod player;
mod game;

fn main() {
    let card = Card::new(Suit::Club, "2".to_string());
    card.print();

    let mut deck = Deck::new();
    deck.print_deck();
    deck.shuffle();
    deck.print_deck();
    let mut p1 = Player::new("Ayan", &2234, Call::Two(2));
    p1.add_card(Card::new(Suit::Diamonds, "2".to_string()));
    p1.reveal(&2234);

    let mut new_game = Game::new_game();
    new_game.add_players("as", 1234.borrow(), 2);
    new_game.add_players("bb", 1234.borrow(), 3);
    new_game.add_players("b1b", 1234.borrow(), 4);
    new_game.add_players("b2b", 1234.borrow(), 2);
    new_game.start();

    println!("Hello, world!, {}", deck.total_len());
}
