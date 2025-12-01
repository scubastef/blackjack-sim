use crate::card::Card;
use crate::shoe::Shoe;
use crate::hand::Hand;

pub struct Player {
    hand: Hand,
    bankroll: f64,
    current_bet: u32,
}

impl Player {
    pub fn new(bankroll: u32) -> Player {
        Player {
            hand: Hand::new(),
            bankroll: bankroll as f64,
            current_bet: 0,
        }
    }
    
    pub fn receive_card(&mut self, card: Card) {
        self.hand.add_card(card, false);
    }
    
    pub fn play_turn(&mut self, dealer_up_card: Card, deck: &mut Shoe) {
        while self.hand.value() < 17 {
            println!("Dealer upcard value {}", dealer_up_card.get_value());
            println!("Player hand total {}", self.hand.value());
            println!("Taking a card");
            
            self.receive_card((*deck).deal());
        }
        println!("Player hand total {}", self.hand.value());
    }

    pub fn place_bet(&mut self, amount: u32) {
        println!("Player bankroll: ${:.2}", self.bankroll);
        println!("Placing bet of ${}", amount);
        self.current_bet = amount;
    }

    pub fn win_bet(&mut self) {
        if self.hand.is_blackjack() {
            self.bankroll += 2.5 * self.current_bet as f64;
        } else {
            self.bankroll += (2 * self.current_bet) as f64;
        }
        println!("Player wins! Bankroll: ${:.2}", self.bankroll);
        self.current_bet = 0;
    }

    pub fn lose_bet(&mut self) {
        self.current_bet = 0;
        println!("Player loses! Bankroll: ${:.2}", self.bankroll);
    }

    pub fn push(&mut self) {
        self.bankroll += self.current_bet as f64;
        self.current_bet = 0;
        println!("Player pushes. Bankroll: ${:.2}", self.bankroll);
    }
    
    pub fn is_busted(&self) -> bool {
        self.hand.is_busted()
    }
    
    pub fn hand_value(&self) -> u8 {
        self.hand.value()
    }
    
    pub fn clear_hand(&mut self) {
        self.hand = Hand::new();
    }

}