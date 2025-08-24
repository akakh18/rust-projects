use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,

}

impl Deck {
    fn new() -> Self {
        // list of suits
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        // list of values
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];

        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck{cards }
    }

    fn shuffle(&mut self) {
        let mut rand_num_gen = rng();
        self.cards.shuffle(&mut rand_num_gen)
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // TODO Add error handling
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let cards = deck.deal(3);
    println!("Here is your deck: {:?}", deck);
    println!("Here is your cards: {:?}", cards);
}
