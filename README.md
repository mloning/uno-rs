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
* pile 
* player hand
* player cycle (current player and direction)
* color for wild cards
* potentially player strategy (play history of other players)

Some observations:

* top card references pile but pile changes during turn when played card is discarded 
* when empty, deck recycles all cards from pile except top card
* card actions are only executed once after the card is played, even though an action card may stay as top card for various turns
* we need to distinguish between playable cards and legal cards; playable cards are usually the player's hand or when no card was played and a new card is drawn, the newly drawn hand, whereas legal cards are the cards that can be played given the top card (symbol/color matches and special rules for wild-draw-4 cards)
* we need to identify cards, e.g. when playing a card, we first select a card from the legal cards and then need to remove it from the player's hand before returning it; we could pass a mutable reference to the player's hand and pop the selected card from the hand, on the other hand, the legal cards are usually a subset of the hand and the selection algorithm should only see (unique) legally playable cards; a further complication is that for wild cards, the selected card has a selected color, whereas the card on the player's hand is still colorless
* the player or turn cycle needs to start with the first player and infinitely cycle through the players, it must be reversible, returning from the current player to the previous player instead of the next player, reversing the cycle before the first turn should make the last player the first player
