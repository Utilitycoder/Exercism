use std::collections::HashMap;

pub fn frequency(texts: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    texts.iter().for_each(|text| {
        text.to_lowercase().chars().filter(|c| c.is_alphabetic()).for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        })
    });
    map
}
