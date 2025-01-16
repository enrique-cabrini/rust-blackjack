pub mod card {}

#[derive(Debug, Clone, Copy)]
pub enum CardSuit {
    Club,
    Heart,
    Spade,
    Diamond,
}

#[derive(Debug, Clone, Copy)]
pub enum CardRank {
    Value(u8),
    King,
    Queen,
    Jack,
    Ace,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub rank: CardRank,
    pub suit: CardSuit,
}

#[derive(Clone)]
pub struct CardDeck {
    pub cards: Vec<Card>
}

impl CardDeck {
    pub fn new() -> Self {
        CardDeck {
            cards: vec![
                Card { rank: CardRank::Value(2), suit: CardSuit::Club },
                Card { rank: CardRank::Value(3), suit: CardSuit::Club },
                Card { rank: CardRank::Value(4), suit: CardSuit::Club },
                Card { rank: CardRank::Value(5), suit: CardSuit::Club },
                Card { rank: CardRank::Value(6), suit: CardSuit::Club },
                Card { rank: CardRank::Value(7), suit: CardSuit::Club },
                Card { rank: CardRank::Value(8), suit: CardSuit::Club },
                Card { rank: CardRank::Value(9), suit: CardSuit::Club },
                Card { rank: CardRank::Value(10), suit: CardSuit::Club },
                Card { rank: CardRank::Jack, suit: CardSuit::Club },
                Card { rank: CardRank::Queen, suit: CardSuit::Club },
                Card { rank: CardRank::King, suit: CardSuit::Club },
                Card { rank: CardRank::Ace, suit: CardSuit::Club },
                Card { rank: CardRank::Value(2), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(3), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(4), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(5), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(6), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(7), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(8), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(9), suit: CardSuit::Heart },
                Card { rank: CardRank::Value(10), suit: CardSuit::Heart },
                Card { rank: CardRank::Jack, suit: CardSuit::Heart },
                Card { rank: CardRank::Queen, suit: CardSuit::Heart },
                Card { rank: CardRank::King, suit: CardSuit::Heart },
                Card { rank: CardRank::Ace, suit: CardSuit::Heart },
                Card { rank: CardRank::Value(2), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(3), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(4), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(5), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(6), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(7), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(8), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(9), suit: CardSuit::Spade },
                Card { rank: CardRank::Value(10), suit: CardSuit::Spade },
                Card { rank: CardRank::Jack, suit: CardSuit::Spade },
                Card { rank: CardRank::Queen, suit: CardSuit::Spade },
                Card { rank: CardRank::King, suit: CardSuit::Spade },
                Card { rank: CardRank::Ace, suit: CardSuit::Spade },
                Card { rank: CardRank::Value(2), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(3), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(4), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(5), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(6), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(7), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(8), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(9), suit: CardSuit::Diamond },
                Card { rank: CardRank::Value(10), suit: CardSuit::Diamond },
                Card { rank: CardRank::Jack, suit: CardSuit::Diamond },
                Card { rank: CardRank::Queen, suit: CardSuit::Diamond },
                Card { rank: CardRank::King, suit: CardSuit::Diamond },
                Card { rank: CardRank::Ace, suit: CardSuit::Diamond },
            ]
        }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
