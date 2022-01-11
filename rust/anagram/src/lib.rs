use std::collections::HashSet;

// Simple sorted vector of lowercase char comparison
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let l_word = word.to_lowercase();
    let mut s_word: Vec<char> = word.to_lowercase().chars().collect();
    s_word.sort();
    possible_anagrams
        .iter()
        .filter(|a| a.len() == word.len())
        .map(|a| *a)
        .filter(|a| {
            let l_a = a.to_lowercase();
            if l_a == l_word {
                false
            } else {
                let mut s: Vec<char> = l_a.chars().collect();
                s.sort();
                s == s_word
            }
        })
        .collect()
}
