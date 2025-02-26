use std::cmp::{Ordering, Reverse};
use crate::point::Point;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::ops::Range;
use num::{abs, ToPrimitive};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<Point<usize>, Vec<Point<usize>>>,
    node_list: HashMap<Point<usize>, usize>,
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
        let mut node_list = HashMap::new();
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
                        let mut edges: Vec<Point<usize>> = Vec::new();
                        // let mut edges: Vec<(Point<usize>, EdgeData)> = Vec::new();
                        for cardinal in (Point {x: x as i32, y: y as i32 }).cardinal_points() {
                                let cardinal= Point { x: cardinal.x.to_usize().unwrap(), y: cardinal.y.to_usize().unwrap() };
                                if xrange.contains(&cardinal.x) && yrange.contains(&cardinal.y) {
                                    let n = maze[cardinal.y][cardinal.x];
                                    match n {
                                        '.' | 'S' | 'E' => edges.push(cardinal),
                                        // '.' | 'S' | 'E' => edges.push((cardinal, EdgeData::Weight(1))),
                                        _ => (),
                                    }
                            }
                        }
                        let node = Point { x, y };
                        node_list.insert(node, usize::MAX);
                        adjacency_list.insert(node, edges);
                    },
                    _ => (),
                }
            }
        }
        Self {
            adjacency_list,
            node_list,
            xrange,
            yrange,
            start,
            end,
        }
    }
    
    // Dijkstra's shortest path algorithm.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    fn shortest_path(&mut self) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost. node_list already init with usize::MAX
        self.node_list.insert(self.start, 0);
        heap.push(Reverse((0, self.start)));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Reverse((cost, position))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.end { return Some(cost); }

            // Important as we may have already found a better way
            if cost > self.node_list[&position] { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&position) {
                for node in edges {
                    let next_cost = cost + 1;
                    let next = Reverse((next_cost, *node));
                    // If so, add it to the frontier and continue
                    if next_cost < self.node_list[&node] {
                        heap.push(next);
                        // Relaxation, we have now found a better way
                        self.node_list.insert(*node, next_cost);
                    }
                }
            }
        }

        // Goal not reachable
        None
    }

    fn heuristic_distance(&mut self, pos: Point<usize>) -> usize {
        (abs(self.end.x as i32 - pos.x as i32) 
            + abs(self.end.y as i32 - pos.y as i32))
            .to_usize()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;
    use crate::point::Point;
    use super::Graph;

    #[test]
    fn test_graph() {
        let graph = Graph::new("src/test_graph.txt");
        dbg!(graph);
    }
    
    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new("src/test_graph.txt");
        assert_eq!(graph.shortest_path(), Some(28));
        dbg!(&graph.node_list);
    }
    
    #[test]
    fn test_binary_heap() {
        
        #[derive(Debug, Eq, PartialEq)]
        struct State {
            priority: usize,
            index: usize,
        }
        
        // min-heap
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // lowest priority
                other.priority.cmp(&self.priority)
            }
        }
        
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut heap = BinaryHeap::new();
        heap.push(State{ priority: 28, index: 34 });
        heap.push(State{ priority: 30, index: 32 });
        heap.push(State{ priority: 28, index: 3 });
        heap.push(State{ priority: 24, index: 4 });
        heap.push(State{ priority: 24, index: 43 });
        heap.push(State{ priority: 24, index: 45 });
        heap.push(State{ priority: 30, index: 9 });
        heap.push(State{ priority: 30, index: 12 });
        heap.push(State{ priority: 28, index: 56 });
        heap.push(State{ priority: 28, index: 22 });
        
        dbg!(heap.pop());       // 24
        dbg!(heap.pop());       // 24
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 28
        dbg!(heap.pop());       // 30
        dbg!(heap.pop());       // 30
    }
    
    #[test]
    fn test_priority_queue() {
        let mut queue: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
        
        queue.push(Reverse((28, 9)));
        queue.push(Reverse((6, 9)));
        queue.push(Reverse((2, 1)));
        queue.push(Reverse((24, 88)));
        queue.push(Reverse((7, 55)));
        queue.push(Reverse((1, 2)));
        
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        
        queue.push(Reverse((1, 3)));
        queue.push(Reverse((1, 5)));
        queue.push(Reverse((1, 2)));
        assert_eq!(queue.pop().unwrap(), Reverse((1, 2)));
        assert_eq!(queue.pop().unwrap(), Reverse((1, 3)));
        assert_eq!(queue.pop().unwrap(), Reverse((1, 5)));
    }
    
    #[test]
    fn test_another_priority_queue() {
        let mut queue: BinaryHeap<Reverse<(usize, Point<usize>)>> = BinaryHeap::new();

        queue.push(Reverse((8, Point{x:28, y: 99})));
        queue.push(Reverse((4, Point{x:18, y: 99})));
        queue.push(Reverse((34, Point{x:28, y: 99})));
        queue.push(Reverse((14, Point{x:28, y: 99})));
        
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        dbg!(queue.pop().unwrap());
        
        
        
    }
}
