use std::io::{self, Write};

use card::Card;

mod card;
mod player;

fn main() {
    let mut deck = card::CardDeck::new();
    deck.shuffle();

    let mut player_hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

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

        if !player.place_bet(bet_amount) {
            continue;
        }

        println!("\nYou have ${}.00 left", player.amount);

        deck.deal_card(&mut player_hand);
        for card in &player_hand {
            println!("{}", card);
        }

        deck.deal_card(&mut dealer_hand);
        for card in &dealer_hand {
            println!("{}", card);
        }
    }
}
