pub fn factors_(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut divisor = 2;
    let mut m = n;
    while m > 1 {
        while m % divisor == 0 {
            factors.push(divisor);
            m /= divisor;
        }
        divisor += 1;
    }
    factors
}
