use solitaire_core::card::{Rank, Suit};

pub fn get_card_bytes(suit: Suit, rank: Rank) -> &'static [u8] {
    match (suit, rank) {
        // Spade
        (Suit::Spade, Rank::Ace) => include_bytes!("../assets/cards/spade/card_1_spade.png"),
        (Suit::Spade, Rank::Two) => include_bytes!("../assets/cards/spade/card_2_spade.png"),
        (Suit::Spade, Rank::Three) => include_bytes!("../assets/cards/spade/card_3_spade.png"),
        (Suit::Spade, Rank::Four) => include_bytes!("../assets/cards/spade/card_4_spade.png"),
        (Suit::Spade, Rank::Five) => include_bytes!("../assets/cards/spade/card_5_spade.png"),
        (Suit::Spade, Rank::Six) => include_bytes!("../assets/cards/spade/card_6_spade.png"),
        (Suit::Spade, Rank::Seven) => include_bytes!("../assets/cards/spade/card_7_spade.png"),
        (Suit::Spade, Rank::Eight) => include_bytes!("../assets/cards/spade/card_8_spade.png"),
        (Suit::Spade, Rank::Nine) => include_bytes!("../assets/cards/spade/card_9_spade.png"),
        (Suit::Spade, Rank::Ten) => include_bytes!("../assets/cards/spade/card_10_spade.png"),
        (Suit::Spade, Rank::Jack) => include_bytes!("../assets/cards/spade/card_11_spade.png"),
        (Suit::Spade, Rank::Queen) => include_bytes!("../assets/cards/spade/card_12_spade.png"),
        (Suit::Spade, Rank::King) => include_bytes!("../assets/cards/spade/card_13_spade.png"),

        // Heart
        (Suit::Heart, Rank::Ace) => include_bytes!("../assets/cards/heart/card_1_heart.png"),
        (Suit::Heart, Rank::Two) => include_bytes!("../assets/cards/heart/card_2_heart.png"),
        (Suit::Heart, Rank::Three) => include_bytes!("../assets/cards/heart/card_3_heart.png"),
        (Suit::Heart, Rank::Four) => include_bytes!("../assets/cards/heart/card_4_heart.png"),
        (Suit::Heart, Rank::Five) => include_bytes!("../assets/cards/heart/card_5_heart.png"),
        (Suit::Heart, Rank::Six) => include_bytes!("../assets/cards/heart/card_6_heart.png"),
        (Suit::Heart, Rank::Seven) => include_bytes!("../assets/cards/heart/card_7_heart.png"),
        (Suit::Heart, Rank::Eight) => include_bytes!("../assets/cards/heart/card_8_heart.png"),
        (Suit::Heart, Rank::Nine) => include_bytes!("../assets/cards/heart/card_9_heart.png"),
        (Suit::Heart, Rank::Ten) => include_bytes!("../assets/cards/heart/card_10_heart.png"),
        (Suit::Heart, Rank::Jack) => include_bytes!("../assets/cards/heart/card_11_heart.png"),
        (Suit::Heart, Rank::Queen) => include_bytes!("../assets/cards/heart/card_12_heart.png"),
        (Suit::Heart, Rank::King) => include_bytes!("../assets/cards/heart/card_13_heart.png"),

        // Diamond
        (Suit::Diamond, Rank::Ace) => include_bytes!("../assets/cards/diamond/card_1_diamond.png"),
        (Suit::Diamond, Rank::Two) => include_bytes!("../assets/cards/diamond/card_2_diamond.png"),
        (Suit::Diamond, Rank::Three) => {
            include_bytes!("../assets/cards/diamond/card_3_diamond.png")
        }
        (Suit::Diamond, Rank::Four) => include_bytes!("../assets/cards/diamond/card_4_diamond.png"),
        (Suit::Diamond, Rank::Five) => include_bytes!("../assets/cards/diamond/card_5_diamond.png"),
        (Suit::Diamond, Rank::Six) => include_bytes!("../assets/cards/diamond/card_6_diamond.png"),
        (Suit::Diamond, Rank::Seven) => {
            include_bytes!("../assets/cards/diamond/card_7_diamond.png")
        }
        (Suit::Diamond, Rank::Eight) => {
            include_bytes!("../assets/cards/diamond/card_8_diamond.png")
        }
        (Suit::Diamond, Rank::Nine) => include_bytes!("../assets/cards/diamond/card_9_diamond.png"),
        (Suit::Diamond, Rank::Ten) => include_bytes!("../assets/cards/diamond/card_10_diamond.png"),
        (Suit::Diamond, Rank::Jack) => {
            include_bytes!("../assets/cards/diamond/card_11_diamond.png")
        }
        (Suit::Diamond, Rank::Queen) => {
            include_bytes!("../assets/cards/diamond/card_12_diamond.png")
        }
        (Suit::Diamond, Rank::King) => {
            include_bytes!("../assets/cards/diamond/card_13_diamond.png")
        }

        // Club
        (Suit::Club, Rank::Ace) => include_bytes!("../assets/cards/club/card_1_club.png"),
        (Suit::Club, Rank::Two) => include_bytes!("../assets/cards/club/card_2_club.png"),
        (Suit::Club, Rank::Three) => include_bytes!("../assets/cards/club/card_3_club.png"),
        (Suit::Club, Rank::Four) => include_bytes!("../assets/cards/club/card_4_club.png"),
        (Suit::Club, Rank::Five) => include_bytes!("../assets/cards/club/card_5_club.png"),
        (Suit::Club, Rank::Six) => include_bytes!("../assets/cards/club/card_6_club.png"),
        (Suit::Club, Rank::Seven) => include_bytes!("../assets/cards/club/card_7_club.png"),
        (Suit::Club, Rank::Eight) => include_bytes!("../assets/cards/club/card_8_club.png"),
        (Suit::Club, Rank::Nine) => include_bytes!("../assets/cards/club/card_9_club.png"),
        (Suit::Club, Rank::Ten) => include_bytes!("../assets/cards/club/card_10_club.png"),
        (Suit::Club, Rank::Jack) => include_bytes!("../assets/cards/club/card_11_club.png"),
        (Suit::Club, Rank::Queen) => include_bytes!("../assets/cards/club/card_12_club.png"),
        (Suit::Club, Rank::King) => include_bytes!("../assets/cards/club/card_13_club.png"),
    }
}

pub fn get_card_template_bytes(suit: Suit) -> &'static [u8] {
    match suit {
        Suit::Spade => include_bytes!("../assets/cards/spade_template.png"),
        Suit::Heart => include_bytes!("../assets/cards/heart_template.png"),
        Suit::Diamond => include_bytes!("../assets/cards/diamond_template.png"),
        Suit::Club => include_bytes!("../assets/cards/club_template.png"),
    }
}

pub fn get_card_back_bytes() -> &'static [u8] {
    include_bytes!("../assets/cards/back.png")
}
