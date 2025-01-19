use num::{One, Zero};
use std::iter::zip;
use std::ops::{Add, Sub};

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

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
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

impl<T> Point<T>
where
    T: Sized //
        + Add<Output = T> // Add implemented where Output is T (above)
        + Sub<Output = T> // Add implemented where Output is T (above)
        + Zero // T implements Zero
        + One // T implements One
        + Clone, // T implements Clone
{
    pub fn cardinal_points(&self) -> Vec<Self> {
        let mut res: Vec<Point<T>> = Vec::new();
        let zero: T = Zero::zero();
        let minus_one: T = zero - One::one();
        let offsets = vec![
            Point {
                x: Zero::zero(),
                y: minus_one.clone(),
            }, // N
            Point {
                x: One::one(),
                y: Zero::zero(),
            }, // E
            Point {
                x: Zero::zero(),
                y: One::one(),
            }, // S
            Point {
                x: minus_one.clone(),
                y: Zero::zero(),
            }, // W
        ];
        for offset in offsets {
            let next_pos = self.clone() + offset;
            res.push(next_pos);
        }
        res
    }

    pub fn ordinal_points(&self) -> Vec<Self> {
        let mut res: Vec<Point<T>> = Vec::new();
        let zero: T = Zero::zero();
        let minus_one: T = zero - One::one();
        let offsets = vec![
            Point {
                x: One::one(),
                y: minus_one.clone(),
            }, // NE
            Point {
                x: One::one(),
                y: One::one(),
            }, // SE
            Point {
                x: minus_one.clone(),
                y: One::one(),
            }, // SW
            Point {
                x: minus_one.clone(),
                y: minus_one.clone(),
            }, // NW
        ];
        for offset in offsets {
            let next_pos = self.clone() + offset;
            res.push(next_pos);
        }
        res
    }

    pub fn compass_points(&self) -> Vec<Self> {
        // Clockwise! N, NE, E, SE, S, SW, W, NW
        let compass =
            zip(self.cardinal_points(), self.ordinal_points()).fold(Vec::new(), |mut arr, p| {
                // unpack the tuple into Vec
                arr.push(p.0);
                arr.push(p.1);
                arr
            });
        compass
    }
}

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
    fn test_sub_point() {
        assert_eq!(
            Point { x: 1, y: 0 } - Point { x: 2, y: 3 },
            Point { x: -1, y: -3 }
        );
    }

    #[test]
    fn test_cardinal_points() {
        let pt: Point<isize> = Point { x: 3, y: 4 };
        assert_eq!(
            pt.cardinal_points(),
            vec![
                Point { x: 3, y: 3 },
                Point { x: 4, y: 4 },
                Point { x: 3, y: 5 },
                Point { x: 2, y: 4 },
            ]
        );
    }
    #[test]
    fn test_ordinal_points() {
        let pt: Point<isize> = Point { x: 3, y: 4 };
        assert_eq!(
            pt.ordinal_points(),
            vec![
                Point { x: 2, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 5 },
                Point { x: 2, y: 5 },
            ]
        );
    }

    #[test]
    fn test_compass() {
        // Clockwise! N, NE, E, SE, S, SW, W, NW
        let pt: Point<i64> = Point { x: 3, y: 4 };
        assert_eq!(
            pt.compass_points(),
            vec![
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 4 },
                Point { x: 4, y: 5 },
                Point { x: 3, y: 5 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 3 },
            ]
        );
    }
}
