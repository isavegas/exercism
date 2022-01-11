pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => {
            let mut candidate = 4;
            let mut count = 0;
            let mut primes = vec![2, 3];
            while count < n {
                let half = candidate/2;
                let mut is_prime = true;
                for known_prime in primes.iter().filter(|k| **k < half) {
                    if candidate % known_prime == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    primes.push(candidate);
                    count += 1;
                }
                candidate += 1;
            }
            // should never error during normal execution
            primes.pop().expect("Unable to obtain prime")
        },
    }
}
