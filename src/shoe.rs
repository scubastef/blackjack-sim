use rand::rng;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::card::Rank;
use crate::card::Suit;

const CARDS_PER_DECK: usize = 52;
const ALL_SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
const ALL_RANKS: [Rank; 13] = [
    Rank::Ace,
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
];

pub struct Shoe {
    cards: Vec<Card>,
    cut_card_idx: u32,
}

impl Shoe {
    pub fn new(num_decks: usize) -> Shoe {
        let mut cards: Vec<Card> = Vec::with_capacity(num_decks * CARDS_PER_DECK);

        for _ in 0..num_decks {
            for rank in ALL_RANKS.iter() {
                for suit in ALL_SUITS.iter() {
                    cards.push(Card { rank: *rank, suit: *suit });
                }
            }
        }

        let mut shoe = Shoe { cards, cut_card_idx: 270 };
        shoe.shuffle();
        shoe
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng)
    }

    pub fn cut_card_seen(&self) -> bool {
        self.cards.len() < 30
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

}