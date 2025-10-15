use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let low_word = word.to_lowercase();
    let sorted_word = get_sorted(&low_word);

    possible_anagrams
        .iter()
        .filter(|candidata| {
            let lower = candidata.to_lowercase();
            lower.len() == low_word.len() && lower != low_word && get_sorted(&lower) == sorted_word
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted
}
