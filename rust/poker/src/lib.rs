use std::collections::{BTreeMap, BTreeSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands = hands.to_vec();
    let mut hands_map = BTreeMap::new();
    let mut hands_set = BTreeSet::new();
    let mut max_rank = 0;
    let mut max_rank_hands = Vec::new();
    for hand in hands.iter() {
        let rank = rank_hand(hand);
        hands_map.insert(hand, rank);
        hands_set.insert(rank);
        if rank > max_rank {
            max_rank = rank;
        }
    }
    for (hand, rank) in hands_map.iter() {
        if *rank == max_rank {
            max_rank_hands.push(**hand);
        }
    }
    max_rank_hands
}

fn rank_hand(hand: &str) -> u32 {
    let mut cards = hand.split_whitespace();
    let mut card_values = Vec::new();
    let mut card_suits = Vec::new();
    while let Some(card) = cards.next() {
        let mut card_chars = card.chars();
        let card_value = card_chars.next().unwrap();
        let card_suit = card_chars.next().unwrap();
        card_values.push(card_value);
        card_suits.push(card_suit);
    }
    card_values.sort();
    card_suits.sort();
    let mut rank = 0;
    if is_straight(&card_values) && is_flush(&card_suits) {
        rank = 8;
    } else if is_four_of_a_kind(&card_values) {
        rank = 7;
    } else if is_full_house(&card_values) {
        rank = 6;
    } else if is_flush(&card_suits) {
        rank = 5;
    } else if is_straight(&card_values) {
        rank = 4;
    } else if is_three_of_a_kind(&card_values) {
        rank = 3;
    } else if is_two_pair(&card_values) {
        rank = 2;
    } else if is_one_pair(&card_values) {
        rank = 1;
    }
    rank
}

fn is_straight(card_values: &[char]) -> bool {
    let mut is_straight = true;
    let mut prev_card_value = card_values[0];
    for card_value in card_values.iter().skip(1) {
        if prev_card_value as u8 + 1 != *card_value as u8 {
            is_straight = false;
            break;
        }
        prev_card_value = *card_value;
    }
    is_straight
}

fn is_flush(card_suits: &[char]) -> bool {
    let mut is_flush = true;
    let mut prev_card_suit = card_suits[0];
    for card_suit in card_suits.iter().skip(1) {
        if prev_card_suit != *card_suit {
            is_flush = false;
            break;
        }
        prev_card_suit = *card_suit;
    }
    is_flush
}

fn is_four_of_a_kind(card_values: &[char]) -> bool {
    let mut is_four_of_a_kind = false;
    let mut prev_card_value = card_values[0];
    let mut count = 1;
    for card_value in card_values.iter().skip(1) {
        if prev_card_value == *card_value {
            count += 1;
            if count == 4 {
                is_four_of_a_kind = true;
                break;
            }
        } else {
            count = 1;
        }
        prev_card_value = *card_value;
    }
    is_four_of_a_kind
}

fn is_full_house(card_values: &[char]) -> bool {
    let mut is_full_house = false;
    let mut prev_card_value = card_values[0];
    let mut count = 1;
    for card_value in card_values.iter().skip(1) {
        if prev_card_value == *card_value {
            count += 1;
            if count == 3 {
                is_full_house = true;
                break;
            }
        } else {
            count = 1;
        }
        prev_card_value = *card_value;
    }
    is_full_house
}

fn is_three_of_a_kind(card_values: &[char]) -> bool {
    let mut is_three_of_a_kind = false;
    let mut prev_card_value = card_values[0];
    let mut count = 1;
    for card_value in card_values.iter().skip(1) {
        if prev_card_value == *card_value {
            count += 1;
            if count == 3 {
                is_three_of_a_kind = true;
                break;
            }
        } else {
            count = 1;
        }
        prev_card_value = *card_value;
    }
    is_three_of_a_kind
}

fn is_two_pair(card_values: &[char]) -> bool {
    let mut is_two_pair = false;
    let mut prev_card_value = card_values[0];
    let mut count = 1;
    let mut pair_count = 0;
    for card_value in card_values.iter().skip(1) {
        if prev_card_value == *card_value {
            count += 1;
            if count == 2 {
                pair_count += 1;
                if pair_count == 2 {
                    is_two_pair = true;
                    break;
                }
            }
        } else {
            count = 1;
        }
        prev_card_value = *card_value;
    }
    is_two_pair
}

fn is_one_pair(card_values: &[char]) -> bool {
    let mut is_one_pair = false;
    let mut prev_card_value = card_values[0];
    let mut count = 1;
    for card_value in card_values.iter().skip(1) {
        if prev_card_value == *card_value {
            count += 1;
            if count == 2 {
                is_one_pair = true;
                break;
            }
        } else {
            count = 1;
        }
        prev_card_value = *card_value;
    }
    is_one_pair
}
