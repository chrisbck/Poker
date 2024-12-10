use crate::card_dealer::{Deck, Card};

pub struct GameController{
    deck: Deck,
    current_stage: GameStage,
    // Add other game specific fields here
    // such as players, pot size, etc.
}

impl GameController{
    pub fn new() -> Self{
        Self {
            deck: Deck::new(),
            current_stage: GameStage::PreFlop,
            // Initilise other fields...
         }
    }

    pub fn deal(&mut self, count: usize) -> Option<Vec<Card>> {
        self.deck.deal(count)
    }

    pub fn reset_deck(&mut self){
        self.deck.reset();
    }
}

#[derive(Debug)]
pub enum GameStage{
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}
