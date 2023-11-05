
/////////////////////////////////////////
use cap_std::fs::Dir;
use serde_json;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Card {
    suit: String,
    rank: String,
}

// Add more game state as needed
#[derive(Serialize, Deserialize)]
struct GameState {
    player_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
}


fn save_game_state(game_state: &GameState, _path: &Path) -> Result<(), Box<dyn Error>> {
    let serialized_state = serde_json::to_string(game_state)?;
    let dir = Dir::open_ambient_dir("./", cap_std::ambient_authority())?;
    dir.write("game_state.json", serialized_state)?;
    Ok(())
}

fn load_game_state(_path: &Path) -> Result<GameState, Box<dyn Error>> {
    let dir = Dir::open_ambient_dir("./", cap_std::ambient_authority())?;
    let serialized_state = dir.read_to_string("game_state.json")?;
    let game_state = serde_json::from_str(&serialized_state)?;
    Ok(game_state)
}

fn main() {
     //initialization of hands for the example
     let player_hand = vec![
        Card {
            suit: "Hearts".to_string(),
            rank: "10".to_string(),
        },
        Card {
            suit: "Spades".to_string(),
            rank: "Ace".to_string(),
        },
    ];

    let dealer_hand = vec![
        Card {
            suit: "Clubs".to_string(),
            rank: "3".to_string(),
        },
        Card {
            suit: "Diamonds".to_string(),
            rank: "7".to_string(),
        },
    ];

    let game_state = GameState {
        player_hand, // using the initialized player's hand
        dealer_hand, // using the initialized dealer's hand
    };

    let path = PathBuf::from("game_save.json");

    // Save the game state
    if let Err(e) = save_game_state(&game_state,&path) {
        eprintln!("Failed to save game state: {}", e);
    }

    // Later... Load the game state
    match load_game_state(&path) {
        Ok(loaded_game_state) => {
            // Resume game with loaded state
            // For example, print out the hands to verify
            println!("Player's Hand: {:?}", loaded_game_state.player_hand);
            println!("Dealer's Hand: {:?}", loaded_game_state.dealer_hand);
        },
        Err(e) => {
            eprintln!("Failed to load game state: {}", e);
        }
    }
}

// use std::io;
// extern crate rand;
// use rand::seq::SliceRandom;
// extern crate cap_std;
// use serde::{Deserialize, Serialize};
// use std::io::{Read, Write};

// //Card Object
// struct Card {
//     suit: String,
//     rank: String,
// }

// //Deck Object
// struct Deck {
//     cards: Vec<Card>,
//     count: usize,
// }

// //implement deck methods
// impl Deck {
//     fn new() -> Self {
//         // Create and initialize the deck with all 52 cards
//         let mut cards = Vec::new();
//         let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
//         let ranks = vec![
//             "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
//         ];
    
//         for suit in suits.iter() {
//             for rank in ranks.iter() {
//                 cards.push(Card {
//                     suit: String::from(*suit),
//                     rank: String::from(*rank),
//                 });
//             }
//         }

//         let count = cards.len();

//         Deck { cards, count }
//     }

//     fn shuffle(&mut self) {
//         let mut rng = rand::thread_rng();
//         self.cards.shuffle(&mut rng);
//     }

//     fn show_remaining_cards(&self) {
//         println!("Remaining cards in the deck ({} total cards):", self.count);
//         // for card in &self.cards {
//         //     println!("{} of {}", card.rank, card.suit);
//         // }
//     }
    
// }

// //deal card function
// fn deal_card(deck: &mut Deck) -> Option<Card> {
//     // Check if there are cards left to deal
//     if !deck.cards.is_empty() {
//         // Remove and return the top card from the deck
//         let card = deck.cards.remove(0);
//         deck.count -= 1; // Decrement the count
//         Some(card)
//     } else {
//         // No cards left to deal
//         None
//     }
// }

// fn show_hands(player_hand: &[Card], dealer_hand: &[Card]) {

//     println!("Dealer's hand:");
//     for card in dealer_hand {
//         println!("{} of {}", card.rank, card.suit);
//     }

//     println!("Player's hand:");
//     for card in player_hand {
//         println!("{} of {}", card.rank, card.suit);
//     }
// }


// fn show_player_hands(player_hand: &[Card]) {
//     println!("Player's hand:");
//     for card in player_hand {
//         println!("{} of {}", card.rank, card.suit);
//     }
// }

