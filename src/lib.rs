use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

pub fn run() {
    println!("Hello, world!");

    let players = generate_players();
    println!("{:?}", players);

    let cards = generate_deck();
    println!("{:?}", cards);
}

fn generate_players() -> Vec<Player> {
    let names = ["A", "B", "C", "D"];

    let mut players: Vec<Player> = vec![];
    for name in names.iter() {
        let player = Player::build(name);
        players.push(player);
    }

    players
}

// EnumIter creates new type with implementation of iter method
#[derive(Debug, Clone, Copy, EnumIter)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn generate_deck() -> Vec<Card> {
    println!("Generating deck ...");
    // TODO use generator to generate numbers
    // TODO use enums for symbols?
    let numbers = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "1", "2", "3", "4", "5", "6", "7", "8",
        "9",
    ];
    let symbols = ["skip", "skip", "reverse", "reverse", "draw-2", "draw-2"];
    let wild_symbols = [
        "wild",
        "wild",
        "wild",
        "wild",
        "wild-draw-4",
        "wild-draw-4",
        "wild-draw-4",
        "wild-draw-4",
    ];

    let mut cards: Vec<Card> = vec![];
    for color in Color::iter() {
        for number in numbers.iter() {
            let card = Card::build(number, Some(color));
            cards.push(card);
        }
        for symbol in symbols.iter() {
            let card = Card::build(symbol, Some(color));
            cards.push(card);
        }
    }
    for wild_symbol in wild_symbols.iter() {
        let card = Card::build(wild_symbol, Option::None);
        cards.push(card);
    }

    cards
}

#[derive(Debug)]
struct Card {
    symbol: &'static str,
    color: Option<Color>,
}

impl Card {
    fn build(symbol: &'static str, color: Option<Color>) -> Card {
        Card { symbol, color }
    }
}

#[derive(Debug)]
struct Player {
    name: &'static str,
    hand: Vec<Card>,
}

impl Player {
    fn build(name: &'static str) -> Player {
        let mut hand: Vec<Card> = vec![];
        Player {
            name: name,
            hand: hand,
        }
    }
}

// define test module, annotated with cfg attribute for conditional compilation,
// excluding tests when building the package
#[cfg(test)]
mod tests {
    use super::*; // bring private functions into scope

    #[test]
    fn test_generate_deck_number_of_cards() {
        let deck = generate_deck();
        let n_cards = deck.len();
        assert_eq!(n_cards, 108);
    }
}
