use rand::seq::SliceRandom;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

pub fn run() {
    let players = generate_players();
    println!("{:?}", players);

    let mut dealer = Dealer::new();
    println!("{:?}", dealer.stack);

    // TODO draw initial hands
    dealer.flip_first_card();

    // TODO remaining game logic
    // get top card
    // if wild card, let first player set color
    // game while loop ---
    // if action card, execute card action
    // next player
    // get top card
    // play
    // if card, discard, check game over
    // otherwise, draw 1, play again with that card
}

fn generate_players() -> Vec<Player> {
    // we define players as vector to be able to vary the number of players at runtime
    // TODO expose number of players as input parameter
    let names = ["A", "B", "C", "D"];
    let mut players: Vec<Player> = vec![];
    for name in names.iter() {
        let player = Player::new(name);
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
            let card = Card::new(number, Some(color));
            cards.push(card);
        }
        for symbol in symbols.iter() {
            let card = Card::new(symbol, Some(color));
            cards.push(card);
        }
    }
    for wild_symbol in wild_symbols.iter() {
        let card = Card::new(wild_symbol, None);
        cards.push(card);
    }

    // shuffle deck
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    cards
}

// define card object, with optional color field to handle wild cards where
// color is chosen by player when the card is played
// TODO perhaps distinguish between Card with symbol and color and WildCard
// with optional color and a common trait
#[derive(Debug)]
struct Card {
    symbol: &'static str,
    color: Option<Color>,
}

impl Card {
    fn new(symbol: &'static str, color: Option<Color>) -> Card {
        Card { symbol, color }
    }
}

// define player object
#[derive(Debug)]
struct Player {
    name: &'static str,
    hand: Vec<Card>,
}

impl Player {
    fn new(name: &'static str) -> Player {
        let mut hand: Vec<Card> = vec![];
        Player {
            name: name,
            hand: hand,
        }
    }
}

// define dealer object to handle interactions between stack and pile
#[derive(Debug)]
struct Dealer {
    stack: Vec<Card>,
    pile: Vec<Card>,
}

impl Dealer {
    fn new() -> Dealer {
        let stack = generate_deck();
        let pile: Vec<Card> = vec![];
        Dealer { stack, pile }
    }

    /// Draw `n` cards from stack.
    fn draw(&mut self, n: usize) -> Vec<Card> {
        let m = self.stack.len() - n;
        let range = m..;
        let cards = self.stack.drain(range).collect();
        cards
    }

    /// Discard `cards` onto discard pile.
    fn discard(&mut self, cards: Vec<Card>) {
        self.pile.extend(cards);
    }

    /// Flip first card of stack onto pile to start the game.
    fn flip_first_card(&mut self) {
        let card = self.draw(1);
        self.discard(card);
    }
}

// define test module, annotated with cfg attribute for conditional compilation,
// which excludes tests when building the package
#[cfg(test)]
mod tests {
    use super::*; // bring private functions into scope
    use rstest::rstest;

    #[test]
    fn test_generate_deck_number_of_cards() {
        let deck = generate_deck();
        let n_cards = deck.len();
        assert_eq!(n_cards, 108);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(3)]
    #[case(7)]
    #[case(13)]
    fn test_dealer_draw_number_of_cards(#[case] n: usize) {
        let mut dealer = Dealer::new();
        let cards = dealer.draw(n);
        assert_eq!(cards.len(), n);
    }
}
