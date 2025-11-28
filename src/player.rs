use crate::card::Card;
use crate::shoe::Shoe;
use crate::hand::Hand;

pub struct Player {
    hand: Hand,
    chips: f64,
    current_bet: u32,
}

impl Player {
    pub fn new(starting_chips: u32) -> Player {
        Player {
            hand: Hand::new(),
            chips: starting_chips as f64,
            current_bet: 0,
        }
    }
    
    pub fn receive_card(&mut self, card: Card) {
        self.hand.add_card(card, false);
    }
    
    pub fn play_turn(&mut self, dealer_up_card: Card, deck: &mut Shoe) {
        while self.hand.value() < 17 {
            self.receive_card((*deck).deal().unwrap());
        }
    }

    pub fn place_bet(&mut self, amount: u32) {
        self.current_bet = amount;
    }

    pub fn win_bet(&mut self) {
        if self.hand.is_blackjack() {
            self.chips += 2.5 * self.current_bet as f64;
        } else {
            self.chips += (2 * self.current_bet) as f64;
        }

        self.current_bet = 0;
    }

    pub fn lose_bet(&mut self) {
        self.current_bet = 0;
    }

    pub fn push(&mut self) {
        self.chips += self.current_bet as f64;
        self.current_bet = 0;
    }
    
    pub fn is_busted(&self) -> bool {
        self.hand.is_busted()
    }

}