use log::debug;
use crate::dealer::Dealer;
use crate::shoe::Shoe;
use crate::player::Player;

pub struct Game {
    pub shoe: Shoe,
    pub player: Player,
    pub dealer: Dealer,
}

impl Game {
    pub fn new(num_decks: usize, bankroll: u32) -> Game {
        Game {
            shoe: Shoe::new(num_decks),
            player: Player::new(bankroll),
            dealer: Dealer::new(),
        }
    }

    pub fn start_round(&mut self) {
        self.player.place_bet(10);
    }

    pub fn initial_deal(&mut self) {
        self.player.receive_card(self.shoe.deal());
        self.dealer.receive_card(self.shoe.deal(), false);
        self.player.receive_card(self.shoe.deal());
        self.dealer.receive_card(self.shoe.deal(), true);
    }

    pub fn play_player_turn(&mut self) {
        debug!("Player playing turn...");
        self.player.play_turn(self.dealer.get_up_card().unwrap(), &mut self.shoe);
        debug!("Player finished playing turn",)
    }

    pub fn play_dealer_turn(&mut self) {
        debug!("Dealer playing turn...");
        if !self.player.is_busted() {
            self.dealer.play_turn(&mut self.shoe)
        }
    }

    pub fn settle_bets(&mut self) {
        let player_total: u8 = self.player.hand_value();
        let dealer_total: u8 = self.dealer.hand_value();

        let player_busted: bool = self.player.is_busted();
        let dealer_busted: bool = self.dealer.is_busted();

        let win_scenario1: bool = dealer_busted && !player_busted;
        let win_scenario2: bool = player_total > dealer_total && !player_busted;


        if win_scenario1 || win_scenario2 {
            self.player.win_bet();
        } else if self.player.hand_value() == self.dealer.hand_value() {
            self.player.push();
        } else {
            self.player.lose_bet();
        }
    }

    pub fn reset_state(&mut self) {
        self.player.clear_hand();
        self.dealer.clear_hand();

        if self.shoe.cut_card_seen() {
            debug!("CUT CARD SEEN! Shuffling deck...");
            self.shoe.shuffle()
        }
    }
}