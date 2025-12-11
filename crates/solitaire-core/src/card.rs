use enum_iterator::{Sequence, all};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Getters, Setters)]
pub struct Card {
    #[getset(get = "pub")]
    rank: Rank,
    #[getset(get = "pub")]
    suit: Suit,
    #[getset(get = "pub", set = "pub")]
    face: Face,
}
impl Card {
    /// Creates new Card
    pub fn new(rank: Rank, suit: Suit, face: Face) -> Self {
        Self { rank, suit, face }
    }
    pub fn color(&self) -> Color {
        self.suit().color()
    }
    pub fn flip(&mut self) {
        self.face.flip();
    }
}

/// Gets all cards in order (suit then rank)
pub fn all_cards() -> Vec<Card> {
    let mut cards = vec![];
    for suit in all::<Suit>() {
        for rank in all::<Rank>() {
            cards.push(Card::new(rank, suit, Face::Up));
        }
    }
    cards
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Sequence, Hash,
)]
pub enum Rank {
    Ace = 1,
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
}

impl Rank {
    pub fn higher(&self) -> Option<Self> {
        match self {
            Self::Ace => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => Some(Self::Five),
            Self::Five => Some(Self::Six),
            Self::Six => Some(Self::Seven),
            Self::Seven => Some(Self::Eight),
            Self::Eight => Some(Self::Nine),
            Self::Nine => Some(Self::Ten),
            Self::Ten => Some(Self::Jack),
            Self::Jack => Some(Self::Queen),
            Self::Queen => Some(Self::King),
            Self::King => None,
        }
    }
    pub fn lower(&self) -> Option<Self> {
        match self {
            Self::Ace => None,
            Self::Two => Some(Self::Ace),
            Self::Three => Some(Self::Two),
            Self::Four => Some(Self::Three),
            Self::Five => Some(Self::Four),
            Self::Six => Some(Self::Five),
            Self::Seven => Some(Self::Six),
            Self::Eight => Some(Self::Seven),
            Self::Nine => Some(Self::Eight),
            Self::Ten => Some(Self::Nine),
            Self::Jack => Some(Self::Ten),
            Self::Queen => Some(Self::Jack),
            Self::King => Some(Self::Queen),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Sequence, Hash)]
pub enum Suit {
    Heart,
    Club,
    Diamond,
    Spade,
}
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Club => "club",
            Self::Diamond => "diamond",
            Self::Heart => "heart",
            Self::Spade => "spade",
        };
        write!(f, "{string}")
    }
}
impl Suit {
    pub fn color(&self) -> Color {
        match self {
            Self::Club | Self::Spade => Color::Black,
            Self::Heart | Self::Diamond => Color::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Color {
    Black,
    Red,
}
impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::Red,
            Self::Red => Self::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Face {
    Up,
    Down,
}
impl Face {
    pub fn flip(&mut self) {
        *self = match *self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        };
    }
}
