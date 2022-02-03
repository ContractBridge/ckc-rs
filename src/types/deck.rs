use crate::{CardNumber, PokerCard};

/// Represents a Standard52 deck as an immutable array of
/// Cactus Kev Cards (`PokerCard`).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Deck([PokerCard; 52]);

pub const POKER_DECK: Deck = Deck([
    CardNumber::ACE_SPADES,
    CardNumber::KING_SPADES,
    CardNumber::QUEEN_SPADES,
    CardNumber::JACK_SPADES,
    CardNumber::TEN_SPADES,
    CardNumber::NINE_SPADES,
    CardNumber::EIGHT_SPADES,
    CardNumber::SEVEN_SPADES,
    CardNumber::SIX_SPADES,
    CardNumber::FIVE_SPADES,
    CardNumber::FOUR_SPADES,
    CardNumber::TREY_SPADES,
    CardNumber::DEUCE_SPADES,
    CardNumber::ACE_HEARTS,
    CardNumber::KING_HEARTS,
    CardNumber::QUEEN_HEARTS,
    CardNumber::JACK_HEARTS,
    CardNumber::TEN_HEARTS,
    CardNumber::NINE_HEARTS,
    CardNumber::EIGHT_HEARTS,
    CardNumber::SEVEN_HEARTS,
    CardNumber::SIX_HEARTS,
    CardNumber::FIVE_HEARTS,
    CardNumber::FOUR_HEARTS,
    CardNumber::TREY_HEARTS,
    CardNumber::DEUCE_HEARTS,
    CardNumber::ACE_DIAMONDS,
    CardNumber::KING_DIAMONDS,
    CardNumber::QUEEN_DIAMONDS,
    CardNumber::JACK_DIAMONDS,
    CardNumber::TEN_DIAMONDS,
    CardNumber::NINE_DIAMONDS,
    CardNumber::EIGHT_DIAMONDS,
    CardNumber::SEVEN_DIAMONDS,
    CardNumber::SIX_DIAMONDS,
    CardNumber::FIVE_DIAMONDS,
    CardNumber::FOUR_DIAMONDS,
    CardNumber::TREY_DIAMONDS,
    CardNumber::DEUCE_DIAMONDS,
    CardNumber::ACE_CLUBS,
    CardNumber::KING_CLUBS,
    CardNumber::QUEEN_CLUBS,
    CardNumber::JACK_CLUBS,
    CardNumber::TEN_CLUBS,
    CardNumber::NINE_CLUBS,
    CardNumber::EIGHT_CLUBS,
    CardNumber::SEVEN_CLUBS,
    CardNumber::SIX_CLUBS,
    CardNumber::FIVE_CLUBS,
    CardNumber::FOUR_CLUBS,
    CardNumber::TREY_CLUBS,
    CardNumber::DEUCE_CLUBS,
]);

impl Deck {
    #[must_use]
    pub fn get(index: usize) -> PokerCard {
        if index < POKER_DECK.len() {
            POKER_DECK.0[index]
        } else {
            CardNumber::BLANK
        }
    }

    #[must_use]
    pub fn len() -> usize {
        POKER_DECK.0.len()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod poker_deck_tests {
    use super::*;

    #[test]
    fn pile() {
        let pile = Deck::pile();

        assert_eq!(pile, cardpack::Pile::french_deck())
    }

    #[test]
    fn poker_cards() {
        let cards = Deck::poker_cards();

        for (i, card) in Deck::iter().enumerate() {
            let got = cards.get(i);
            assert!(got.is_some());
            assert_eq!(got.unwrap(), card);
        }
        assert_eq!(cards.len(), Deck::len());
    }
}
