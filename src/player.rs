pub mod player {}

pub struct Player {
    pub amount: u8,
}

impl Player {
    pub fn buy_in(buy_in_amount: u8) -> Player {
        Self {
            amount: buy_in_amount,
        }
    }

    pub fn place_bet(&mut self, bet_amount: u8) -> Result<(), String> {
        if bet_amount > self.amount {
            Err(String::from("Cannot bet more than you currently have."))
        } else {
            self.amount -= bet_amount;
            Ok(())
        }
    }
}