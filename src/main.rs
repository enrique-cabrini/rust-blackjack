const CARDS: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) = 
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);

fn main() {
    println!("Welcome to Blackjack");

    let player_card = deal_player_card();
    println!("{player_card}");

    let dealer_card = deal_dealer_card();
    println!("{dealer_card}");
}

fn deal_player_card() -> u8 {
    CARDS.13
}

fn deal_dealer_card() -> u8 {
    CARDS.13
}