use std::fmt;
use std;

use graph::direction::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub index: usize,
}

impl Vertex {
    fn new() -> Vertex {
        Vertex {
            index: 0,
        }
    }

    fn from_index(index: usize) -> Vertex {
        Vertex {
            index: index,
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Edge {
    pub index: usize,
    pub source: Vertex,
    pub target: Vertex,
}

impl Edge {
    fn endpoint(&self, direction: Direction) -> Vertex {
        match direction {
            Direction::Outgoing => self.target,
            Direction::Incoming => self.source,
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.source, self.target)
    }
}

pub struct Graph {
    num_vertices: usize,
    edges: Vec<Edge>,
    outgoing: Vec<Vec<Edge>>,
    incoming: Vec<Vec<Edge>>,
}

struct VertexIt<'a> {
    curr: Vertex,
    graph: &'a Graph,
}

impl <'a> Iterator for VertexIt<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {

        let curr = self.curr;
        self.curr = Vertex::from_index(self.curr.index + 1);

        if curr.index >= self.graph.num_vertices {
            None
        } else {
            Some(curr)
        }
    }
}

type EdgeIt<'a> = std::slice::Iter<'a, Edge>;

impl Graph {
    pub fn new() -> Graph {
        Graph {
            num_vertices: 0,
            edges: Vec::new(),
            outgoing: Vec::new(),
            incoming: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self) -> Vertex {
        let s = self.num_vertices;
        self.num_vertices += 1;
        self.outgoing.push(Vec::new());
        self.incoming.push(Vec::new());

        Vertex::from_index(s)
    }

    pub fn vertices(&self) -> VertexIt {
        VertexIt {
            curr: Vertex::new(),
            graph: self,
        }
    }

    pub fn add_edge(&mut self, source: &Vertex, target: &Vertex) -> Edge {
        let edge = Edge {
            index: self.edges.len(),
            source: source.clone(),
            target: target.clone(),
        };

        self.outgoing[source.index].push(edge);
        self.incoming[target.index].push(edge);
        self.edges.push(edge);

        edge
    }

    pub fn edges(&self) -> EdgeIt {
        self.edges.iter()
    }

    pub fn out_edges(&self, vertex: Vertex) -> EdgeIt {
        self.outgoing[vertex.index].iter()
    }

    pub fn in_edges(&self, vertex: Vertex) -> EdgeIt {
        self.outgoing[vertex.index].iter()
    }

    pub fn adjacent_edges(&self, vertex: Vertex, direction: Direction) -> EdgeIt {
        match direction {
            Direction::Outgoing => self.out_edges(vertex),
            Direction::Incoming => self.in_edges(vertex),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_construction() {
        let mut graph = Graph::new();

        let n = 4;

        for _ in 0..n {
            graph.add_vertex();
        }

        let vertices : Vec<Vertex> = graph.vertices().collect();

        for v in &vertices {
            for w in &vertices {
                if v != w {
                    graph.add_edge(v, w);
                }
            }
        }

        for vertex in graph.vertices() {
            assert!(graph.out_edges(vertex).count() == n -1);
            for edge in graph.out_edges(vertex) {
                assert!(edge.source == vertex);
                assert!(edge.target != vertex);
            }
        }

        assert!(graph.vertices().count() == n);
        assert!(graph.edges().count() == n*(n-1))
    }
}
