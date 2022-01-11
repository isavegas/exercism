use std::collections::HashMap;

pub fn nucleotide_counts(sequence: &str) -> HashMap<char, usize> {
  let mut a = 0;
  let mut t = 0;
  let mut c = 0;
  let mut g = 0;
  for ch in sequence.to_uppercase().chars() {
    match ch {
      'A' => a = a + 1,
      'T' => t = t + 1,
      'C' => c = c + 1,
      'G' => g = g + 1,
      _ => {}
    }
  }
  let mut counts: HashMap<char, usize> = HashMap::new();
  counts.insert('A', a);
  counts.insert('T', t);
  counts.insert('G', g);
  counts.insert('C', c);
  counts
}

pub fn count(nucleotide: char, sequence: &str) -> i32 {
  let mut count = 0;
  for c in sequence.to_uppercase().chars() {
    if c == nucleotide { count = count + 1; }
  }
  return count
}