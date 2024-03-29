mod cycle;
use colored::ColoredString;
use colored::Colorize;
use cycle::Cycle;
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::fmt;
use std::str;
use std::vec::Vec;
use strum::IntoEnumIterator; // import trait created by EnumIter macro into scope
use strum_macros::EnumIter;

const N_CARDS: usize = 108; // number of cards in standard deck
const N_INITIAL_CARDS: usize = 7; // number of cards in initial player hands
const N_PLAYERS: usize = 4;

// TODO add proper logging
pub fn run() {
    // initialize player cycle
    let n_players = 4;
    let mut players = PlayerCycle::new(n_players);
    println!("Players: {:?}", players.get_names());

    // initialize dealer and player hands
    let mut dealer = Dealer::new();
    let hands = dealer.draw_hands(n_players, N_INITIAL_CARDS);
    players.take_hands(hands);

    // initialize pile
    dealer.flip_first_card();

    // set first card so that actions will be executed at the start of the game
    let mut play = Some(dealer.top_card());

    // cycle through players until game over
    loop {
        // if action card was played, execute card action
        println!("Played: {}", fmt_play(&play));
        if let Some(card) = play {
            match card.symbol {
                Symbol::Skip => players.skip(),
                Symbol::Reverse => players.reverse(),
                Symbol::Draw2 => {
                    let player = players.next();
                    let cards = dealer.draw(2);
                    player.take_cards(cards);
                    println!("Player: {} takes 2 cards", player.name);
                }
                Symbol::WildDraw4 => {
                    let player = players.next();
                    let cards = dealer.draw(4);
                    player.take_cards(cards);
                    println!("Player: {} takes 4 cards", player.name);
                }
                _ => {}
            }
        }

        // pick next player
        let player = players.next();

        // try playing card from hand
        let top_card = dealer.top_card();
        play = player.play_from_hand(&top_card);
        println!("Played from hand: {}", fmt_play(&play));

        // if no card is played, draw a new card and try playing it
        if play.is_none() {
            let new_card = dealer.draw(1);
            println!("Drawn: {}", fmt_card(&new_card[0]));
            play = player.play_from_cards(&top_card, new_card.clone());
            println!("Played from cards: {}", fmt_play(&play));
            // if the new card is not played, take it onto the hand
            if play.is_none() {
                player.take_cards(new_card)
            }
        }

        // if card, discard and check game over
        if let Some(card) = play {
            dealer.discard(card);
            if game_over(player) {
                println!("Player: {} won! Game over.", player.name);
                break;
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
type Play = Option<Card>;

fn generate_players(n_players: usize) -> Players {
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
#[derive(Hash, Eq, Debug, Clone, Copy, EnumIter, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn generate_deck() -> Deck {
    let numbers: [u8; 19] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let color_symbols: [Symbol; 6] = [
        Symbol::Draw2,
        Symbol::Draw2,
        Symbol::Reverse,
        Symbol::Reverse,
        Symbol::Skip,
        Symbol::Skip,
    ];
    let wild_symbols: [Symbol; 8] = [
        Symbol::Wild,
        Symbol::Wild,
        Symbol::Wild,
        Symbol::Wild,
        Symbol::WildDraw4,
        Symbol::WildDraw4,
        Symbol::WildDraw4,
        Symbol::WildDraw4,
    ];

    let mut cards: Cards = Vec::with_capacity(N_CARDS);

    // color cards
    for color in Color::iter() {
        for number in numbers.into_iter() {
            let card = Card {
                symbol: Symbol::Number(number),
                color: Some(color),
            };
            cards.push(card);
        }
        for symbol in color_symbols.into_iter() {
            let card = Card {
                symbol,
                color: Some(color),
            };
            cards.push(card);
        }
    }

    // wild cards
    for symbol in wild_symbols.into_iter() {
        let card = Card {
            symbol,
            color: None,
        };
        cards.push(card);
    }

    // shuffle deck
    cards = randomly_shuffle_cards(cards);

    // return as deque type
    VecDeque::from(cards)
}

/// Randomly shuffle cards.
fn randomly_shuffle_cards(mut cards: Cards) -> Cards {
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    cards
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    Number(u8),
    Skip,
    Reverse,
    Draw2,
    Wild,
    WildDraw4,
}

// add display trait to convert symbol enum values to string using `to_string`
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// define card object, with optional color field to handle wild cards where
// color is chosen by player when the card is played
#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct Card {
    symbol: Symbol,
    color: Option<Color>,
}

impl Card {
    fn is_wild(&self) -> bool {
        matches!(self.symbol, Symbol::Wild | Symbol::WildDraw4)
    }

    fn is_wild_draw_4(&self) -> bool {
        matches!(self.symbol, Symbol::WildDraw4)
    }

    // TODO identify cards better so that we don't need to rely on this function
    fn is_equal_ignore_wild_color(&self, other: &Card) -> bool {
        match self.is_wild() {
            true => self.symbol == other.symbol,
            false => self.symbol == other.symbol && self.color == other.color,
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt_card(self))
    }
}

fn fmt_card(card: &Card) -> ColoredString {
    // remove Number wrapper when printing
    let symbol = match card.symbol {
        Symbol::Number(number) => number.to_string(),
        _ => card.symbol.to_string(),
    };

    match card.color {
        None => symbol.white(),
        Some(color) => match color {
            Color::Red => symbol.red(),
            Color::Blue => symbol.blue(),
            Color::Green => symbol.green(),
            Color::Yellow => symbol.yellow(),
        },
    }
}

fn fmt_play(play: &Play) -> ColoredString {
    match play {
        Some(card) => fmt_card(card),
        None => String::from("None").white(),
    }
}

fn filter_legal_cards(cards: Cards, top_card: Card) -> Cards {
    debug_assert!(!cards.is_empty());
    debug_assert!(top_card.color.is_some());

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

/// A single player, containing a name, the hand of cards, and a strategy how to play cards.
struct Player {
    name: &'static str,
    hand: Cards,
    strategy: Box<dyn Strategy>, // any object implementing the strategy trait
}

impl Player {
    fn new(name: &'static str) -> Self {
        // TODO expose strategy to constructor
        let strategy = RandomStrategy {};
        let hand: Cards = vec![]; // start with empty hand
        Self {
            name,
            hand,
            strategy: Box::new(strategy),
        }
    }

    /// Take `cards` into hand.
    fn take_cards(&mut self, cards: Cards) {
        debug_assert!(!cards.is_empty());
        self.hand.extend(cards);
    }

    /// Play card from `playable_cards` if possible for given `top_card`.
    fn play_from_cards(&self, top_card: &Card, cards: Cards) -> Play {
        debug_assert!(!cards.is_empty());
        let legal_cards = filter_legal_cards(cards, *top_card);
        let legal_cards = remove_duplicates(legal_cards);
        match legal_cards.is_empty() {
            true => None,
            false => self.strategy.select_card(legal_cards),
        }
    }

    /// Play card from hand if possible for given `top_card`.
    fn play_from_hand(&mut self, top_card: &Card) -> Play {
        let cards = self.hand.clone();
        let card = self.play_from_cards(top_card, cards);

        // remove card from hand
        if let Some(card) = card {
            let index = self
                .hand
                .iter()
                .position(|x| x.is_equal_ignore_wild_color(&card))
                .expect("selected card not in hand");
            self.hand.remove(index);
        }

        // say "uno" if one card left
        if card.is_some() && self.hand.len() == 1 {
            println!("Player {:?}: Uno!", self.name);
        }

        card
    }
}

fn remove_duplicates(cards: Cards) -> Cards {
    cards.into_iter().unique().collect()
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
        let player = self.players.get_mut(index).expect("no players");
        println!("Turn: {} | Player: {}", self.cycle.turn(), player.name,);
        player
    }

    /// Reverse player cycle.
    fn reverse(&mut self) {
        println!("Player cycle reversed.");
        self.cycle.reverse();
    }

    /// Skip player.
    fn skip(&mut self) {
        let player = self.next();
        println!("Player: {} skipped", player.name);
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
}

/// Strategy trait defining method for selecting a card to play.
trait Strategy {
    /// Select card from `legal_cards`.
    // TODO pass on play history for enabling strategies to make smarter decisions
    fn select_card(&self, legal_cards: Cards) -> Play;
}

// TODO implement more strategies
/// Random strategy.
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
    fn select_card(&self, legal_cards: Cards) -> Play {
        debug_assert!(!legal_cards.is_empty());
        let mut rng = rand::thread_rng();
        let mut card = *legal_cards.choose(&mut rng).expect("empty legal cards");
        if card.is_wild() {
            // if wild card, select color
            debug_assert!(card.color.is_none());
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
        debug_assert!(n_cards < (n_pile + n_available - 1));

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
        debug_assert!(
            n_available >= n_cards,
            "n_available: {}, n_cards: {}",
            n_available,
            n_cards
        );
        let start = n_available - n_cards;
        self.deck.drain(start..).collect()
    }

    /// Discard `cards` onto discard pile.
    fn discard(&mut self, card: Card) {
        debug_assert!(card.color.is_some());
        self.pile.push(card);
    }

    /// Flip first card of deck onto pile to start the game, discarding wild cards.
    fn flip_first_card(&mut self) {
        // if the card is a wild card, it is returned to the deck and a new card is drawn.
        let card = loop {
            // take first element of vector without copy, destroying vector
            let card = self.draw(1).into_iter().nth(0).expect("no cards drawn");
            match card.is_wild() {
                true => self.refill_deck(vec![card]),
                false => break card,
            }
        };
        self.discard(card);
    }

    /// Refill deck with `cards`.
    fn refill_deck(&mut self, cards: Cards) {
        for mut card in cards {
            // reset color of wild cards
            if card.is_wild() {
                card.color = None;
            }
            self.deck.push_front(card)
        }
    }

    /// Recyle all cards except top card from discard pile into deck.
    fn recycle_pile(&mut self) {
        let n = self.pile.len();
        debug_assert!(n > 0); // pile must have at least one card
        let end = self.pile.len() - 1; // keep top card
        let mut cards = self.pile.drain(0..end).collect();
        cards = randomly_shuffle_cards(cards);
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

    // helper function for testing to generate cards
    fn generate_cards(values: Vec<(Symbol, Option<Color>)>) -> Cards {
        let n = values.len();
        let mut cards = Vec::with_capacity(n);
        for (symbol, color) in values.into_iter() {
            let card = Card { symbol, color };
            cards.push(card);
        }
        cards
    }

    #[test]
    fn test_dealer_flip_first_card() {
        let mut dealer = Dealer::new();
        assert_eq!(dealer.pile.len(), 0);

        let n_before = dealer.deck.len();
        dealer.flip_first_card();
        let n_after = dealer.deck.len();

        assert_eq!(dealer.pile.len(), 1);
        assert_eq!(n_before - n_after, 1);
    }

    #[test]
    fn test_dealer_flip_first_card_wild_cards() {
        let mut dealer = Dealer::new();

        // add wild cards to back of deck
        let first_cards = generate_cards(vec![
            (Symbol::Number(0), Some(Color::Red)),
            (Symbol::Wild, None),
            (Symbol::WildDraw4, None),
        ]);
        for card in first_cards.clone() {
            dealer.deck.push_back(card)
        }

        assert!(dealer.pile.is_empty());
        dealer.flip_first_card();

        // check top card
        assert!(!dealer.top_card().is_wild());
        assert_eq!(dealer.top_card(), first_cards[0]);

        // check discarded wild cards
        assert_eq!(dealer.deck[0], first_cards[1]);
        assert_eq!(dealer.deck[1], first_cards[2]);
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

        // draw most cards from deck
        let mut dealer = Dealer::new();
        let mut _cards = dealer.draw(100);

        // set color for discard to work
        for mut _card in _cards.into_iter() {
            if _card.is_wild() {
                _card.color = Some(select_random_color());
            }
            dealer.discard(_card);
        }
        let n_available = dealer.deck.len();

        // draw more cards than remaining in deck
        let top_card = dealer.top_card();
        assert!(n > n_available);
        let cards = dealer.draw(n);

        assert_eq!(cards.len(), n); // check all requested cards were drawn
        assert_eq!(top_card, dealer.top_card()); // check top card stays the same
    }

    #[test]
    fn test_filter_legal_cards_top_card_red_1() {
        let top_card = Card {
            symbol: Symbol::Number(1),
            color: Some(Color::Red),
        };

        let cards = generate_cards(vec![
            // legal, same symbol
            (Symbol::Number(1), Some(Color::Red)), // same card
            (Symbol::Number(1), Some(Color::Blue)),
            (Symbol::Number(1), Some(Color::Green)),
            // legal, same color
            (Symbol::Number(2), Some(Color::Red)),
            (Symbol::Draw2, Some(Color::Red)),
            (Symbol::Skip, Some(Color::Red)),
            // legal, wild
            (Symbol::Wild, None),
            // illegal
            (Symbol::Number(0), Some(Color::Green)),
            (Symbol::Number(3), Some(Color::Green)),
            (Symbol::WildDraw4, None),
        ]);
        let legal_cards = filter_legal_cards(cards.clone(), top_card);
        assert_eq!(legal_cards, cards[..=6]);
    }

    #[test]
    fn test_filter_legal_cards_top_card_blue_3() {
        let top_card = Card {
            symbol: Symbol::Number(3),
            color: Some(Color::Blue),
        };
        let cards = generate_cards(vec![
            // legal, same symbol
            (Symbol::Number(3), Some(Color::Red)),
            (Symbol::Number(3), Some(Color::Yellow)),
            (Symbol::Number(3), Some(Color::Green)),
            // legal, same color
            (Symbol::Number(2), Some(Color::Blue)),
            (Symbol::Draw2, Some(Color::Blue)),
            (Symbol::Skip, Some(Color::Blue)),
            // legal, wild
            (Symbol::Wild, None),
            // illegal
            (Symbol::Number(4), Some(Color::Green)),
            (Symbol::Number(9), Some(Color::Yellow)),
            (Symbol::WildDraw4, None),
        ]);
        let legal_cards = filter_legal_cards(cards.clone(), top_card);
        assert_eq!(legal_cards, cards[..=6]);
    }

    #[test]
    fn test_filter_legal_cards_top_card_yellow_skip() {
        let top_card = Card {
            symbol: Symbol::Skip,
            color: Some(Color::Yellow),
        };
        let cards = generate_cards(vec![
            // legal, same symbol
            (Symbol::Skip, Some(Color::Red)),
            (Symbol::Skip, Some(Color::Yellow)),
            (Symbol::Skip, Some(Color::Green)),
            // legal, same color
            (Symbol::Number(9), Some(Color::Yellow)),
            (Symbol::Draw2, Some(Color::Yellow)),
            (Symbol::Skip, Some(Color::Yellow)), // same card
            // legal, wild
            (Symbol::Wild, None),
            // illegal
            (Symbol::Number(6), Some(Color::Green)),
            (Symbol::Number(2), Some(Color::Red)),
            (Symbol::WildDraw4, None),
        ]);
        let legal_cards = filter_legal_cards(cards.clone(), top_card);
        assert_eq!(legal_cards, cards[..=6]);
    }

    #[test]
    fn test_filter_legal_cards_no_color_matches_wild_draw_4() {
        let top_card = Card {
            symbol: Symbol::Number(0),
            color: Some(Color::Green),
        };
        let cards = generate_cards(vec![
            // legal, same symbol
            (Symbol::Number(0), Some(Color::Red)),
            (Symbol::Number(0), Some(Color::Yellow)),
            // legal, wild
            (Symbol::Wild, None),
            (Symbol::WildDraw4, None),
            // illegal
            (Symbol::Number(6), Some(Color::Blue)),
            (Symbol::Number(2), Some(Color::Red)),
        ]);
        let legal_cards = filter_legal_cards(cards.clone(), top_card);
        assert_eq!(legal_cards, cards[..=3]);
    }
}
