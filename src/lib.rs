mod cycle;
use cycle::Cycle;
use cycle::Turn;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

const N_CARDS: usize = 108; // number of cards in standard deck
const N_INITIAL_CARDS: usize = 7; // number of cards in initial player hands
const N_PLAYERS: usize = 4;

// TODO add logging
pub fn run() {
    let n_players = 4;
    let mut players = PlayerCycle::new(n_players);
    println!("Players: {:?}", players.get_names());

    let mut dealer = Dealer::new();

    let hands = dealer.draw_hands(n_players, N_INITIAL_CARDS);
    players.take_hands(hands);

    dealer.flip_first_card();
    let card = Some(dealer.top_card());

    loop {
        // if action card, execute card action
        match card {
            None => {}
            Some(card) => {
                // TODO
                if card.is_action() {}
            }
        }

        // pick next player
        let player = players.next();

        // try playing card from hand
        let top_card = dealer.top_card();
        let card = player.play_from_hand(&top_card);
        println!("{:?}, {:?}", player.name, player.hand.len());

        // if no card is played, draw a new card and try playing it
        match card {
            Some(_) => {}
            None => {
                let new_card = dealer.draw(1);
                // TODO
                // card = player.play_from_card(&top_card, &new_card);
                match card {
                    Some(_) => {}
                    None => player.take_cards(new_card),
                }
            }
        };

        // if card, discard and check game over
        match card {
            None => {}
            Some(card) => {
                dealer.discard(vec![card]);
                if game_over(player) {
                    println!("Player: {:?} won! Game over.", player.name);
                    break;
                }
            }
        }
    }
}

/// Check if `player` has empty hand.
fn game_over(player: &Player) -> bool {
    player.hand.is_empty()
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

    fn is_wild_draw_4(&self) -> bool {
        self.symbol == "wild-draw-4s"
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

fn filter_legal_cards(cards: Cards, top_card: Card) -> Cards {
    assert!(!cards.is_empty());
    assert!(top_card.color.is_some());

    let n = cards.len();
    let mut legal_cards = Vec::with_capacity(n);
    let mut wild_draw_4s = Vec::with_capacity(n);
    let mut has_color_match = false;

    for card in cards {
        if card.is_wild_draw_4() {
            wild_draw_4s.push(card);
            continue;
        }
        let color_match = card.color == top_card.color;
        let symbol_match = card.symbol == top_card.symbol;
        if card.is_wild() || color_match || symbol_match {
            legal_cards.push(card);
        }
        if color_match {
            has_color_match = true;
        }
    }
    if !has_color_match {
        legal_cards.extend(wild_draw_4s);
    }
    legal_cards
}

// define object for a single player, encapsulating player state and strategy
#[derive(Debug)]
struct Player {
    name: &'static str,
    hand: Cards,
    // TODO should handle all objects implementing the strategy trait not just the
    // specific random strategy
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
    fn take_cards(&mut self, cards: Cards) {
        assert!(!cards.is_empty());
        self.hand.extend(cards);
    }

    /// Play card from `playable_cards` if possible for given `top_card`.
    fn play_from_cards(&self, top_card: &Card, cards: Cards) -> Option<Card> {
        assert!(!cards.is_empty());
        let legal_cards = filter_legal_cards(cards, *top_card);
        match legal_cards.is_empty() {
            true => None,
            false => self.strategy.select_card(legal_cards),
        }
    }

    /// Play card from hand if possible for given `top_card`.
    fn play_from_hand(&mut self, top_card: &Card) -> Option<Card> {
        let cards = self.hand.clone();
        let card = self.play_from_cards(top_card, cards);
        match card {
            None => {}
            Some(card) => {
                println!("hand={:?} \n card={:?}", self.hand, card);
                // remove card from hand
                // TODO card played has color set, but card on hand doesn't; maybe
                // add some unique ID to cards?
                let index = self
                    .hand
                    .iter()
                    .position(|x| *x == card)
                    .expect("selected card not in hand");
                self.hand.remove(index);
            }
        }
        card
    }
}

// define object for multiple players, handling player cycles
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

    /// Get player names.
    fn get_names(&self) -> Vec<&str> {
        self.players.iter().map(|x| x.name).collect()
    }

    /// Take `hands`, one for each player.
    fn take_hands(&mut self, hands: Vec<Cards>) {
        assert_eq!(self.players.len(), hands.len());
        for (player, hand) in self.players.iter_mut().zip(hands.into_iter()) {
            player.take_cards(hand);
        }
    }

    /// Get turn number.
    fn turn(&self) -> Turn {
        self.cycle.turn()
    }
}

// define strategy trait
trait Strategy {
    /// Select card from `legal_cards`.
    // TODO pass on game state for enabling strategies to make smarter decisions
    fn select_card(&self, legal_cards: Cards) -> Option<Card>;
}

#[derive(Debug)]
struct RandomStrategy {}

/// Randomly select color.
fn select_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let colors: Vec<Color> = Color::iter().collect();
    // de-reference data, see e.g. https://micouy.github.io/rust-dereferencing/
    *colors.choose(&mut rng).expect("empty colors")
}

