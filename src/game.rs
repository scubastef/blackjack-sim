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
        self.player.receive_card(self.shoe.deal().unwrap());
        self.dealer.receive_card(self.shoe.deal().unwrap(), false);
        self.player.receive_card(self.shoe.deal().unwrap());
        self.dealer.receive_card(self.shoe.deal().unwrap(), true);
    }

    pub fn play_player_turn(&mut self) {
        self.player.play_turn(self.dealer.get_up_card().unwrap(), &mut self.shoe)
    }

    pub fn play_dealer_turn(&mut self) {
        if !self.player.is_busted() {
            self.dealer.play_turn(&mut self.shoe)
        }
    }

    pub fn reset_state(&mut self) {
        if self.shoe.cut_card_seen() {
            todo!()
        }
    }
}