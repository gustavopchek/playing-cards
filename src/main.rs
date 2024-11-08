use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // return type annotation
    // same as fn new() -> Deck
    fn new() -> Self {
        // List of 'suits' - 'diamonds', 'clubs'
        // List of 'values' - 'ace', 'two', 'three'

        // Double nested for loops

        let suits = ["Hearts", "Spades", "Clubs", "Diamonds"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        // make it mutable
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // same as { cards: cards }
        // without 
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();

        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}


fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    // needs error handling
    let cards = deck.deal(3);

    println!("Here's your hand: {:#?}", cards);
    println!("Here is the deck: {:#?}", deck);
}


