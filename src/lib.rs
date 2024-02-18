use std::str;
use std::vec::Vec;

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

fn generate_deck() -> Vec<Card> {
    println!("Generating deck ...");
    let colors: Vec<&'static str> = vec!["red", "blue", "green", "yellow"];
    // TODO use generator to generate numbers
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
    for color in colors.iter() {
        for number in numbers.iter() {
            let card = Card::build(number, color);
            cards.push(card);
        }
        for symbol in symbols.iter() {
            let card = Card::build(symbol, color);
            cards.push(card);
        }
    }
    // TODO handle optional color argument
    // for wild_symbol in wild_symbols.iter() {
    //     let card = Card::build(symbol);
    //     cards.push(card);
    // }

    cards
}

#[derive(Debug)]
struct Card {
    symbol: &'static str,
    color: &'static str,
}

impl Card {
    fn build(symbol: &'static str, color: &'static str) -> Card {
        Card {
            symbol: symbol,
            color: color,
        }
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
