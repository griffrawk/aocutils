pub fn prime_factors(of: usize) -> Vec<usize> {
    /*
    Trial Division
    From comp prog book https://github.com/pllk/cphb p.191 (21.1 Primes and Factors)
    Also https://en.wikipedia.org/wiki/Trial_division
    */
    let mut prime_factors = Vec::new();
    let mut x = 2;
    let mut n = of;
    // x only goes as far as sqrt(n)
    while x * x <= n {
        while n % x == 0 {
            prime_factors.push(x);
            n /= x;
        }
        x += 1;
    }
    if n > 1 {
        prime_factors.push(n);
    }
    prime_factors
}

pub fn is_prime(n: usize) -> bool {
    prime_factors(n).len() == 1
}
