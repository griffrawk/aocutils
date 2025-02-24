use crate::point::Point;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
pub enum EdgeData {
    Weight(usize),
}

#[derive(Debug)]
pub struct Node {
    // Dijkstra distance
    g: usize,
    // Heuristic distance
    h: usize,
}

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<Point<usize>, Vec<(Point<usize>, EdgeData)>>,
    // node_list: HashMap<Point<usize>, Node>,
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
        let mut maze: Vec<Vec<char>> = Vec::new();
        for (y, row) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate()
        {
            maze.push(row.chars().collect());
            xrange = 0..row.len();
            yrange = 0..y + 1;
        }
        for (y, row) in maze.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let node = Point { x, y };
                match c {
                    '.' | 'S' | 'E' => {
                        if *c == 'S' {
                            start.x = x;
                            start.y = y;
                        }
                        if *c == 'E' {
                            end.x = x;
                            end.y = y;
                        }
                        let mut edges: Vec<(Point<usize>, EdgeData)> = Vec::new();
                        for cardinal in node.cardinal_usize() {
                            if let Some(card) = cardinal {
                                if xrange.contains(&card.x) && yrange.contains(&card.y) {
                                    let n = maze[card.y][card.x];
                                    match n {
                                        '.' | 'S' | 'E' => edges.push((card, EdgeData::Weight(1))),
                                        _ => (),
                                    }
                                }
                            }
                        }
                        adjacency_list.insert(node, edges);
                    },
                    _ => (),
                }
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
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use crate::point::Point;
    use super::Graph;

    #[test]
    fn test_graph() {
        let graph = Graph::new("src/test_graph.txt");
        dbg!(graph);
    }
    
    #[test]
    fn test_binary_heap() {
        
        #[derive(Debug, Eq, PartialEq)]
        struct State {
            priority: usize,
            pos: Point<usize>,
        }
        
        // min-heap
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // lowest priority, then by highest pos.x
                other.priority.cmp(&self.priority)
                    .then_with(|| self.pos.x.cmp(&other.pos.x))
            }
        }
        
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut heap = BinaryHeap::new();
        heap.push(State{ priority: 28, pos: Point {x: 13, y: 19 } });
        heap.push(State{ priority: 30, pos: Point {x: 3, y: 29 } });
        heap.push(State{ priority: 28, pos: Point {x: 13, y: 49 } });
        heap.push(State{ priority: 24, pos: Point {x: 15, y: 11 } });
        heap.push(State{ priority: 28, pos: Point {x: 23, y: 32 } });
        heap.push(State{ priority: 24, pos: Point {x: 19, y: 12 } });
        heap.push(State{ priority: 30, pos: Point {x: 43, y: 15 } });
        
        dbg!(heap.pop());       // 24
        dbg!(heap.pop());       // 24
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 30
        dbg!(heap.pop());       // 30
    }
    
    
}
