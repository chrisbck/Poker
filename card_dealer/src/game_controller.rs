use crate::card_dealer::{Card, Deck};
use crate::poker_hand::{find_best_hand, HandRank};

pub struct GameController {
    deck: Deck,
    community_cards: Vec<Card>,       // Holds all 5 community cards
    player_hole_cards: Vec<Card>,    // Single player's hole cards
    last_evaluated_hand: Option<HandRank>, // Stores the most recent evaluation
}

impl GameController {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            community_cards: Vec::new(),
            player_hole_cards: Vec::new(),
            last_evaluated_hand: None,
        }
    }

    pub fn deal_hole_cards(&mut self) -> Option<Vec<Card>> {
        if let Some(cards) = self.deck.deal(2) {
            self.player_hole_cards = cards.clone(); // Overwrite existing hole cards
            Some(cards)
        } else {
            None
        }
    }

    pub fn deal_community_cards(&mut self) -> Option<Vec<Card>> {
        if let Some(cards) = self.deck.deal(5) {
            self.community_cards = cards.clone(); // Overwrite existing community cards
            self.evaluate_best_hand();
            Some(cards)
        } else {
            None
        }
    }

    pub fn reset_deck(&mut self) {
        self.deck.reset();
        self.community_cards.clear();
        self.player_hole_cards.clear();
        self.last_evaluated_hand = None;
    }

    pub fn get_last_evaluated_hand(&self) -> Option<HandRank> {
        self.last_evaluated_hand.clone()
    }

    /// Evaluate the best hand combining hole cards and community cards
    fn evaluate_best_hand(&mut self) {
        let mut combined_cards = self.community_cards.clone();
        combined_cards.extend(self.player_hole_cards.clone());

        if combined_cards.len() >= 5 {
            self.last_evaluated_hand = Some(find_best_hand(&combined_cards));
        } else {
            self.last_evaluated_hand = None;
        }
    }
}
