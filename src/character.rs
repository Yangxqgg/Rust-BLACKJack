/*I'm planing to write a rust version of the game BLACKJack, 
this is a character module that create the character object for player and dealer */


//object for player
pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

//object for dealer

pub struct Dealer {
    pub name: String,
    hand: Vec<Card>,
}