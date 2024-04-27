use cards::card::{Card, Suit};
use deck::Deck;

mod cards;
mod deck;

fn main() {
    let card = Card::new(Suit::Club, "2".to_string());
    card.print();

    let deck = Deck::new();
    deck.print_deck();

    println!("Hello, world!, {}", deck.total_len());
}
