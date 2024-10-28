#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
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
    let deck = Deck { cards };
    // print each element on a new line
    println!("Here is the deck: {:#?}", deck);
}