impl Strategy for RandomStrategy {
    /// Randomly select card from `legal_cards`.
    fn select_card(&self, legal_cards: Cards) -> Option<Card> {
        assert!(!legal_cards.is_empty());
        let mut rng = rand::thread_rng();
        let mut card = *legal_cards.choose(&mut rng).expect("empty legal cards");
        if card.is_wild() {
            // if wild card, select color
            let color = select_random_color();
            card.color = Some(color);
        }
        Some(card)
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

    /// Draw `n_cards` cards from deck.
    fn draw(&mut self, n_cards: usize) -> Cards {
        let n_pile = self.pile.len();
        let n_available = self.deck.len();

        // check there are enough cards in deck and pile
        assert!(n_cards < (n_pile + n_available - 1));

        if n_cards <= n_available {
            // if enough cards are in the deck, simply draw cards
            self.draw_from_deck(n_cards)
        } else {
            // otherwise, draw available cards, recycle pile and draw remaining cards
            let mut cards = Vec::with_capacity(n_cards);
            cards.extend(self.draw_from_deck(n_available));

            self.recycle_pile();

            let n_remaining = n_cards - n_available;
            cards.extend(self.draw_from_deck(n_remaining));
            cards
        }
    }

    // Draw `n_cards` from deck, without recycling pile.
    fn draw_from_deck(&mut self, n_cards: usize) -> Cards {
        let n_available = self.deck.len();
        assert!(
            n_available >= n_cards,
            "n_available: {}, n_cards: {}",
            n_available,
            n_cards
        );
        let start = n_available - n_cards;
        self.deck.drain(start..).collect()
    }

    /// Discard `cards` onto discard pile.
    fn discard(&mut self, cards: Cards) {
        // TODO change discard to take single Card instead of Vec<Card>
        self.pile.extend(cards);
    }

    /// Flip first card of deck onto pile to start the game.
    fn flip_first_card(&mut self) {
        let mut cards = self.draw(1);

        // if the card is a wild card, it is returned to the deck and a new card is drawn.
        while cards.first().expect("no cards drawn").is_wild() {
            self.refill_deck(cards);
            cards = self.draw(1);
        }

        self.discard(cards);
    }

    /// Refill deck with `cards`.
    fn refill_deck(&mut self, mut cards: Cards) {
        cards = shuffle(cards);
        for card in cards {
            self.deck.push_back(card)
        }
    }

    /// Recyle all cards except top card from discard pile into deck.
    fn recycle_pile(&mut self) {
        let n = self.pile.len();
        assert!(n > 0); // pile must have at least one card
        let end = self.pile.len() - 1; // keep top card
        let cards = self.pile.drain(0..end).collect();

        self.refill_deck(cards);
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
    fn top_card(&self) -> Card {
        // TODO can we avoid the de-referencing (copy using the copy trait) here
        // and use an immutable reference instead?
        // the problem is that we both look at the top card on the pile and change
        // the pile when we discard a newly played card, but in theory discarding
        // the card should happen at the end, when we no longer need the top card
        *self.pile.last().expect("empty pile")
    }
}

// define test module, annotated with cfg attribute for conditional compilation,
// which excludes tests when building the package
#[cfg(test)]
mod tests {
    use super::*; // bring private functions into scope
    use rstest::rstest;

    #[test]
    fn test_generate_deck_n_cards() {
        let deck = generate_deck();
        assert_eq!(deck.len(), N_CARDS);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(3)]
    #[case(7)]
    #[case(13)]
    fn test_dealer_draw_n_cards_without_recycling(#[case] n: usize) {
        let mut dealer = Dealer::new();
        let n_before = dealer.deck.len();
        let cards = dealer.draw(n);
        let n_after = dealer.deck.len();

        assert_eq!(cards.len(), n);
        assert_eq!(n_before - n_after, n);
    }

    #[test]
    fn test_dealer_draw_with_recycle() {
        let n = 20;

        // empty deck into pile
        let mut dealer = Dealer::new();
        let _cards = dealer.draw(100);
        dealer.discard(_cards);
        let n_available = dealer.deck.len();

        // draw more cards than in deck
        let top_card = dealer.top_card();
        assert!(n > n_available);
        let cards = dealer.draw(n);

        assert_eq!(cards.len(), n); // check all requested cards were drawn
        assert_eq!(top_card, dealer.top_card()); // check top card stays the same
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

    // TODO add more tests
    #[test]
    fn test_filter_legal_cards() {
        let cards = vec![Card::new("0", Some(Color::Red))];
        let top_card = Card::new("1", Some(Color::Red));
        let legal_cards = filter_legal_cards(cards.clone(), top_card);
        assert_eq!(legal_cards, cards);
    }
}
