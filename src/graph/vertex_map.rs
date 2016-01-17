use std::ops::{Index,IndexMut};

use graph::graph::*;

struct VertexMap<'a, Output> {
    graph: &'a Graph,
    elements: Vec<Output>,
}

impl <'a, Output> VertexMap<'a, Output> {

    fn new(graph: &'a Graph, def: Output) -> VertexMap<'a, Output>
        where Output : Copy {

        let mut elements = Vec::with_capacity(graph.edges().count());

        for edge in graph.edges() {
            elements.push(def);
        }

        VertexMap {
            graph: graph,
            elements: elements,
        }
    }

    fn from_func<Func>(graph: &'a Graph, func: Func) -> VertexMap<'a, Output>
        where Func: Fn(Vertex) -> Output {

        let mut elements = Vec::with_capacity(graph.edges().count());

        for vertex in graph.vertices() {
            elements.push(func(vertex));
        }

        VertexMap {
            graph: graph,
            elements: elements,
        }
    }
}

impl <'a, Output> Index<Vertex> for VertexMap<'a, Output> {
    type Output = Output;

    fn index(&self, vertex: Vertex) -> &Output {
        self.elements.index(vertex.index)
    }
}

impl <'a, Output> IndexMut<Vertex> for VertexMap<'a, Output> {
    fn index_mut(&mut self, vertex: Vertex) -> &mut Output {
        self.elements.index_mut(vertex.index)
    }
}

#[cfg(test)]
mod test {

    use graph::graph::*;
    use super::VertexMap;

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

        let mut map : VertexMap<usize> = VertexMap::new(&graph, 0);

        for vertex in graph.vertices() {
            map[vertex] = vertex.index;
        }

        for vertex in graph.vertices() {
            assert!(map[vertex] == vertex.index);
        }

        let mut map : VertexMap<usize> = VertexMap::from_func(&graph, |vertex| vertex.index);

        for vertex in graph.vertices() {
            assert!(map[vertex] == vertex.index);
        }

        let r : &VertexMap<usize> = &map;
    }
}
