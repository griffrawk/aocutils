use crate::point::Point;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
pub enum NodeData {
    Weight(usize),
}

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<Point<usize>, Vec<(Point<usize>, NodeData)>>,
    xrange: Range<usize>,
    yrange: Range<usize>,
    start: Point<usize>,
    end: Point<usize>,
}

impl Graph {
    pub fn new(file: &str) -> Self {
        let mut xrange = Range::default();
        let mut yrange = Range::default();
        let mut start = Point::default();
        let mut end = Point::default();
        let mut adjacency_list = HashMap::new();
        for (y, row) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate() 
        {
            xrange = 0..row.len();
            yrange = 0..y + 1;
            for (x, c) in row.chars().enumerate() {
                let node = Point { x, y };
                let mut edges: Vec<(Point<usize>, NodeData)> = Vec::new();
                match c {
                    '#' => {
                        for cardinal in node.cardinal_points() {
                            if xrange.contains(&cardinal.x) && yrange.contains(&cardinal.y) {
                                let n = input_vec[cardinal.y].to_string()[cardinal.x]; 
                                match n {
                                    '.' => edges.push((cardinal, NodeData::Weight(1))),
                                    _ => (),
                                }
                            }
                        }
                    },
                    'S' => {
                        start.x = x;
                        start.y = y;
                    },
                    'E' => {
                        end.x = x;
                        end.y = y;
                    },
                    _ => (),
                }
                adjacency_list.insert(node, edges);
            }
        }

        Self {
            adjacency_list,
            xrange,
            yrange,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    
    #[test]
    fn test_graph() {
        let graph = Graph::new("src/test_graph.txt");
        dbg!(graph);
    }
}
