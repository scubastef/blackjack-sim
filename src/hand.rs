use crate::card::{Card, Rank};

pub struct Hand {
    cards: Vec<Card>,
    up_card: Option<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new(), up_card: None }
    }

    pub fn add_card(&mut self, card: Card, is_up_card: bool) {
        self.cards.push(card);
        
        if is_up_card {
            self.up_card = Some(card);
        }
    }

    pub fn value(&self) -> u8 {
        let mut num_soft_aces = self.cards.iter()
            .filter(|card| card.rank == Rank::Ace).count();

        let mut value: u8 = 0;
        for card in self.cards.iter() {
            value += card.get_value();
        }

        while value > 21 && num_soft_aces > 0 {
            value -= 10;
            num_soft_aces -= 1;
        }

        value
    }
    
    pub fn is_soft(&self) -> bool {
        let has_ace: bool = self.cards.iter().any(|card| card.rank == Rank::Ace);

        if !has_ace { return false; }

        // Calculate value with all aces as 1
        let hard_value: u8 = self.cards.iter()
            .map(|card| {
                if card.rank == Rank::Ace {
                    1
                } else {
                    card.get_value()
                }
            })
            .sum();

        // If we can add 10 (treating one ace as 11 instead of 1) without busting,
        // then the hand is soft
        hard_value + 10 <= 21
    }
    
    pub fn is_splittable(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].get_value() == self.cards[1].get_value()
    }
    
    pub fn is_doubleable(&self) -> bool { self.cards.len() == 2 }

    pub fn is_busted(&self) -> bool {
        self.value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }
    
    pub fn get_up_card(&self) -> Option<Card> {
        self.up_card
    }
}