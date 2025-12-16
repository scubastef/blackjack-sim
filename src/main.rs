use crate::game::Game;
use log::{info, warn, error, debug};

mod card;
mod hand;
mod player;
mod game;
mod dealer;
mod shoe;
mod basic_strategy;


fn main() {
    env_logger::init();

    let num_decks: usize = 6;
    let bankroll: u32 = 1000;
    let num_hands_to_play: u32 = 10;
    info!(
        "Playing {} hands with {} decks and a bankroll of ${:.2}",
        num_hands_to_play, num_decks, bankroll as f64
    );

    let mut game = Game::new(num_decks, bankroll);

    for i in 0..num_hands_to_play {
        debug!("Starting hand number {}...", i + 1);

        game.start_round();
        game.initial_deal();
        game.play_player_turn();
        game.play_dealer_turn();
        game.settle_bets();
        game.reset_state();

        debug!("Finished hand number: {} \n\n", i + 1);

    }
}
