use std::io;
extern crate rand;
use rand::seq::SliceRandom;
mod deck;

//Card Object
struct Card {
    suit: String,
    rank: String,
}

//Deck Object
struct Deck {
    cards: Vec<Card>,
}

//implement deck methods
impl Deck {
    fn new() -> Self {
        // Create and initialize the deck with all 52 cards
        let mut cards = Vec::new();
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];
    
        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(Card {
                    suit: String::from(*suit),
                    rank: String::from(*rank),
                });
            }
        }
    
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn show_remaining_cards(&self) {
        println!("Remaining cards in the deck:");
        for card in &self.cards {
            println!("{} of {}", card.rank, card.suit);
        }
    }
    
}

//deal card function
fn deal_card(deck: &mut Deck) -> Option<Card> {
    // Check if there are cards left to deal
    if !deck.cards.is_empty() {
        // Remove and return the top card from the deck
        Some(deck.cards.remove(0))
    } else {
        // No cards left to deal
        None
    }
}

fn main() {
    println!("Welcome to Rust Blackjack!");

    let mut deck = Deck::new();
    deck.shuffle();

    // Deal cards to players
    for _ in 0..5 {
        if let Some(card) = deal_card(&mut deck) {
            println!("Dealt card: {} of {}", card.rank, card.suit);
        } else {
            println!("No cards left in the deck.");
            break;
        }
    }


    // Show the remaining cards
    deck.show_remaining_cards();










//    // Create a vector to represent the game steps
//    let game_steps = vec!["Licensing", "Player action", "Dealer action", "Settlement card"];
//    let mut current_step = 0;

//    loop {
//        // Display the current game step
//        println!("Current step: {}", game_steps[current_step]);


//         if current_step == 0 {
//             println!("Licensing completed!");
//         }
        




//        // Wait for the user to press Enter to proceed to the next step
//        println!("Press Enter to proceed to the next step...");
//        let mut input = String::new();
//        io::stdin()
//            .read_line(&mut input)
//            .expect("Failed to read line");

//        // Increment the step index, or exit if all steps are completed
//        current_step += 1;
//        if current_step >= game_steps.len() {
//            println!("Game completed!");
//            break;
//        }
//    }
}
