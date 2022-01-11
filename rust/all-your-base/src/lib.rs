#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

// Converts to internal u64 representation and outputs
// list of digits that represent the u64 in requested base
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u64>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let mut sum: u64 = 0;
    for (i, d) in number.iter().rev().enumerate() {
        if d >= &from_base {
            return Err(Error::InvalidDigit(*d));
        }
        sum += (*d as u64) * (from_base as u64).pow(i as u32);
        println!("{} += {} * {}.pow({})", sum, d, from_base, i);
    }
    println!("{}", sum);
    let mut digits = vec![];
    let base = to_base as u64;
    while sum > 0 {
        digits.push(sum % base);
        // Integer division
        sum /= base;
    }
    digits.reverse();
    Ok(digits)
}
