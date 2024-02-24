mod cycle;
use cycle::Cycle;
use cycle::Turn;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

const N_CARDS: usize = 108;
const N_INITIAL_CARDS: usize = 7;
const N_PLAYERS: usize = 4;

// TODO add logging
pub fn run() {
    let n_players = 4;
    let mut players = PlayerCycle::new(n_players);

    let names: Vec<&str> = players.get_names();
    println!("Players: {:?}", names);

    let mut dealer = Dealer::new();
    // println!("Deck: {:?}", dealer.deck);

    // draw and take initial hands
    let hands = dealer.draw_hands(n_players, N_INITIAL_CARDS);
    players.take_hands(hands);

    // flip first card
    dealer.flip_first_card();

    loop {
        let mut top_card = dealer.top_card();

        // TODO if action card, execute card action
        // if card.is_action() {}

        // pick next player
        let mut player = players.next();
        println!("Player: {:?}", player.name);
        // let card = player.play_card(top_card);

        // play
        // if card, discard, check game over, if game over, break
        // otherwise, draw 1, play again with that card
        if players.get_turn() == 10 {
            break;
        }
    }
}

type Cards = Vec<Card>;
type Players = Vec<Player>;
type Deck = VecDeque<Card>;

fn generate_players(n_players: usize) -> Players {
    // we define players as vector to be able to vary the number of players at runtime
    // TODO expose number of players as input parameter
    assert_eq!(n_players, N_PLAYERS);
    let names = ["A", "B", "C", "D"];
    let mut players: Players = Vec::with_capacity(n_players);
    for name in names.iter() {
        let player = Player::new(name);
        players.push(player);
    }
    players
}

// EnumIter creates new type with implementation of iter method
#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn generate_deck() -> Deck {
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

    let mut cards: Cards = Vec::with_capacity(N_CARDS);
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
    cards = shuffle(cards);

    // return as deque type
    VecDeque::from(cards)
}

/// Randomly shuffle cards.
fn shuffle(mut cards: Cards) -> Cards {
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    cards
}

// define card object, with optional color field to handle wild cards where
// color is chosen by player when the card is played
#[derive(Debug, Copy, Clone, PartialEq)]
struct Card {
    symbol: &'static str,
    color: Option<Color>,
}

impl Card {
    fn new(symbol: &'static str, color: Option<Color>) -> Self {
        Self { symbol, color }
    }

    fn is_wild(&self) -> bool {
        self.symbol.starts_with("wild")
    }

    fn is_action(&self) -> bool {
        let symbols = ["skip", "reverse", "draw-2", "wild-draw-4"];
        symbols.contains(&self.symbol)
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
    fn take_cards(&mut self, cards: &Cards) {
        self.hand.extend(cards);
    }

    // Play card if possible for given `playable_cards` and `top_card`.
    // fn play_card(&mut self, playable_cards: Option<Cards>, top_card: Card) -> Option<&Card> {
    //     // if no playable cards are given, all cards from the hand can be played
    //     let _playable_cards = match playable_cards {
    //         Some(cards) => cards,
    //         None => self.hand.clone(),
    //     };
    //     self.strategy.select_card(&_playable_cards, &top_card)
    // }
}

/// define object for multiple players, handling player cycles
struct PlayerCycle {
    players: Players,
    cycle: Cycle,
}

impl PlayerCycle {
    fn new(n_players: usize) -> Self {
        let players = generate_players(n_players);
        let cycle = Cycle::new(n_players);
        Self { players, cycle }
    }

    /// Get next player.
    fn next(&mut self) -> &mut Player {
        let index = self.cycle.next().expect("no cycle values");
        self.players.get_mut(index).expect("no players")
    }

    /// Reverse player cycle.
    fn reverse(&mut self) {
        self.cycle.reverse();
    }

    /// Skip player.
    fn skip(&mut self) {
        self.cycle.next();
    }

    /// Get number of players.
    fn len(&self) -> usize {
        self.players.len()
    }

    /// Get player names.
    fn get_names(&self) -> Vec<&str> {
        self.players.iter().map(|x| x.name).collect()
    }

    /// Take `hands`, one for each player.
    fn take_hands(&mut self, hands: Vec<Cards>) {
        assert_eq!(self.players.len(), hands.len());
        for (player, hand) in self.players.iter_mut().zip(hands.iter()) {
            player.take_cards(hand);
        }
    }

    fn get_turn(&self) -> Turn {
        self.cycle.get_turn()
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

// define dealer object to handle interactions between deck and pile
#[derive(Debug)]
struct Dealer {
    deck: Deck,
    pile: Cards,
}

impl Dealer {
    fn new() -> Self {
        let deck = generate_deck();
        let pile: Cards = Vec::with_capacity(N_CARDS);
        Self { deck, pile }
    }

    /// Draw `n` cards from deck.
    fn draw(&mut self, n: usize) -> Cards {
        // TODO recycle pile if deck is empty
        let m = self.deck.len() - n;
        let range = m..;
        let cards = self.deck.drain(range).collect();
        cards
    }

    /// Discard `cards` onto discard pile.
    fn discard(&mut self, cards: Cards) {
        self.pile.extend(cards);
    }

    /// Flip first card of deck onto pile to start the game.
    fn flip_first_card(&mut self) {
        let mut cards = self.draw(1);
        // If the card is a wild card, it is returned to the deck and a new card is drawn.
        while cards.first().expect("no cards drawn").is_wild() {
            self.refill(cards);
            cards = self.draw(1);
        }
        self.discard(cards);
    }

    /// Refill deck with `cards`.
    fn refill(&mut self, mut cards: Cards) {
        cards = shuffle(cards);
        for card in cards {
            self.deck.push_back(card)
        }
    }

    /// Draw `n_cards` initial hands for `n_players`.
    fn draw_hands(&mut self, n_players: usize, n_cards: usize) -> Vec<Cards> {
        let mut hands: Vec<Cards> = Vec::with_capacity(n_players);
        for _ in 0..n_players {
            let hand = self.draw(n_cards);
            hands.push(hand);
        }
        hands
    }

    /// Get top card from pile.
    fn top_card(&mut self) -> &mut Card {
        self.pile.last_mut().expect("empty pile")
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
        assert_eq!(deck.len(), N_CARDS);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(3)]
    #[case(7)]
    #[case(13)]
    fn test_dealer_draw_number_of_cards(#[case] n: usize) {
        let mut dealer = Dealer::new();
        let n_before = dealer.deck.len();
        let cards = dealer.draw(n);
        let n_after = dealer.deck.len();

        assert_eq!(cards.len(), n);
        assert_eq!(n_before - n_after, n);
    }

    #[test]
    fn test_dealer_flip_initial_card() {
        let mut dealer = Dealer::new();
        assert_eq!(dealer.pile.len(), 0);

        let n_before = dealer.deck.len();
        dealer.flip_first_card();
        let n_after = dealer.deck.len();

        assert_eq!(dealer.pile.len(), 1);
        assert_eq!(n_before - n_after, 1);
    }

    #[test]
    fn test_dealer_flip_initial_card_wild_card() {
        let mut dealer = Dealer::new();
        let wild_card = Card::new("wild-draw-4", None);
        dealer.deck.push_front(wild_card);
        dealer.flip_first_card();
        let card = dealer.top_card();

        assert_ne!(card.clone(), wild_card);
        assert_eq!(dealer.deck.pop_front().unwrap(), wild_card);
    }
}
