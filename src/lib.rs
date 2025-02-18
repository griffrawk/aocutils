pub mod divisors;
pub mod eratosthenes_sieve;
pub mod fibonacci_gen;
pub mod num_factors;
pub mod point;
pub mod prime_factors;
pub mod sums;

#[cfg(test)]
mod tests {
    use crate::divisors::gcd;
    use crate::eratosthenes_sieve::Sieve;
    use crate::fibonacci_gen::Fibseq;
    use crate::num_factors::{num_factors, num_factors_alt};
    use crate::prime_factors::prime_factors;

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(84), vec![2, 2, 3, 7]);
    }

    #[test]
    fn test_num_factors() {
        // Some triangle numbers
        assert_eq!(num_factors(1), 1);
        assert_eq!(num_factors(3), 2);
        assert_eq!(num_factors(6), 4);
        assert_eq!(num_factors(10), 4);
        assert_eq!(num_factors(15), 4);
        assert_eq!(num_factors(21), 4);
        assert_eq!(num_factors(28), 6);
        assert_eq!(num_factors(84), 12);
    }

    #[test]
    fn test_num_factors_alt() {
        // Some triangle numbers
        assert_eq!(num_factors_alt(1), 1);
        assert_eq!(num_factors_alt(3), 2);
        assert_eq!(num_factors_alt(6), 4);
        assert_eq!(num_factors_alt(10), 4);
        assert_eq!(num_factors_alt(15), 4);
        assert_eq!(num_factors_alt(21), 4);
        assert_eq!(num_factors_alt(28), 6);
        assert_eq!(num_factors_alt(84), 12);
    }

    #[test]
    fn test_fibonacci() {
        let mut seq = Fibseq::new();
        seq.next(); // 1
        seq.next(); // 1
        seq.next(); // 2
        seq.next(); // 3
        seq.next(); // 5
        let result = seq.next(); // 8
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_sieve() {
        let mut seq = Sieve::new(50);
        seq.next(); // 2
        seq.next(); // 3
        seq.next(); // 5
        seq.next(); // 7
        seq.next(); // 11
        let result = seq.next(); // 13
        assert_eq!(result, Some(13));
        for p in seq {
            println!("{}", p);
        }
    }

    #[test]
    fn test_gcd() {
        let result = gcd(10, 25);
        // println!("{:?}", result);
        assert_eq!(result, 5);
    }
}
