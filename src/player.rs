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

    pub fn place_bet(&mut self, bet_amount: u8) -> bool {
        match self.amount >= bet_amount {
            true => {
                self.amount -= bet_amount;
                true
            }
            false => false,
        }
    }
}