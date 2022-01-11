pub fn hamming_distance(a: &str, b: &str) -> Result<i64, &'static str> {
  if a.len() != b.len() {
    return Err("inputs of different length");
  } else {
    // black magic
    return Ok(a.chars().zip(b.chars()).inspect(|&item| println!("{:?}", item)).filter(|&item| item.0 != item.1).fold(0, |i, _| i+1 ));
  }
}