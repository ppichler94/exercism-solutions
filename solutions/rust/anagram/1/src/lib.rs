use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .into_iter()
        .filter(|t| is_anagram(word, t))
        .map(|w| *w)
        .collect()
}

fn is_anagram(word: &str, target: &str) -> bool {
    if word.to_lowercase() == target.to_lowercase() {
        return false;
    }

    let word_chars = count_chars(word);
    let target_chars = count_chars(target);
    return word_chars == target_chars;
}

fn count_chars(word: &str) -> HashMap<char, u32> {
    let mut word_chars = HashMap::new();
    word.chars().for_each(|c| {
        let char = c.to_lowercase().next().unwrap();
        word_chars.insert(char, word_chars.get(&char).unwrap_or(&0u32) + 1);
    });

    word_chars
}
