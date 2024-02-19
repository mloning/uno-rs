use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

const N_INITIAL_CARDS: usize = 7;

pub fn run() {
    let mut players = generate_players();
    let n_players = players.len();
    let names: Vec<&str> = players.iter().map(|x| x.name).collect();
    println!("Players: {:?}", names);

    let mut dealer = Dealer::new();
    println!("Stack: {:?}", dealer.stack);

    // draw and take initial hands
    let hands = dealer.draw_initial_hands(n_players, N_INITIAL_CARDS);
    for (player, hand) in players.iter_mut().zip(hands.iter()) {
        player.take(hand);
    }

    dealer.flip_initial_card();
    let top_card = dealer.get_top_card();

    // TODO remaining game logic
    // if wild card, let first player set color
    // game while loop ---
    // if action card, execute card action
    // next player
    // get top card
    // play
    // if card, discard, check game over
    // otherwise, draw 1, play again with that card
}

type Players = Vec<Player>;
type Cards = Vec<Card>;

fn generate_players() -> Players {
    // we define players as vector to be able to vary the number of players at runtime
    // TODO expose number of players as input parameter
    let names = ["A", "B", "C", "D"];
    let mut players: Players = vec![];
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

fn generate_deck() -> Cards {
    println!("Generating deck ...");
    // TODO use generator to generate numbers
    // TODO use enums for numbers/symbols?
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

    let mut cards: Cards = vec![];
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
#[derive(Debug, Copy, Clone)]
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
// TODO define composite players object
#[derive(Debug)]
struct Player {
    name: &'static str,
    hand: Cards,
    // TODO should handle all objects implementing the strategy trait
    strategy: RandomStrategy,
}

impl Player {
    fn new(name: &'static str) -> Player {
        let hand: Cards = vec![];
        let strategy = RandomStrategy {};
        Player {
            name,
            hand,
            strategy,
        }
    }

    /// Take `cards` into hand.
    fn take(&mut self, cards: &Cards) {
        self.hand.extend(cards);
    }

    /// Select color of wild cards.
    fn select_color(&self) -> Color {
        self.strategy.select_color()
    }

    // TODO
    // fn play(&mut self, playable_cards: Option<Cards>, top_card: Card) -> Option<Card> {}
}

// define strategy trait
trait Strategy {
    fn select_color(&self) -> Color;
    fn select_card<'a>(&self, playable_cards: &'a Cards, top_card: &Card) -> Option<&'a Card>;
}

#[derive(Debug)]
struct RandomStrategy {}

impl Strategy for RandomStrategy {
    /// Randomly select card from `playable_cards`, ignoring `top_card`.
    fn select_card<'a>(&self, playable_cards: &'a Cards, _top_card: &Card) -> Option<&'a Card> {
        let mut rng = rand::thread_rng();
        playable_cards.choose(&mut rng)
    }

    /// Randomly select color.
    fn select_color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let colors: Vec<Color> = Color::iter().collect();
        // de-reference data, see e.g. https://micouy.github.io/rust-dereferencing/
        *colors
            .choose(&mut rng)
            .unwrap_or_else(|| panic!("No color selected!"))
    }
}

// define dealer object to handle interactions between stack and pile
#[derive(Debug)]
struct Dealer {
    stack: Cards,
    pile: Cards,
}

impl Dealer {
    fn new() -> Dealer {
        let stack = generate_deck();
        let pile: Cards = vec![];
        Dealer { stack, pile }
    }

    /// Draw `n` cards from stack.
    fn draw(&mut self, n: usize) -> Cards {
        let m = self.stack.len() - n;
        let range = m..;
        let cards = self.stack.drain(range).collect();
        cards
    }

    /// Discard `cards` onto discard pile.
    fn discard(&mut self, cards: Cards) {
        self.pile.extend(cards);
    }

    /// Flip first card of stack onto pile to start the game.
    fn flip_initial_card(&mut self) {
        let card = self.draw(1);
        self.discard(card);
    }

    /// Draw initial hands for players.
    fn draw_initial_hands(&mut self, n_players: usize, n_cards: usize) -> Vec<Cards> {
        let mut hands: Vec<Cards> = vec![];
        for _ in 0..n_players {
            let hand = self.draw(n_cards);
            hands.push(hand);
        }
        hands
    }

    /// Get top card from pile.
    fn get_top_card(&mut self) -> &Card {
        // TODO how to avoid panic, always initialize object with non-empty pile?
        self.pile.last().unwrap_or_else(|| panic!("Empty pile!"))
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
        assert_eq!(deck.len(), 108);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(3)]
    #[case(7)]
    #[case(13)]
    fn test_dealer_draw_number_of_cards(#[case] n: usize) {
        let mut dealer = Dealer::new();
        let n_before = dealer.stack.len();
        let cards = dealer.draw(n);
        let n_after = dealer.stack.len();

        assert_eq!(cards.len(), n);
        assert_eq!(n_before - n_after, n);
    }
}
