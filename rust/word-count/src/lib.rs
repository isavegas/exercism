use std::collections::HashMap;

pub fn word_count(data: &str) -> HashMap<String, u32> {
  let mut map: HashMap<String, u32> = HashMap::new();

  for word in data.split(|c| !char::is_alphanumeric(c)) {
    if word != "" {
      let count = map.entry(word.to_lowercase()).or_insert(0);
      *count += 1;
    }
  }
  map.shrink_to_fit();
  map
}