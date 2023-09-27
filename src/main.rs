use std::io;
fn main() {
    println!("Welcome to Rust Blackjack!");

   // Create a vector to represent the game steps
   let game_steps = vec!["Licensing", "Player action", "Dealer action", "Settlement card"];
   let mut current_step = 0;

   loop {
       // Display the current game step
       println!("Current step: {}", game_steps[current_step]);


        if current_step == 0 {
            println!("Licensing completed!");
        }
        




       // Wait for the user to press Enter to proceed to the next step
       println!("Press Enter to proceed to the next step...");
       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read line");

       // Increment the step index, or exit if all steps are completed
       current_step += 1;
       if current_step >= game_steps.len() {
           println!("Game completed!");
           break;
       }
   }
}
