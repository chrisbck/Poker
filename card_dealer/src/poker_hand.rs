use crate::card_dealer::Card;
use itertools::Itertools;
use serde::Serialize;
use std::cmp::Ordering;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Clone, Serialize)]
pub struct Hand {
    pub cards: Vec<Card>, // The cards forming the hand
    pub rank: HandRank,   // The rank of the hand (e.g., Full House, Flush)
}

impl Hand {
    /// Creates a new `Hand` instance by evaluating the given cards
    pub fn new(mut cards: Vec<Card>) -> Self {
        cards.sort_by(|a, b| b.rank.cmp(&a.rank)); // Sort by rank descending
        let rank = evaluate_hand(&cards[..]);
        Self { cards, rank }
    }

    /// Compares two hands, including tie-breaking logic
    pub fn compare(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                // Tie-breaking logic for hands with equal rank
                for (card1, card2) in self.cards.iter().zip(&other.cards) {
                    match card1.rank.cmp(&card2.rank) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal // Completely tied
            }
            other => other,
        }
    }
}

pub fn find_best_hand(cards: &[Card]) -> Hand {
    cards
        .iter()
        .combinations(5) // Generate all 5-card combinations
        .map(|combination| {
            let mut combination_cards = combination.into_iter().copied().collect::<Vec<Card>>();
            combination_cards.sort_by(|a, b| b.rank.cmp(&a.rank)); // Sort by rank descending
            Hand::new(combination_cards) // Create a `Hand` for each combination
        })
        .max_by(|hand1, hand2| hand1.compare(hand2)) // Use `compare` for tie-breaking
        .unwrap_or_else(|| Hand::new(vec![])) // Default to an empty hand
}



fn evaluate_hand(hand: &[Card]) -> HandRank {
    let suits = count_suits(hand);
    let ranks = count_ranks(hand);

    let is_flush = check_flush(&suits);
    let is_straight = check_straight(&ranks);

    match (is_flush, is_straight) {
        (true, true) => HandRank::StraightFlush,
        _ if ranks.contains(&4) => HandRank::FourOfAKind,       // Check if any rank has 4 cards
        _ if ranks.contains(&3) && ranks.contains(&2) => HandRank::FullHouse,
        (true, false) => HandRank::Flush,
        (false, true) => HandRank::Straight,
        _ if ranks.contains(&3) => HandRank::ThreeOfAKind,
        _ if ranks.iter().filter(|&&count| count == 2).count() == 2 => HandRank::TwoPair,
        _ if ranks.contains(&2) => HandRank::OnePair,
        _ => HandRank::HighCard,
    }
}


/// Counts the number of occurrences of each suit.
fn count_suits(hand: &[Card]) -> Vec<usize> {
    let mut suits = vec![0; 4];
    for card in hand {
        suits[card.suit as usize] += 1;     // convert the enum to a usize and increment the count
    }
    suits
}

/// Counts the number of occurrences of each rank.
fn count_ranks(hand: &[Card]) -> Vec<usize> {
    let mut ranks = vec![0; 13];
    for card in hand {
        ranks[card.rank as usize] += 1;    // convert the enum to a usize and increment the count
    }
    ranks
}

/// Checks if the hand is a flush (all cards have the same suit).
fn check_flush(suits: &[usize]) -> bool {
    suits.iter().any(|&count| count == 5)   // return true if any suit count == 5
}

/// Checks if the hand is a straight (5 consecutive ranks).
fn check_straight(ranks: &[usize]) -> bool {
    let mut consecutive = 0;
    // Each rank has been counted, so a consecutive straight would look like this: 0,1,1,1,1,1,0,0,0,0,0,0,0
    for &count in ranks {
        if count > 0 {
            consecutive += 1;
            if consecutive == 5 {
                return true;
            }
        } else {
            consecutive = 0;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::card_dealer::{Card, Rank, Suit};

    /// Helper function to create a card
    fn create_card(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn test_best_hand_scenario() {
        // Community cards: 9H, JH, 5C, AS, JD
        let community_cards = vec![
            create_card(Rank::Nine, Suit::Hearts),
            create_card(Rank::Jack, Suit::Hearts),
            create_card(Rank::Five, Suit::Clubs),
            create_card(Rank::Ace, Suit::Spades),
            create_card(Rank::Jack, Suit::Diamonds),
        ];

        // Player 1: 5H, 7S
        let player1_cards = vec![
            create_card(Rank::Five, Suit::Hearts),
            create_card(Rank::Seven, Suit::Spades),
        ];

        // Player 2: KC, 9H
        let player2_cards = vec![
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::Nine, Suit::Hearts),
        ];

        // Player 3: 10D, AH
        let player3_cards = vec![
            create_card(Rank::Ten, Suit::Diamonds),
            create_card(Rank::Ace, Suit::Hearts),
        ];

        // Evaluate the best hand for each player
        let player1_hand = find_best_hand(&[player1_cards.clone(), community_cards.clone()].concat());
        let player2_hand = find_best_hand(&[player2_cards.clone(), community_cards.clone()].concat());
        let player3_hand = find_best_hand(&[player3_cards.clone(), community_cards.clone()].concat());

        // Compare hands to determine the winner
        let mut best_hand = &player1_hand;
        if player2_hand.compare(best_hand) == std::cmp::Ordering::Greater {
            best_hand = &player2_hand;
        }
        if player3_hand.compare(best_hand) == std::cmp::Ordering::Greater {
            best_hand = &player3_hand;
        }

        // Print the results for debugging
        println!("Player 1 Hand: {:?}, Rank: {:?}", player1_hand.cards, player1_hand.rank);
        println!("Player 2 Hand: {:?}, Rank: {:?}", player2_hand.cards, player2_hand.rank);
        println!("Player 3 Hand: {:?}, Rank: {:?}", player3_hand.cards, player3_hand.rank);
        println!("Winning Hand: {:?}, Rank: {:?}", best_hand.cards, best_hand.rank);

        // Assert the correct winner
        assert_eq!(best_hand.rank, HandRank::TwoPair); // The strongest expected hand
    }
}

