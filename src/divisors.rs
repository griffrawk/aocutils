use num::abs;

// Euclid's Algorithm for greatest common divisor
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        // Just for explanation purposes while looking at Diophantine equations
        println!("{} - ({} * {}) = {}", a, abs(a / b), b, a % b);
        gcd(b, a % b)
    }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

mod tests {
    use crate::divisors::gcd;

    #[test]
    fn gcd_test() {
        dbg!(gcd(94, 22));
    }
}