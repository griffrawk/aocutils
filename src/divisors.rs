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

mod tests {
    use crate::divisors::{gcd, gcd_extended, lcm};

    #[test]
    fn gcd_test() {
        let g = gcd(39, 15);
        dbg!(g);
    }

    #[test]
    fn extended_gcd_test() {
        let a = 94;
        let b = 22;
        let res = gcd_extended(a, b);
        dbg!(res);
    }
#[test]
    fn extended_gcd_all_solutions_test() {
        // pretty, but doesn't get me any closer
        // any value of k also gives solutions
        let a = 94;
        let b = 22;
        let (gcd, x, y) = gcd_extended(a, b);
        dbg!(gcd, x, y);
        let mut k = 0;
        while k < 10 {
            let xn = x + ((k * b) / gcd);
            let yn = y - ((k * a) / gcd);
            dbg!(k, xn, yn);
            k += 1;

        }

    }


    #[test]
    fn lcm_test() {
        dbg!(lcm(94, 34 ));
    }
}