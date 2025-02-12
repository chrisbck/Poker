use std::collections::HashMap;
use crate::card_dealer::Card; use crate::game_controller::GameController;
// Import Card from card_dealer.rs
use crate::poker_hand::HandRank; // Import HandRank from poker_hand.rs

#[derive(Debug)]
pub struct Table {
    pub community_cards: Vec<Card>,             // Shared cards on the table
    pub pots: Vec<Pot>,                         // Multiple pots for the game
    pub player_bets: HashMap<String, u32>,      // Current round bets (player_id -> amount)
    pub min_bet: u32,                           // Minimum bet for the current round
    pub max_bet: u32,                           // Current maximum bet
}

#[derive(Debug)]
pub struct Pot {
    pub total: u32,                             // Total chips in this pot
    pub eligible_players: Vec<String>,          // Player IDs eligible to win this pot
    pub winners: Option<Vec<String>>,           // Winners of this pot, None if not resolved yet
}

impl Table {
    pub fn new() -> Self {
        Self {
            community_cards: Vec::new(),
            pots: Vec::new(),
            player_bets: HashMap::new(),
            min_bet: 0,
            max_bet: 0,
        }
    }

    /// Adds a player's bet to the table and manages pots
    pub fn add_bet(&mut self, player_id: &str, amount: u32) -> Result<(), String> {
        let mut remaining_amount = amount;

        for pot in &mut self.pots {
            if remaining_amount == 0 {
                break;
            }

            if pot.eligible_players.contains(&player_id.to_string()) {
                let contribution = remaining_amount.min(self.max_bet - self.player_bets.get(player_id).copied().unwrap_or(0));
                pot.total += contribution;
                remaining_amount -= contribution;
            }
        }

        if remaining_amount > 0 {
            // Create a new side pot
            self.pots.push(Pot {
                total: remaining_amount,
                eligible_players: self.active_players(),
                winners: None,
            });
        }

        *self.player_bets.entry(player_id.to_string()).or_insert(0) += amount;

        Ok(())
    }
    

    /// Clears the table for a new round
    pub fn reset_for_new_round(&mut self) {
        self.community_cards.clear();
        self.pots.clear();
        self.player_bets.clear();
        self.min_bet = 0;
        self.max_bet = 0;
    }

    /// Helper function to determine active players
    fn active_players(&self) -> Vec<String> {
        self.player_bets.keys().cloned().collect()
    }
}
