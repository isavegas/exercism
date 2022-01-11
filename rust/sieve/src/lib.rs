pub fn primes_up_to(n: i32) -> Vec<i32> {
  let mut primes: Vec<i32> = Vec::new();
  let mut sieve: Vec<i32> = Vec::new();
  if n >= 2 {
    for i in 2..n+1 {
      if !sieve.contains(&i) {
        primes.push(i);
        let mut q = i;
        while q * i <= n {
          sieve.push(q*i);
          q += 1;
        }
      }
    }
  }
  return primes;
}