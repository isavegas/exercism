pub fn hex_to_int(hex_str: &str) -> Option<i32> {
  let mut decimal: i32 = 0;
  let len = hex_str.len();
  for (i, c) in hex_str.to_uppercase().chars().rev().enumerate() {
    let sig = i as i32;
    let mut d = 0;
    if c.is_alphabetic() {
      match c {
        'A' => d = 10,
        'B' => d = 11,
        'C' => d = 12,
        'D' => d = 13,
        'E' => d = 14,
        'F' => d = 15,
        _ => return None
      }
    } else {
      let val = c.to_string().parse::<i32>();
      if val.is_err() {
        return None
      } else {
        d = val.unwrap();;
      }
    }
    decimal += d * 16_i32.pow(sig as u32);
  }
  return Some(decimal);
}
/*
pub fn hex_to_int_fold(hex_str: &str) -> Option<i32> {
  Some(hex_str.chars().rev().fold(0, |sum, c| {
    return 1; // TODO
  }))
}
*/