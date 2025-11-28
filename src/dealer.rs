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

    pub fn play_turn(&mut self, deck: &mut Shoe) {
        while self.should_hit() {
            self.hand.add_card(deck.deal().unwrap(), false)
        }
    }

    pub fn get_up_card(&self) -> Option<Card> {
        self.hand.get_up_card()
    }
}