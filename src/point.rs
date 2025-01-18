use std::ops::Add;

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

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn point_add() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    #[test]
    fn point_nesw() {
        let pt: Point<i32> = Point { x: 3, y: 4 };
        assert_eq!(pt.cardinal_points(), vec![
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 2, y: 4 },
        ]);
    }
}

