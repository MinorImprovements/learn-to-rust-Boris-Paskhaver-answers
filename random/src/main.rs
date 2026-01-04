use rand::{rng, seq::SliceRandom, Rng};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Suit {
    const SUITS: [Self; 4] = [Self::Clubs, Self::Spades, Self::Hearts, Self::Diamonds];
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl Rank {
    const RANKS_WITHOUT_JOKER: [Self; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

impl Card {
    fn new(rank: Rank, suit: Option<Suit>) -> Self {
        Self { rank, suit }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut deck: Vec<Card> = vec![];
        for suit in Suit::SUITS {
            for rank in Rank::RANKS_WITHOUT_JOKER {
                deck.push(Card::new(rank, Some(suit)));
            }
        }
        Self { cards: deck }
    }

    fn shuffle(&mut self) {
        let mut my_rng = rng();
        self.cards.shuffle(&mut my_rng);
    }

    fn insert_jokers(&mut self) {
        let mut my_rng = rng();
        for _ in 0..2 {
            let point_of_insertion = my_rng.random_range(0..self.cards.len());
            self.cards
                .insert(point_of_insertion, Card::new(Rank::Joker, None));
        }
    }

    fn delete_random_card(&mut self) {
        let mut my_rng = rng();
        let delete_card_at_this_position = my_rng.random_range(0..self.cards.len());
        let will_delete = my_rng.random_bool(0.65);
        if will_delete {
            self.cards.remove(delete_card_at_this_position);
        }
    }
}

fn main() {
    let mut playing_deck = Deck::new();
    //playing_deck.insert_jokers();

    for _ in 1..10 {
        playing_deck.delete_random_card();
    }

    println!("{:#?}", playing_deck.cards.len());

    //playing_deck.shuffle();
    //println!("{playing_deck:#?}");}
}
