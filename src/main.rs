use crate::game::Game;

mod card;
mod hand;
mod player;
mod game;
mod dealer;
mod shoe;
mod basic_strategy;


fn main() {
    let num_decks: usize = 6;
    let bankroll: u32 = 1000;
    let num_hands_to_play: u32 = 100;

    let mut game = Game::new(num_decks, bankroll);

    for i in 0..num_hands_to_play {
        println!("Starting hand number {}...", i + 1);

        game.start_round();
        game.initial_deal();
        game.play_player_turn();
        game.play_dealer_turn();
        game.settle_bets();
        game.reset_state();

        println!("Finished hand number: {} \n\n", i + 1);

    }
}
