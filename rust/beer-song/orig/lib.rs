#![feature(test)]
extern crate test;

const LINE_ZERO: &'static str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const LINE_ONE: &'static str  = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const LINE_TWO: &'static str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(i: i32) -> String {
  match i {
    0 => LINE_ZERO.to_string(),
    1 => LINE_ONE.to_string(),
    2 => LINE_TWO.to_string(),
    _ => format!("{a} bottles of beer on the wall, {a} bottles of beer.\nTake one down and pass it around, {b} bottles of beer on the wall.\n", a = i, b = i - 1)
  }
}

pub fn sing(start: i32, stop: i32) -> String {
  let mut out = String::from("");
  let mut i: i32 = start;
  while i >= stop {
    out.push_str(verse(i).as_str());
    if i != stop { out.push_str("\n"); }
    i = i - 1;
  }
  return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_verse(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..99 {
                verse(i);
            }
        });
    }
    #[bench]
    fn bench_sing(b: &mut Bencher) {
        b.iter(|| {
            sing(99, 0);
        });
    }
}