// graphemes support has been deprecated and official documentation
// indicates that unicode-segmentation from crates.io should be used.

pub fn reverse(input: &str) -> String {
  return input.chars().rev().collect();
}
