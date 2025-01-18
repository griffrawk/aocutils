use std::ops::Add;
use num::{One, Zero};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point<i32> {
    // return a Vec of 4 cardinal points surrounding the input
    pub fn cardinal_points(&self) -> Vec<Point<i32>> {
        let mut res = Vec::new();
        for offset in [
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
        ] {
            let next_pos = *self + offset;
            res.push(next_pos);
        }
        res
    }
}

impl Point<i64> {
    // return a Vec of 4 cardinal points surrounding the input
    pub fn cardinal_points(&self) -> Vec<Point<i64>> {
        let mut res = Vec::new();
        for offset in [
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
        ] {
            let next_pos = *self + offset;
            res.push(next_pos);
        }
        res
    }
}

// Horrible. Compiler suggestions one after the other give this indecipherable impl and
// fn. Not sure how that return value is supposed to work. I just about understand the where
// clause but not the lifetime spec
// Can't create a vec! of integer literals as T, so have to try this num crate
// num crate gives one() & zero() which are T, but no negative numbers, so my Point
// array isn't correct.
// I have to use T, as the input is Point<T> and let it sort it out.

// impl<T> Point<T> where for<'a> &'a Point<T>: Add<Point<T>> {
//     pub fn cardinal_points(&self) -> Vec<<&Point<T> as Add<Point<T>>>::Output> {
//         let mut res = Vec::new();
//         let mut minus_one: T = Zero::zero();
//         // can't even do this, doesnt support subtraction or -ve numbers...
//         minus_one = minus_one - One::one();
//         let offsets = vec![
//             Point { x: Zero::zero(), y: minus_one.clone() },
//             Point { x: One::one(), y: Zero::zero() },
//             Point { x: Zero::zero(), y: One::one() },
//             Point { x: minus_one.clone(), y: Zero::zero() },
//         ];
//         for offset in offsets {
//             let next_pos = self + offset;
//             res.push(next_pos);
//         }
//         res
//     }
// }

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn test_add_point() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    #[test]
    fn test_cardinal_points_i32() {
        let pt: Point<i32> = Point { x: 3, y: 4 };
        assert_eq!(pt.cardinal_points(), vec![
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 2, y: 4 },
        ]);
    }

    #[test]
    fn test_cardinal_points_i64() {
        let pt: Point<i64> = Point { x: 3, y: 4 };
        assert_eq!(pt.cardinal_points(), vec![
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 2, y: 4 },
        ]);
    }
}

