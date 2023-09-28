// use rand::seq::SliceRandom;
// use Card;

// pub struct Deck {
//     cards: Vec<Card>,
// }

// impl Deck {
//     // Constructor to create a new deck of 52 cards
//     pub fn new() -> Deck {
//         let mut cards = Vec::new();
//         for suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
//             for value in &[
//                 Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven,
//                 Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace,
//             ] {
//                 cards.push(Card::new(*value, *suit));
//             }
//         }
//         Deck { cards }
//     }

//     // Shuffle the deck
//     pub fn shuffle(&mut self) {
//         let mut rng = rand::thread_rng();
//         self.cards.shuffle(&mut rng);
//     }

//     // Draw a random card from the deck
//     pub fn draw(&mut self) -> Option<Card> {
//         self.cards.pop()
//     }
// }
