use std::cmp::Ordering;

use crate::card_dealer::{Card, Deck};
use crate::player::Player;
use crate::poker_hand::HandRank;
use crate::table::Table; // Import Table

pub struct GameController {
    deck: Deck,
    community_cards: Vec<Card>,       // Shared cards on the table
    players: Vec<Player>,             // All players in the game
    table: Table,                     // The game table
}

impl GameController {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            community_cards: Vec::new(),
            players: Vec::new(),
            table: Table::new(), // Initialize the table
        }
    }

    /// Initializes players with their names and other parameters
    pub fn initialize_players(
        &mut self,
        player_data: Vec<(String, String, usize, u32)>, // (player_id, display_name, table_position, chip_stack)
    ) {
        self.players = player_data
            .into_iter()
            .map(|(player_id, display_name, table_position, chip_stack)| {
                Player::new(player_id, display_name, table_position, chip_stack)
            })
            .collect();
    }

    /// Deals hole cards to each player
    pub fn deal_hole_cards(&mut self) -> Result<(), String> {
        for player in &mut self.players {
            if let Some(cards) = self.deck.deal(2) {
                player.hole_cards = cards;
            } else {
                return Err("Not enough cards to deal hole cards.".to_string());
            }
        }
        Ok(())
    }

    /// Deals community cards
    pub fn deal_community_cards(&mut self) -> Result<(), String> {
        if let Some(cards) = self.deck.deal(5) {
            self.community_cards = cards;
            self.evaluate_player_hands(); // Evaluate hands after dealing community cards
            Ok(())
        } else {
            Err("Not enough cards to deal community cards.".to_string())
        }
    }

    /// Resets the deck and clears all players' hole cards
    pub fn reset_deck(&mut self) {
        self.deck.reset();
        self.community_cards.clear();
        for player in &mut self.players {
            player.reset_for_new_hand();
        }
    }

    /// Evaluates the best hand for each player
    pub fn evaluate_player_hands(&mut self) {
        for player in &mut self.players {
            player.evaluate_hand(&self.community_cards);
        }

        self.resolve_pots();
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_community_cards(&self) -> &Vec<Card> {
        &self.community_cards
    }

    /// Find the winner(s) amongst the provided player pool
    /// Returns the indexes of the winning players (more than one in case of a tie)
    pub fn get_winners(&self, player_pool: &[String]) -> Option<Vec<String>> {
        let mut best_hand_rank = HandRank::HighCard;
        let mut winners: Vec<String> = Vec::new(); // Renamed from `best_players`
    
        for player in self.get_players() {
            if !player_pool.contains(&player.player_id) {
                continue; // Skip players not in the provided pool
            }
    
            if let Some(ref hand) = player.best_hand {
                if hand.rank > best_hand_rank {
                    // Found a stronger hand, reset winner list
                    best_hand_rank = hand.rank.clone();
                    winners.clear();
                    winners.push(player.player_id.clone());
                } else if hand.rank == best_hand_rank {
                    // Tie: Add player to winners
                    winners.push(player.player_id.clone());
                }
            }
        }
    
        if winners.is_empty() {
            None // Return None if no winners found
        } else {
            Some(winners) // Return Some(Vec<String>) if there are winners
        }
    }


    pub fn resolve_pots(&mut self) {
    // Step 1: Collect winners for each pot BEFORE mutably borrowing `self.table.pots`
    let winners_for_pots: Vec<Option<Vec<String>>> = self.table.pots.iter()
        .map(|pot| self.get_winners(&pot.eligible_players)) // Get winners for each pot
        .collect();

    // Step 2: Mutably iterate over `self.table.pots` AFTER winner data is collected
    for (pot, winners) in self.table.pots.iter_mut().zip(winners_for_pots) {
        pot.winners = winners; // Assign winners to each pot
    }
}



    pub fn get_table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    
}
