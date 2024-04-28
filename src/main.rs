use rand::distributions::uniform::SampleBorrow;

use card::{Card, Suit};
use deck::Deck;
use players::Players;

use crate::game::Game;

mod card;
mod deck;
mod players;
mod game;

fn main() {
    let card = Card::new(Suit::Club, "2".to_string());
    card.print();

    let mut deck = Deck::new();
    deck.print_deck();
    deck.shuffle();
    deck.print_deck();
    let mut p1 = Players::new("Ayan", &2234);
    p1.add_card(Card::new(Suit::Diamonds, "2".to_string()));
    p1.reveal(&2234);

    let mut new_game = Game::new_game();
    new_game.add_players("as", 1234.borrow());
    new_game.add_players("bb", 1234.borrow());
    new_game.add_players("bb", 1234.borrow());
    new_game.add_players("bb", 1234.borrow());
    new_game.start();

    println!("Hello, world!, {}", deck.total_len());
}
