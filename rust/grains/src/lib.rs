pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64")
    } else {
        2_u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
