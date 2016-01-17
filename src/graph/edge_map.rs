use std::ops::{Index,IndexMut};

use graph::graph::*;

struct EdgeMap<'a, Output> {
    graph: &'a Graph,
    elements: Vec<Output>,
}

impl <'a, Output> EdgeMap<'a, Output> {

    fn new(graph: &'a Graph, def: Output) -> EdgeMap<'a, Output>
        where Output : Copy {

        let mut elements = Vec::with_capacity(graph.edges().count());

        for edge in graph.edges() {
            elements.push(def);
        }

        EdgeMap {
            graph: graph,
            elements: elements,
        }
    }

    fn from_func<Func>(graph: &'a Graph, func: Func) -> EdgeMap<'a, Output>
        where Func: Fn(&Edge) -> Output {

        let mut elements = Vec::with_capacity(graph.edges().count());

        for edge in graph.edges() {
            elements.push(func(edge));
        }

        EdgeMap {
            graph: graph,
            elements: elements,
        }
    }
}

impl <'a, Output> Index<&'a Edge> for EdgeMap<'a, Output> {
    type Output = Output;

    fn index(&self, edge: &'a Edge) -> &Output {
        self.elements.index(edge.index)
    }
}

impl <'a, Output> IndexMut<&'a Edge> for EdgeMap<'a, Output> {
    fn index_mut(&mut self, edge: &'a Edge) -> &mut Output {
        self.elements.index_mut(edge.index)
    }
}

#[cfg(test)]
mod test {

    use graph::graph::*;
    use super::EdgeMap;

    #[test]
    fn test_map() {
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

        let mut map = EdgeMap::new(&graph, 0);

        for edge in graph.edges() {
            map[edge] = edge.index;
        }

        for edge in graph.edges() {
            assert!(map[edge] == edge.index);
        }

        let mut map = EdgeMap::from_func(&graph, |edge| edge.index);

        for edge in graph.edges() {
            assert!(map[edge] == edge.index);
        }
    }
}
