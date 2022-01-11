pub fn encode(n: u64) -> String {
  let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

  if n == 0 {
    return String::from(digits[0]); // Only time we'll ever use zero
  }
  if n == 1 {
    return String::from(digits[1]); // 1 borks everything up due to division by zero
  }

  let mut num: u64 = n;
  let mut parts: Vec<String> = vec!();

  let quintillions = num / 1_000_000_000_000_000_000;
  if quintillions > 0 {
    parts.push(format!("{} quintillion", encode(quintillions)));
    num -= quintillions * 1_000_000_000_000_000_000;
  }
  let quadrillions = num / 1_000_000_000_000_000;
  if quadrillions > 0 {
    parts.push(format!("{} quadrillion", encode(quadrillions)));
    num -= quadrillions * 1_000_000_000_000_000;
  }
  let trillions = num / 1_000_000_000_000;
  if trillions > 0 {
    parts.push(format!("{} trillion", encode(trillions)));
    num -= trillions * 1_000_000_000_000;
  }
  let billions = num / 1_000_000_000;
  if billions > 0 {
    parts.push(format!("{} billion", encode(billions)));
    num -= billions * 1_000_000_000;
  }
  let millions = num / 1_000_000;
  if millions > 0 {
    parts.push(format!("{} million", encode(millions)));
    num -= millions * 1_000_000;
  }
  let thousands = num / 1_000;
  if thousands > 0 {
    parts.push(format!("{} thousand", encode(thousands)));
    num -= thousands * 1_000;
  }
  let hundreds = num / 100;
  if hundreds > 0 {
    parts.push(format!("{} hundred", encode(hundreds)));
    num -= hundreds * 100;
  }
  let tens = num / 10;
  num -= tens * 10;
  let mut last_word = String::new();
  match tens {
    9 => last_word = String::from("ninety"),
    8 => last_word = String::from("eighty"),
    7 => last_word = String::from("seventy"),
    6 => last_word = String::from("sixty"),
    5 => last_word = String::from("fifty"),
    4 => last_word = String::from("forty"),
    3 => last_word = String::from("thirty"),
    2 => last_word = String::from("twenty"),
    1 => {
      match num {
        9 => parts.push(String::from("nineteen")),
        8 => parts.push(String::from("eighteen")),
        7 => parts.push(String::from("seventeen")),
        6 => parts.push(String::from("sixteen")),
        5 => parts.push(String::from("fifteen")),
        4 => parts.push(String::from("fourteen")),
        3 => parts.push(String::from("thirteen")),
        2 => parts.push(String::from("twelve")),
        1 => parts.push(String::from("eleven")),
        0 => parts.push(String::from("ten")),
        _ => ()
      }
    }
    _ => ()
  }
  if tens > 1 && num > 0 {
    last_word.push('-');
  }
  if tens != 1 {
    match num {
      9 => last_word.push_str(digits[9]),
      8 => last_word.push_str(digits[8]),
      7 => last_word.push_str(digits[7]),
      6 => last_word.push_str(digits[6]),
      5 => last_word.push_str(digits[5]),
      4 => last_word.push_str(digits[4]),
      3 => last_word.push_str(digits[3]),
      2 => last_word.push_str(digits[2]),
      1 => last_word.push_str(digits[1]),
      _ => ()
    }
  }
  if last_word.as_str() != "" {
    parts.push(last_word);
  }

  return parts.join(" ");
}
