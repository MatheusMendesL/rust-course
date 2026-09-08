#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

struct  Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    /*let first_card = CardSuit::Diamonds;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;*/

    let card_suits = [CardSuit::Spades, CardSuit::Clubs];
}
