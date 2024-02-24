mod cycle;
use rand::seq::SliceRandom;
use std::slice::IterMut;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

const N_INITIAL_CARDS: usize = 7;

pub fn run() {
    // TODO add logging
    let mut players = generate_players();
    let n_players = players.len();
    let names: Vec<&str> = players.get_names();
    println!("Players: {:?}", names);

    let mut dealer = Dealer::new();
    println!("Stack: {:?}", dealer.stack);

    // draw and take initial hands
    let hands = dealer.draw_initial_hands(n_players, N_INITIAL_CARDS);
    players.take(hands);

    // flip first card
    dealer.flip_initial_card();
    let card = dealer.get_top_card();

    // if first card is wild card, let first player set color
    if is_wild(card) {
        let player = players.first();
        let color = player.select_color();
        card.color = Some(color);
    }

    // loop {
    // TODO if action card, execute card action
    // if is_action(card) {}

    // TODO pick next player
    // let player = players.next();

    // let card = dealer.get_top_card();

    // get top card
    // play
    // if card, discard, check game over, if game over, break
    // otherwise, draw 1, play again with that card
    // }
}

type Cards = Vec<Card>;

fn is_wild(card: &Card) -> bool {
    card.symbol.contains("wild")
}

fn is_action(card: &Card) -> bool {
    let symbols = ["skip", "reverse", "draw-2", "wild-draw-4"];
    symbols.contains(&card.symbol)
}

fn generate_players() -> Players {
    // we define players as vector to be able to vary the number of players at runtime
    // TODO expose number of players as input parameter
    let names = ["Anne", "Ben", "Cam", "Dan"];
    let mut players: Vec<Player> = vec![];
    for name in names.iter() {
        let player = Player::new(name);
        players.push(player);
    }
    Players { players }
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
#[derive(Debug, Copy, Clone)]
struct Card {
    symbol: &'static str,
    color: Option<Color>,
}

impl Card {
    fn new(symbol: &'static str, color: Option<Color>) -> Self {
        Self { symbol, color }
    }
}

// TODO distinguish between stateless ColorCard (symbol and color) and stateful WildCard,
// or two structs for wild cards, one without color and one with color (symbol and optional color)
// struct WildCard {
//     symbol: &'static str,
// }
//
// impl WildCard {
//     fn set_color(&'static self, color: Color) -> SetWildCard {
//         SetWildCard {
//             card: self,
//             symbol: self.symbol,
//             color,
//         }
//     }
// }
//
// struct SetWildCard {
//     card: &'static WildCard,
//     symbol: &'static str,
//     color: Color,
// }

// define object for a single player, encapsulating player state and strategy
#[derive(Debug)]
struct Player {
    name: &'static str,
    hand: Cards,
    // TODO should handle all objects implementing the strategy trait
    strategy: RandomStrategy,
}

impl Player {
    fn new(name: &'static str) -> Self {
        let hand: Cards = vec![];
        let strategy = RandomStrategy {};
        Self {
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

/// define object for multiple players, handling player cycles
struct Players {
    players: Vec<Player>,
}

// TODO implement player cycling
// fn cycle(players: Vec<Player>, current: Player, is_reversed: bool) -> IterMut<'_, Player> {}

impl Players {
    /// Get first player.
    fn first(&self) -> &Player {
        self.players
            .first()
            .unwrap_or_else(|| panic!("Empty players."))
    }

    /// Get next player.
    // TODO
    // fn next(&mut self) -> &mut Player {}

    /// Reverse player cycle.
    // TODO
    // fn reverse(&mut self) -> {}

    /// Skip player.
    // TODO
    // fn skip(&mut self) -> {}

    /// Get number of players.
    fn len(&self) -> usize {
        self.players.len()
    }

    /// Get player names.
    fn get_names(&self) -> Vec<&str> {
        self.players.iter().map(|x| x.name).collect()
    }

    /// Iterate over mutable reference of players.
    fn iter_mut(&mut self) -> IterMut<'_, Player> {
        self.players.iter_mut()
    }

    /// Take one `hand` for each player.
    fn take(&mut self, hands: Vec<Cards>) {
        for (player, hand) in self.players.iter_mut().zip(hands.iter()) {
            player.take(hand);
        }
    }
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
    fn new() -> Self {
        let stack = generate_deck();
        let pile: Cards = vec![];
        Self { stack, pile }
    }

    /// Draw `n` cards from stack.
    fn draw(&mut self, n: usize) -> Cards {
        // TODO recycle pile if stack is empty
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
    fn get_top_card(&mut self) -> &mut Card {
        // TODO how to avoid panic, always initialize object with non-empty pile?
        self.pile
            .last_mut()
            .unwrap_or_else(|| panic!("Empty pile!"))
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
