use log::debug;
use crate::hand::Hand;
use crate::card::Card;
use crate::shoe::Shoe;

pub struct Dealer {
    hand: Hand
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer { hand: Hand::new() }
    }

    pub fn receive_card(&mut self, card: Card, is_up_card: bool) {
        self.hand.add_card(card, is_up_card);
    }

    pub fn should_hit(&self) -> bool {
        self.hand.value() < 17
    }

    pub fn play_turn(&mut self, shoe: &mut Shoe) {
        debug!("Dealer total: {}", self.hand.value());
        while self.should_hit() {
            self.hand.add_card(shoe.deal(), false)
        }
        debug!("Dealer total: {}", self.hand.value());
    }

    pub fn get_up_card(&self) -> Option<Card> {
        self.hand.get_up_card()
    }

    pub fn hand_value(&self) -> u8 {
        self.hand.value()
    }
    
    pub fn is_busted(&self) -> bool {
        self.hand.is_busted()
    }

    pub fn clear_hand(&mut self) {
        self.hand = Hand::new();
    }
}