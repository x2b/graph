mod graph;

use graph::graph::*;

fn main() {
    let mut graph = Graph::new();

    for i in 0..4 {
        println!("{}", i);
        graph.add_vertex();
    }

    let vertices : Vec<Vertex> = graph.vertices().collect();

    for v in &vertices {
        for w in &vertices {
            graph.add_edge(v, w);
        }
    }

    println!("Vertices: {}", graph.vertices().count());

    for v in graph.vertices() {
        println!("{}", v);
    }

    for e in graph.edges() {
        println!("{}", e);
    }

    println!("Hello, world!");
}
