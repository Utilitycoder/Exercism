use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_sorted = sort_string(&word);

    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let possible_anagram = possible_anagram.to_lowercase();
            word != possible_anagram && word_sorted == sort_string(&possible_anagram)
        })
        .cloned()
        .collect()
}

fn sort_string(string: &str) -> Vec<char> {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_unstable();
    chars
}
