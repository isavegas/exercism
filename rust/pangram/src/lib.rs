// Add the ASCII numerical values for each char within a-z after
// making all valid characters lowercase and dedupping. Pretty neat!
pub fn is_pangram(sentence: &str) -> bool {
    let mut chars: Vec<u16> = sentence
        .chars()
        .map(|c| {
            c.to_lowercase()
                .next()
                .unwrap()
        })
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| c as u16)
        .collect();
    chars.sort();
    chars.dedup();
    chars.iter().sum::<u16>() == 2847
}
