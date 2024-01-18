use std::collections::{BTreeMap, BTreeSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut ranking = BTreeMap::<(Vec<usize>, Vec<usize>), Vec<&'a str>>::new();
    for hand in hands {
        let mut u = [0usize; 13];
        let mut suit = 0u8;
        for card in hand.split_ascii_whitespace() {
            u["234567891JQKA".find(card.chars().next().unwrap()).unwrap()] += 1;
            suit |= 1 << "CDHS".find(card.chars().last().unwrap()).unwrap();
        }
        let set: BTreeSet<_> = u.into_iter().zip(0..13).filter(|p| p.0 != 0).collect();
        let (mut counts, mut cards): (Vec<_>, Vec<_>) = set.into_iter().rev().unzip();
        if counts[0] == 1 {
            let mut straight = cards[0] - cards[4] == 4;
            if cards.starts_with(&[12, 3]) {
                cards.rotate_left(1);
                straight = true;
            }
            match (suit.is_power_of_two(), straight) {
                (false, true) => counts = vec![3, 1, 2],
                (true, false) => counts = vec![3, 1, 3],
                (true, true) => counts[0] = 5,
                _ => (),
            }
        } 
        ranking.entry((counts, cards)).or_default().push(hand);
    }
    ranking.pop_last().unwrap().1
}
