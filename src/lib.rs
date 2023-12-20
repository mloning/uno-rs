use std::str;
use std::vec::Vec;

pub fn run() {
    println!("Hello, world!");

    let _card = Card::build("blue", "1");
    let _player = Player::build("A");
}

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
