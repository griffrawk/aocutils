use num::{Integer, One, Zero};
use num::integer::div_floor;

// Euclid's Algorithm for greatest common divisor
pub fn gcd<T: Integer + Clone>(a: T, b: T ) -> T {
    if b == Zero::zero() {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

// Extended Euclidean Algorithm
pub fn gcd_extended<T: Integer + Clone>(a: T, b: T) -> (T, T, T) {
    if a == Zero::zero() {
        return (b, Zero::zero(), One::one());
    }
    let (gcd, x1, y1) = gcd_extended(b.clone() % a.clone(), a.clone());
    let x = y1 - div_floor(b, a) * x1.clone();
    let y = x1;
    (gcd, x, y)
}

pub fn lcm<T: Integer + Clone>(a: T, b: T) -> T {
    (a.clone() / gcd(a, b.clone())) * b
}
