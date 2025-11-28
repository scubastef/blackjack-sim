use crate::game::Game;

mod deck;
mod card;
mod hand;
mod player;
mod game;
mod dealer;
mod shoe;


fn main() {
    let num_decks: usize = 6;
    let bankroll: u32 = 1000;
    let num_hands_to_play: u32 = 1000;

    let mut game = Game::new(num_decks, bankroll);

    for _ in 0..num_hands_to_play {

        game.start_round();
        game.initial_deal();
        game.play_player_turn();
        game.play_dealer_turn();
        game.reset_state();

        println!("made it");

    }
}
