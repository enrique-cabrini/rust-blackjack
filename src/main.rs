struct Player {
    amount: u8
}

impl Player {
    fn buy_in(buy_in_amount: u8) -> Player {
        Self {
            amount: buy_in_amount
        }
    }

    fn place_bet(&mut self, bet_amount: u8) {
        self.amount -= bet_amount;
    }
}

const CARDS: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) = 
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);

fn main() {
    println!("Welcome to Blackjack\n
        ************************************");
    
    let mut player = Player::buy_in(10);
    println!("\nYou bought in for $10.00");

    while player.amount != 0 {
        player.place_bet(1);
        println!("\nYou have ${}.00 left", player.amount);

        let player_cards = deal_first_cards();
        println!("Player Cards:\n{} {}", 
            player_cards[0], player_cards[1]);

        let dealer_cards = deal_first_cards();
        println!("Dealer Cards:\n{} {}",
            dealer_cards[0], dealer_cards[1]);
    }
}

fn deal_first_cards() -> Vec<u8> {
    vec![CARDS.1, CARDS.13]
}