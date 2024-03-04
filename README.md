# Rust implementation of Uno card game

For my Python implementation, see this [blog post](https://www.mloning.com/posts/implementing-uno-card-game-in-python) and [repo](https://github.com/mloning/uno-py).

For Uno rules, see [Wikipedia](<https://en.m.wikipedia.org/wiki/Uno_(card_game)>) or these [instructions](<https://service.mattel.com/instruction_sheets/UNO%20Power%20Grab%20Rules.pdf>).

## How to play

Run: `cargo run`

## Development

* `cargo clippy` for linting using [Clippy](https://github.com/rust-lang/rust-clippy)
* `cargo test --lib` run unit tests in library 
* `cargo add <dependency>` to add a new dependency
* `cargo remove <dependency>` to remove a dependency

## Resources

Here are a few resources I found useful while writing the Uno game:

* [Educational blog for Rust beginners](https://github.com/pretzelhammer/rust-blog/tree/master)
* [SO answer on lifetimes](https://stackoverflow.com/a/70674633/9334962)
* [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
* [Programming Rust: Fast, Safe Systems Development](https://www.goodreads.com/book/show/25550614-programming-rust)
* [Rustlings Coding Exercises](https://github.com/rust-lang/rustlings)

## Notes

State involves the following objects:

* deck
* pile (top card)
* player hand
* player cycle (current player and direction)
* color for wild cards
* potentially player strategy (history of plays by other players)

Some observations on state:

* the top card references the pile (shared reference), but the pile changes when played card is discarded at the end of the turn (mutable reference), reference to top card is only needed at the beginning of a turn
* when empty, deck recycles all cards from pile, except the top card
* a card action is only executed once after an action card is played, even though the card may stay on top of the pile for various turn
* we need to distinguish between playable cards and legal cards; playable cards are usually a player's hand; when no card is played and a new card is drawn, the new card will be the playable cards; on the other hand, legal cards are those cards that can legally be played given the top card (symbol/color matches and special rules for wild-draw-4 cards)
* we need to identify cards, e.g. when playing a card, we first select a card from the legal cards and then need to remove it from the player's hand before returning it; we could pass a mutable reference to the player's hand and pop the selected card from the hand; on the other hand, the legal cards are usually a subset of the hand and the selection algorithm should only see (unique) legally playable cards; a further complication is that for wild cards, the selected card has a selected color, whereas the card on the player's hand is still colorless
* the player or turn cycle needs to start with the first player and infinitely cycle through the players; it also needs to be reversible, returning from the current player to the previous player instead of the next player, reversing the cycle before the first turn should make the last player the first player

Some observations on extensibility:

* new cards, especially action cards; this requires implementing both the new card and its action; we would have to define a interface for handling card state (e.g. resetting state when recycling the pile) and actions (e.g. executing an action may force a player to take cards or change the player cycle)
* new player strategy, especially human-input or AI-driven strategies; the strategy interface should take the legal cards and the top card (or a more complete history of plays by other players), and return the next card to play, selected from the playable cards
