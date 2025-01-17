// use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

// impl<T> Point<T> {
//     pub fn nesw(pos: Point<T>) -> Vec<Point<T>>
//         where for<'a> &'a T: Add<i32>
//     {
//         let mut res: Vec<Point<T>> = Vec::new();
//         for (dx, dy)  in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
//             let next_pos: Point<i32> = Point { x: &pos.x + dx, y: &pos.y + dy };
//             res.push(next_pos);
//         }
//         res
//     }
// }
