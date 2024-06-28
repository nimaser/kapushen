use rand::seq::SliceRandom;
use std::fmt;

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

#[derive(Copy, Clone)]
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
        match self.value {
            CardValue::King if matches!(self.suit, CardSuit::Hearts | CardSuit::Diamonds) => 0,
            _ => self.value as isize
        }
    }

    fn get_action(&self) -> Option<CardAction> {
        match self.value {
            CardValue::Seven | CardValue::Eight => Some(CardAction::PeekSelf),
            CardValue::Nine  | CardValue::Ten   => Some(CardAction::PeekElse),
            CardValue::Jack  | CardValue::Queen => Some(CardAction::SwapPair),
            CardValue::King => Some(CardAction::HatTrick),
            _ => None,
        }
    }
}

struct CardStack(Vec<Card>);

impl CardStack {
    fn new_deck(&mut self) -> () {
        let CardStack(deck_vec) = self;
        
    }
    
    fn shuffle_deck(&mut self) -> () {
        let CardStack(deck_vec) = self;
        deck_vec.shuffle(&mut rand::thread_rng());
    }
}