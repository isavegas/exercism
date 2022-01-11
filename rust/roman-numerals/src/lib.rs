pub struct Roman {
  value: i32
}

impl Roman {
  pub fn from(i: i32) -> Roman {
    Roman { value : i }
  }
  pub fn to_string(&self) -> String {
    let mut string = String::new();
    let mut i = self.value;
    while i >= 1000 {
       string.push_str("M");
       i -= 1000;
    }
    while i >= 900 {
      string.push_str("CM");
      i -= 900;
    }
    while i >= 500 {
      string.push_str("D");
      i -= 500;
    }
    while i >= 400 {
      string.push_str("CD");
      i -= 400;
    }
    while i >= 100 {
       string.push_str("C");
       i -= 100;
    }
    while i >= 90 {
      string.push_str("XC");
      i -= 90;
    }
    while i >= 50 {
      string.push_str("L");
      i -= 50;
    }
    while i >= 40 {
      string.push_str("XL");
      i -= 40;
    }
    while i >= 10 {
      string.push_str("X");
      i -= 10;
    }
    while i >= 9 {
      string.push_str("IX");
      i -= 9;
    }
    while i >= 5 {
      string.push_str("V");
      i -= 5;
    }
    while i >= 4 {
      string.push_str("IV");
      i -= 4;
    }
    while i >= 1 {
      string.push_str("I");
      i -= 1;
    }
    return string;
  }
}