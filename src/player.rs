use crate::card::Card;
use crate::shoe::Shoe;
use crate::hand::Hand;
use crate::basic_strategy::Action;
use crate::basic_strategy::get_action;
use log::debug;

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

        loop {
            let action: Action = get_action(&self.hand, &dealer_up_card);

            match action {
                Action::Stand => break,
                Action::Hit => {
                    self.receive_card(deck.deal());
                    if self.is_busted() { break; }
                }
                Action::Double => {
                    self.current_bet *= 2;
                    self.bankroll -= self.current_bet as f64;
                    self.receive_card(deck.deal());
                    break;
                }
                Action::Split => {}
            }
        }
    }

    pub fn place_bet(&mut self, amount: u32) {
        debug!("Player bankroll: ${:.2}", self.bankroll);
        debug!("Placing bet of ${}", amount);
        self.current_bet = amount;
        self.bankroll -= amount as f64;
        debug!("Player bankroll: ${:.2}", self.bankroll);
    }

    pub fn win_bet(&mut self) {
        if self.hand.is_blackjack() {
            self.bankroll += 2.5 * self.current_bet as f64;
        } else {
            self.bankroll += (2 * self.current_bet) as f64;
        }
        debug!("Player wins! Bankroll: ${:.2}", self.bankroll);
        self.current_bet = 0;
    }

    pub fn lose_bet(&mut self) {
        self.current_bet = 0;
        debug!("Player loses! Bankroll: ${:.2}", self.bankroll);
    }

    pub fn push(&mut self) {
        self.bankroll += self.current_bet as f64;
        self.current_bet = 0;
        debug!("Player pushes. Bankroll: ${:.2}", self.bankroll);
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