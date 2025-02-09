use num::{Integer, Zero};

// Euclid's Algorithm for greatest common divisor
pub fn gcd<T: Integer + Clone>(a: T, b: T ) -> T {
    if b == Zero::zero() {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

pub fn lcm<T: Integer + Clone>(a: T, b: T) -> T {
    (a.clone() / gcd(a, b.clone())) * b
}

mod tests {
    use crate::divisors::{gcd, lcm};

    #[test]
    fn gcd_test() {
        dbg!(gcd(94, 22));
    }

    #[test]
    fn lcm_test() {
        dbg!(lcm(22, 56 ));
    }
}