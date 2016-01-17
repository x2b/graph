use graph::graph::*;

pub struct Path {
    source: Vertex,
    edges: Vec<Edge>,
}

impl Path {
    fn new(source: Vertex) -> Path {
        Path {
            source: source,
            edges: Vec::new(),
        }
    }

    fn append(&mut self, edge: Edge) {
        match self.edges.last() {
            None =>
                if self.source != edge.source {
                    panic!();
                },
            Some(last_edge) =>
                if last_edge.target != edge.source {
                    panic!();
                },
        }

        self.edges.push(edge);
    }


    fn prepend(&mut self, edge: Edge) {

        if self.source != edge.target {
            panic!();
        }

        self.source = edge.source;
        self.edges.insert(0, edge);
    }
}

/*
#[cfg(test)]
mod test {

    use super::*;

    fn create_graph() -> Graph {
    }

}
*/
