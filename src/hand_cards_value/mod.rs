/// Represents a playing card.
#[derive(Debug, PartialEq)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

/// Represents a hand of playing cards.
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new, empty hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::hand_cards_value::{Card, Hand};
    ///
    /// let hand = Hand::new();
    /// assert_eq!(hand.value(), 0);
    /// ```
    pub fn new() -> Self {
        Hand { cards: vec![] }
    }

    /// Adds a card to the hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::hand_cards_value::{Card, Hand};
    ///
    /// let mut hand = Hand::new();
    /// hand.add(Card::King);
    /// hand.add(Card::Ace);
    /// assert_eq!(hand.value(), 21);
    /// ```
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Calculates the value of the hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::hand_cards_value::{Card, Hand};
    ///
    /// let mut hand = Hand::new();
    /// hand.add(Card::King);
    /// hand.add(Card::Ace);
    /// assert_eq!(hand.value(), 21);
    /// ```
    pub fn value(&self) -> usize {
        self.cards.iter().fold(0, |acc, card| match card {
            Card::Two => acc + 2,
            Card::Three => acc + 3,
            Card::Four => acc + 4,
            Card::Five => acc + 5,
            Card::Six => acc + 6,
            Card::Seven => acc + 7,
            Card::Eight => acc + 8,
            Card::Nine => acc + 9,
            Card::Jack => acc + 10,
            Card::Queen => acc + 10,
            Card::King => acc + 10,
            Card::Ace => {
                if acc + 11 > 21 {
                    acc + 1
                } else {
                    acc + 11
                }
            }
        })
    }

    /// Determines if the hand is a losing hand (i.e. has a value greater than 21).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::hand_cards_value::{Card, Hand};
    ///
    /// let mut hand = Hand::new();
    /// hand.add(Card::King);
    /// hand.add(Card::Seven);
    /// hand.add(Card::Five);
    /// assert!(hand.is_loosing_hand());
    /// assert_eq!(hand.value(), 22);
    /// ```
    pub fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

#[cfg(test)]
mod hand_cards_test {
    use super::*;
    #[test]
    fn empty_hand() {
        let hand = Hand::new();

        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn strong_hand() {
        let mut hand = Hand::new();
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn risky_hand() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn oops() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Seven);
        hand.add(Card::Five);

        assert!(hand.is_loosing_hand());
        assert_eq!(hand.value(), 22);
    }
}
