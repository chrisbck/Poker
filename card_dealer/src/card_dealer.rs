use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

// Card enums
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck of cards
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                cards.push(Card { rank, suit });
            }
        }
        // Shuffle the deck
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    /// Deals `count` cards from the deck
    pub fn deal(&mut self, count: usize) -> Option<Vec<Card>> {
        if count > self.cards.len() {
            return None; // Not enough cards remaining
        }
        Some(self.cards.drain(0..count).collect()) // Return the cards
    }

    /// Returns the number of remaining cards
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Resets the deck to a full shuffled state
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_initialization() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52); // A new deck should have 52 cards
    }

    #[test]
    fn test_dealing_cards() {
        let mut deck = Deck::new();
        let dealt = deck.deal(5).unwrap(); // Deal 5 cards
        assert_eq!(dealt.len(), 5);        // Check that 5 cards were dealt
        assert_eq!(deck.remaining(), 47); // Remaining cards should be 47
    }

    #[test]
    fn test_not_enough_cards() {
        let mut deck = Deck::new();
        let _ = deck.deal(50);           // Deal most of the deck
        assert!(deck.deal(5).is_none()); // Not enough cards to deal 5 more
    }

    #[test]
    fn test_deck_reset() {
        let mut deck = Deck::new();
        let _ = deck.deal(10);      // Deal 10 cards
        deck.reset();               // Reset the deck
        assert_eq!(deck.remaining(), 52); // Deck should be full again
    }

    #[test]
    fn test_deal_zero() {
        let mut deck = Deck::new();
        let dealt = deck.deal(0).unwrap();
        assert_eq!(dealt.len(), 0);
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deal_more_than_remaining() {
        let mut deck = Deck::new();
        let dealt = deck.deal(53);
        assert!(dealt.is_none());
    }

    #[test]
    fn test_multiple_deals() {
        let mut deck = Deck::new(); // Initialize the deck

        // First deal: 10 cards
        let first_deal = deck.deal(10).unwrap();
        assert_eq!(first_deal.len(), 10); // Verify 10 cards were dealt
        assert_eq!(deck.remaining(), 42); // 52 - 10 = 42

        // Second deal: 15 cards
        let second_deal = deck.deal(15).unwrap();
        assert_eq!(second_deal.len(), 15); // Verify 15 cards were dealt
        assert_eq!(deck.remaining(), 27); // 42 - 15 = 27

        // Third deal: 20 cards
        let third_deal = deck.deal(20).unwrap();
        assert_eq!(third_deal.len(), 20); // Verify 20 cards were dealt
        assert_eq!(deck.remaining(), 7); // 27 - 20 = 7

        // Final check: Remaining cards
        let remaining_cards = deck.deal(7).unwrap();
        assert_eq!(remaining_cards.len(), 7); // All remaining cards dealt
        assert_eq!(deck.remaining(), 0); // No cards left
    }
}



// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_initialization() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52); // A new deck should have 52 cards
    }

    #[test]
    fn test_dealing_cards() {
        let mut deck = Deck::new();
        let dealt = deck.deal(5).unwrap(); // Deal 5 cards
        assert_eq!(dealt.len(), 5);        // Check that 5 cards were dealt
        assert_eq!(deck.remaining(), 47); // Remaining cards should be 47
    }

    #[test]
    fn test_not_enough_cards() {
        let mut deck = Deck::new();
        let _ = deck.deal(50);           // Deal most of the deck
        assert!(deck.deal(5).is_none()); // Not enough cards to deal 5 more
    }

    #[test]
    fn test_deck_reset() {
        let mut deck = Deck::new();
        let _ = deck.deal(10);      // Deal 10 cards
        deck.reset();               // Reset the deck
        assert_eq!(deck.remaining(), 52); // Deck should be full again
    }

    #[test]
    fn test_deal_zero(){
        let mut deck = Deck::new();
        let dealt = deck.deal(0).unwrap();
        assert_eq!(dealt.len(), 0);
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deal_more_than_remaining(){
        let mut deck = Deck::new();
        let dealt = deck.deal(53);
        assert!(dealt.is_none());
    }
    
    #[test]
    fn test_multiple_deals() {
        let mut deck = Deck::new(); // Initialize the deck
    
        // First deal: 10 cards
        let first_deal = deck.deal(10).unwrap();
        assert_eq!(first_deal.len(), 10); // Verify 10 cards were dealt
        assert_eq!(deck.remaining(), 42); // 52 - 10 = 42
    
        // Second deal: 15 cards
        let second_deal = deck.deal(15).unwrap();
        assert_eq!(second_deal.len(), 15); // Verify 15 cards were dealt
        assert_eq!(deck.remaining(), 27); // 42 - 15 = 27
    
        // Third deal: 20 cards
        let third_deal = deck.deal(20).unwrap();
        assert_eq!(third_deal.len(), 20); // Verify 20 cards were dealt
        assert_eq!(deck.remaining(), 7); // 27 - 20 = 7
    
        // Final check: Remaining cards
        let remaining_cards = deck.deal(7).unwrap();
        assert_eq!(remaining_cards.len(), 7); // All remaining cards dealt
        assert_eq!(deck.remaining(), 0); // No cards left
    }
    

}