// fn calculate_hand_value(hand: &[Card]) -> u32 {
//     let mut value = 0;
//     let mut num_aces = 0;

//     for card in hand {
//         match card.rank.as_str() {
//             "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => {
//                 value += card.rank.parse::<u32>().unwrap();
//             }
//             "Jack" | "Queen" | "King" => {
//                 value += 10;
//             }
//             "Ace" => {
//                 value += 11;
//                 num_aces += 1;
//             }
//             _ => {}
//         }
//     }

//     // Adjust for Aces if the total value exceeds 21
//     while num_aces > 0 && value > 21 {
//         value -= 10;
//         num_aces -= 1;
//     }

//     value
// }


// // main function
// fn main() {
//     println!("Welcome to Rust Blackjack!");

//     let mut deck = Deck::new();
//     deck.shuffle();

//     // create two vector one hold player hand and one holding dealer hand
//     let mut player_hand: Vec<Card> = Vec::new(); // Create an empty collection for the player's hand
//     let mut dealer_hand: Vec<Card> = Vec::new(); // Create an empty collection for the dealer's hand

//     //licensing
//     // Deal two cards to the player
//     for _ in 0..2 {
//         if let Some(card) = deal_card(&mut deck) {
//             player_hand.push(card);
//         } else {
//             println!("No cards left in the deck.");
//             return;
//         }
//     }

//     // Deal two cards to the dealer
//     for _ in 0..2 {
//         if let Some(card) = deal_card(&mut deck) {
//             dealer_hand.push(card);
//         } else {
//             println!("No cards left in the deck.");
//             return;
//         }
//     }

//     // Show the dealer's hand with the first card hidden
//     println!("Dealer's hand:");
//     for i in 1..dealer_hand.len() {
//         println!("{} of {}", dealer_hand[i].rank, dealer_hand[i].suit);
//     }

//     // Display the initial hands of the player and dealer
//     println!("Player's hand:");
//     for card in &player_hand {
//         println!("{} of {}", card.rank, card.suit);
//     }

//     loop {
//         println!();
//         println!();
//         println!("Press 1 to take a card: ");
//         println!("Press 2 to final score your hand: ");
//         println!("Press 3 to leave table: ");
//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).expect("Failed to read input.");
        
//         match choice.trim() {
//             "1" => { 
//                 // player deal
//                 if let Some(card) = deal_card(&mut deck) {
//                     player_hand.push(card);
//                     let player_value = calculate_hand_value(&player_hand);
//                     show_player_hands(&player_hand);
//                     println!("Player's hand value: {}", player_value);
//                     println!();
//                     println!();
//                     if player_value > 21 {
//                         println!("Player's hand value is over 21. You lose!");
//                         return;
//                     }
//                 } else {
//                     println!("No cards left in the deck.");
//                 }
//                 // dealer deal cards
//                 let dealer_value = calculate_hand_value(&dealer_hand);
//                 if dealer_value <=17 {
//                     if let Some(card) = deal_card(&mut deck) {
//                         dealer_hand.push(card);
//                     } else {
//                         println!("No cards left in the deck.");
//                     }
//                 }
    
//             },
//             "2" => {
//                 let dealer_value = calculate_hand_value(&dealer_hand);
//                 if dealer_value > 21 {
//                     println!("you win !");
//                 }
//                 if dealer_value <= 21 {
//                     let player_value = calculate_hand_value(&player_hand);
//                     println!("Dealer have: {}", dealer_value);
//                     println!();
//                     println!("Dealer's hand:");
//                     for card in &dealer_hand {
//                         println!("{} of {}",card.rank, card.suit);
//                     }
//                     println!();
//                     println!();

//                     println!("Player have: {}", player_value);
//                     println!();    
//                     // Display the initial hands of the player and dealer
//                     println!("Player's hand:");
//                     for card in &player_hand {
//                         println!("{} of {}", card.rank, card.suit);
//                     }
//                     println!();
//                     if player_value > dealer_value {
//                         println!("you win !");
//                     } else if player_value == dealer_value {
//                         println!("same score !");
//                     } else {
//                         println!("you lose !");
//                     }
//                 }
//                 return; // Player chooses to stop
//             },
//             "3" => {
//                 println!("Quitting the game.");
//                 return; // Quit, end the main function
//             },
//             _ => {
//                 println!("Invalid choice. Please press 1 to take a card or 2 to stop.");
//             },
//         }
//     }
//     // Proceed with the game logic from here
// }