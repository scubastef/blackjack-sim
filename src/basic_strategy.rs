use crate::hand::Hand;
use crate::card::Card;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Hit,
    Stand,
    Double,
    Split,
}

const PAIR_TABLE: [[Action; 10]; 10] = {
    use Action::*;
    // Rows indexed by card rank 2..Ace
    [
        // Pair 2
        [Split, Split, Split, Split, Split, Split, Hit, Hit, Hit, Hit],
        // Pair 3
        [Split, Split, Split, Split, Split, Split, Hit, Hit, Hit, Hit],
        // Pair 4
        [Hit, Hit, Hit, Split, Split, Hit, Hit, Hit, Hit, Hit],
        // Pair 5 (treat as hard 10 -> double unless 10/A)
        [Double, Double, Double, Double, Double, Double, Double, Double, Hit, Hit],
        // Pair 6
        [Split, Split, Split, Split, Split, Hit, Hit, Hit, Hit, Hit],
        // Pair 7
        [Split, Split, Split, Split, Split, Split, Hit, Hit, Hit, Hit],
        // Pair 8
        [Split, Split, Split, Split, Split, Split, Split, Split, Split, Split],
        // Pair 9
        [Split, Split, Split, Split, Split, Stand, Split, Split, Stand, Stand],
        // Pair T
        [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],
        // Pair A
        [Split, Split, Split, Split, Split, Split, Split, Split, Split, Split],
    ]
};

const SOFT_TABLE: [[Action; 10]; 10] = {
    use Action::*;
    [
        // Soft 12 (A+A) REMOVE LATER!!!
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit],
        // Soft 13 (A+2)
        [Hit, Hit, Hit, Double, Double, Hit, Hit, Hit, Hit, Hit],
        // Soft 14 (A+3)
        [Hit, Hit, Hit, Double, Double, Hit, Hit, Hit, Hit, Hit],
        // Soft 15 (A+4)
        [Hit, Hit, Double, Double, Double, Hit, Hit, Hit, Hit, Hit],
        // Soft 16 (A+5)
        [Hit, Hit, Double, Double, Double, Hit, Hit, Hit, Hit, Hit],
        // Soft 17 (A+6)
        [Hit, Double, Double, Double, Double, Hit, Hit, Hit, Hit, Hit],
        // Soft 18 (A+7)
        [Stand, Stand, Double, Double, Double, Stand, Stand, Hit, Hit, Hit],
        // Soft 19
        [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],
        // Soft 20
        [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],
        // Soft 21
        [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],
    ]
};

pub const HARD_TABLE: [[Action; 10]; 18] = {
    use Action::*;
    [
        // 4
        [Hit; 10],
        // 5
        [Hit; 10],
        // 6
        [Hit; 10],
        // 7
        [Hit; 10],
        // 8
        [Hit; 10],
        // 9
        [Hit, Hit, Double, Double, Double, Double, Hit, Hit, Hit, Hit],
        // 10
        [Double, Double, Double, Double, Double, Double, Double, Double, Hit, Hit],
        // 11
        [Double, Double, Double, Double, Double, Double, Double, Double, Double, Hit],
        // 12
        [Hit, Hit, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit],
        // 13
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit],
        // 14
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit],
        // 15
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit],
        // 16
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit],
        // 17
        [Stand; 10],
        // 18
        [Stand; 10],
        // 19
        [Stand; 10],
        // 20
        [Stand; 10],
        // 21
        [Stand; 10],
    ]
};

pub fn get_action(hand: &Hand, dealer_up_card: &Card) -> Action {
    use Action::*;

    let player_total: u8 = hand.value();
    let is_hand_soft: bool = hand.is_soft();
    let is_hand_splitable: bool = false; // hand.is_splittable();
    let is_hand_doubleable: bool = hand.is_doubleable();

    let dealer_index = (dealer_up_card.get_value() - 2) as usize;

    // Pair strategy
    if is_hand_splitable {
        let rank: u8 = 4; // 2–11
        return PAIR_TABLE[(rank - 2) as usize][dealer_index];
    }

    // Soft total strategy
    if is_hand_soft {
        println!("Player tota ----l: {}", player_total);
        let idx = (player_total - 12) as usize; // 13–21
        return SOFT_TABLE[idx][dealer_index];
    }

    // Hard total strategy
    println!("Player tota ----l: {}", player_total);
    let idx = (player_total - 4) as usize; // 5–21
    let action = HARD_TABLE[idx][dealer_index];

    if action == Double && !is_hand_doubleable { Hit } else { action }
}

