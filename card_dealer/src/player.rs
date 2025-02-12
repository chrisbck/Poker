use crate::poker_hand::Hand;
use crate::poker_hand::HandRank;
use crate::poker_hand::find_best_hand;
use crate::card_dealer::Card;


pub struct Player {
    pub player_id: String,                  // Unique identifier, possibly an NFT address
    pub display_name: String,               // Player's display name for UI
    pub hole_cards: Vec<Card>,              // Player's hole cards
    pub hand_strength: Option<HandRank>,    // Best current hand rank
    pub best_hand: Option<Hand>,       // Best current 5-card hand
    pub chip_stack: u32,                    // Player's current chip stack
    pub table_position: usize,              // Position at the table
    pub is_sitting_out: bool,               // Indicates if the player is sitting out
    pub is_in_play: bool,                   // Indicates if the player is active in the current hand
    pub action_history: Vec<PlayerAction>,  // Player's action history
}


#[derive(Debug, Clone)]
pub enum PlayerAction {
    Bet(u32),       // A bet with the amount
    Raise(u32),     // A raise with the amount
    Fold,           // The player folds
    Check,          // The player checks
    Call,           // The player calls
    SitOut,         // The player sits out
}


impl Player {
    /// Creates a new player instance
    pub fn new(player_id: String, display_name: String, table_position: usize, chip_stack: u32) -> Self {
        Self {
            player_id,
            display_name,
            hole_cards: Vec::new(),
            hand_strength: None,
            best_hand: None,
            chip_stack,
            table_position,
            is_sitting_out: false,
            is_in_play: true,
            action_history: Vec::new(),
        }
    }

    /// Evaluates the player's hand strength based on the community cards.
    /// It combines the player's hole cards with the community cards,
    /// and finds the best hand that can be made from them.
    pub fn evaluate_hand(&mut self, community_cards: &[Card]) {
        let mut combined_cards = self.hole_cards.clone();
        combined_cards.extend_from_slice(community_cards);
        self.best_hand = Some(find_best_hand(&combined_cards));
        self.hand_strength = self.best_hand.as_ref().map(|hand| hand.rank.clone());     // Use `map()` to extract rank safely without unwrap
    }

    /// Deducts a bet amount from the player's chip stack
    pub fn bet(&mut self, amount: u32) -> Result<(), String> {
        if amount > self.chip_stack {
            Err("Not enough chips to bet".to_string())
        } else {
            self.chip_stack -= amount;
            self.record_action(PlayerAction::Bet(amount));
            Ok(())
        }
    }

    /// Marks the player as folded for the current hand
    pub fn fold(&mut self) {
        self.is_in_play = false;
        self.hole_cards.clear(); // Optional: Reset cards for clarity
        self.record_action(PlayerAction::Fold);
    }

    /// Combines a bet with an additional raise amount
    pub fn raise(&mut self, current_bet: u32, raise_amount: u32) -> Result<u32, String> {
        let total_bet = current_bet + raise_amount;
        self.bet(total_bet)?;
        self.record_action(PlayerAction::Raise(raise_amount));
        Ok(total_bet)
    }

    /// Marks the player as sitting out
    pub fn sit_out(&mut self) {
        self.is_sitting_out = true;
        self.is_in_play = false;
        self.hole_cards.clear(); // Optional: Reset cards for clarity
        self.record_action(PlayerAction::Fold); // Record as folded for this hand
    }

    /// Resets the player for a new hand
    pub fn reset_for_new_hand(&mut self) {
        self.hole_cards.clear();
        self.is_in_play = !self.is_sitting_out; // Active if not sitting out
        self.hand_strength = None;
        self.clear_action_history();
    }

    /// Adds chips to the player's stack
    pub fn add_chips(&mut self, amount: u32) {
        self.chip_stack += amount;
    }

    /// Records a player's action in the action history
    pub fn record_action(&mut self, action: PlayerAction) {
        self.action_history.push(action);
    }

    /// Clears the player's action history
    pub fn clear_action_history(&mut self) {
        self.action_history.clear();
    }
}
