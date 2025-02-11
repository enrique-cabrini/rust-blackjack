use std::io::{self, Write};

use card::{CardDeck, Card};

mod card;
mod player;

fn main() {
    let mut deck = CardDeck::new();
    deck.shuffle();

    println!("Welcome to Blackjack");
    println!("***********************************");

    print!("\nEnter an amount to buy in: ");
    io::stdout().flush().expect("Could not flush stdout");

    let mut buy_in_amount: String = String::new();
    io::stdin().read_line(&mut buy_in_amount)
        .expect("Could not read input");

    let buy_in_amount: u8 = match buy_in_amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid $ amount to play");
            return;
        }
    };

    let mut player = player::Player::buy_in(buy_in_amount);
    println!("\nYou bought in for ${buy_in_amount}");

    while player.amount != 0 {
        println!("\nYou have ${}.00 left", player.amount);

        let bet_amount = loop {
            print!("Amount to bet: ");
            io::stdout().flush().expect("Could not clear stdout");

            let mut bet_amount_str: String = String::new();

            if let Result::Err(_) = io::stdin().read_line(&mut bet_amount_str) {
                println!("Failed to read input");
                continue;
            }

            match bet_amount_str.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("You entered {}", bet_amount_str);
                    println!("Enter a valid $ amount");

                    continue;
                }
            }
        };

        if let Err(bet_error) = player.place_bet(bet_amount) {
            println!("{}", bet_error);
            continue;
        }

        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();

        gameloop(&mut deck, &mut player_hand, &mut dealer_hand);
    }
}

fn gameloop(
    deck: &mut card::CardDeck,
    player_hand: &mut Vec<Card>, 
    dealer_hand: &mut Vec<Card>
) {
    for _ in 0..2 {
        deck.deal_card(player_hand);

        deck.deal_card(dealer_hand);
    }

    println!("\nPlayer's hand:\n{}\n{}\n", player_hand[0],
        player_hand[1]);

    println!("Dealer's hand:\n{}\n{}\n", dealer_hand[0],
        dealer_hand[1]);

    loop {
        println!("Player's Hand:");
        for card in 0..player_hand.len() {
            println!("{}", player_hand[card]);
        }

        println!("\nMake a choice:\nH - HIT\tS - STAND");

        let mut player_choice_buf = String::new();
        io::stdin().read_line(&mut player_choice_buf)
            .expect("Could not read user input");

        let player_choice = player_choice_buf.chars().nth(0).unwrap();

        match player_choice {
            'H' => deck.deal_card(player_hand),
            'S' => break println!("STAND"),
            _ => continue,
        }

        if CardDeck::get_total_hand_sum(&player_hand) > 21 {
            println!("Player's Hand:");
            for card in 0..player_hand.len() {
                println!("{}", player_hand[card]);
            }

            return println!("\nBUST");
        }
    }

    println!("Dealer's Hand:");
    while CardDeck::get_total_hand_sum(&dealer_hand) <= 17 {
        for card in 0..dealer_hand.len() {
            println!("{}", dealer_hand[card]);
        }

        deck.deal_card(dealer_hand);
        println!();
    }

    println!("Dealer's Hand:");
    for card in 0..dealer_hand.len() {
        println!("{}", dealer_hand[card]);
    }

    if CardDeck::get_total_hand_sum(&player_hand) > 21 {
        return println!("\nDEALER BUSTED OUT! YOU WIN!!!!");
    }
}

/*fn handle_win_condition(player_sum: u8, dealer_sum: u8) {

}*/