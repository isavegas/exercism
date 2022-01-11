pub fn char_score(c: char) -> i32 {
  match c {
    'q'|'z' => 10,
    'j'|'x' => 8,
    'k' => 5,
    'f'|'h'|'v'|'w'|'y' => 4,
    'b'|'c'|'m'|'p' => 3,
    'd'|'g' => 2,
    'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
    _ => 0
  }
}

pub fn score(input: &str) -> i32 {
  return input.to_lowercase().chars().fold(0, |sum, n| sum + char_score(n));
}