use rand::{thread_rng, seq::SliceRandom};

enum CardAction {
    PeekSelf,
    PeekElse,
    SwapPair,
    HatTrick,
}

enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

enum CardValue {
    Ace     = 1,
    Two     = 2,
    Three   = 3,
    Four    = 4,
    Five    = 5,
    Six     = 6,
    Seven   = 7,
    Eight   = 8,
    Nine    = 9,
    Ten     = 10,
    Jack    = 11,
    Queen   = 12,
    King    = 13,
    Joker   = -1,
}

struct Card {
    suit    : CardSuit,
    value   : CardValue,
}

impl Card {
    fn get_value(&self) -> isize {
        match self.suit {
            CardSuit::Hearts | CardSuit::Diamonds => {
                if matches!(CardValue:King, self.value) { 0 }
                else CardValue::King as isize
            }
            _ => self.value as isize
        }
    }

    fn get_action(&self) -> Option<CardAction> {
        match self.value {
            CardValue::Seven | CardValue::Eight => Some(CardActions::PeekSame),
            CardValue::Nine  | CardValue::Ten   => Some(CardActions::PeekElse),
            CardValue::Jack  | CardValue::Queen => Some(CardActions::SwapPair),
            CardValue::King => Some(CardActions::HatTrick),
            _ => None,
        }
    }
}

struct CardStack { cards : Vec<Card> }

impl CardStack {
    fn new_deck() -> Self {
        CardStack {
            vec![]
        }
    }
