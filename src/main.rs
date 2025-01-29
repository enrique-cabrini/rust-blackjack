use std::io::{self, Write};

mod card;
mod player;

const CARDS: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

fn main() {
    let mut deck = card::CardDeck::new();
    deck.shuffle();

    for i in deck.cards {
        println!("{:#?} {:#?}", i.rank, i.suit);
    }

    println!("Welcome to Blackjack");
    println!("***********************************");

    print!("\nEnter an amount to buy in: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(_) => panic!("Could not flush stdout\n"),
    }

    let mut buy_in_amount: String = String::new();
    match io::stdin().read_line(&mut buy_in_amount) {
        Ok(_) => (),
        Err(_) => panic!("Could not read input"),
    }

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
            if let Result::Err(_) = io::stdout().flush() {
                println!("Could not clear stdout");
                return;
            }

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

        let player_cards = deal_first_cards();
        println!("Player Cards:\n{} {}", player_cards[0], player_cards[1]);

        let dealer_cards = deal_first_cards();
        println!("Dealer Cards:\n{} {}", dealer_cards[0], dealer_cards[1]);
    }
}

fn deal_first_cards() -> Vec<u8> {
    vec![CARDS.1, CARDS.10]
}
