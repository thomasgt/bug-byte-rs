use petgraph::{
    dot::Dot,
    graph::{Graph, UnGraph},
};

#[derive(Debug)]
enum Node {
    Leaf,
    Sum(u32),
    Path(Vec<u32>),
    Start,
    End,
}

fn build_initial_graph() -> UnGraph<Node, Option<u32>> {
    let mut graph = Graph::new_undirected();
    let n0 = graph.add_node(Node::Sum(17));
    let n1 = graph.add_node(Node::Sum(3));
    let n2 = graph.add_node(Node::Path(vec![31]));
    let n3 = graph.add_node(Node::Path(vec![19, 23]));
    let n4 = graph.add_node(Node::Start);
    let n5 = graph.add_node(Node::Path(vec![8]));
    let n6 = graph.add_node(Node::Path(vec![6, 9, 16]));
    let n7 = graph.add_node(Node::Sum(54));
    let n8 = graph.add_node(Node::Sum(49));
    let n9 = graph.add_node(Node::Sum(60));
    let n10 = graph.add_node(Node::Sum(79));
    let n11 = graph.add_node(Node::Sum(75));
    let n12 = graph.add_node(Node::Leaf);
    let n13 = graph.add_node(Node::Sum(29));
    let n14 = graph.add_node(Node::Leaf);
    let n15 = graph.add_node(Node::Sum(39));
    let n16 = graph.add_node(Node::Sum(25));
    let n17 = graph.add_node(Node::End);

    graph.add_edge(n0, n1, None);
    graph.add_edge(n0, n3, None);
    graph.add_edge(n1, n4, None);
    graph.add_edge(n3, n4, Some(12));
    graph.add_edge(n3, n7, None);
    graph.add_edge(n4, n8, None);
    graph.add_edge(n2, n7, None);
    graph.add_edge(n5, n8, None);
    graph.add_edge(n6, n9, None);
    graph.add_edge(n7, n9, None);
    graph.add_edge(n8, n9, None);
    graph.add_edge(n7, n10, None);
    graph.add_edge(n8, n11, Some(20));
    graph.add_edge(n9, n10, Some(24));
    graph.add_edge(n9, n11, None);
    graph.add_edge(n10, n12, None);
    graph.add_edge(n10, n13, None);
    graph.add_edge(n10, n15, Some(7));
    graph.add_edge(n11, n14, None);
    graph.add_edge(n11, n16, None);
    graph.add_edge(n13, n15, None);
    graph.add_edge(n13, n16, None);
    graph.add_edge(n15, n17, None);
    graph.add_edge(n16, n17, None);

    graph
}

fn main() {
    let graph = build_initial_graph();
    let config = [];
    println!("{:?}", Dot::with_config(&graph, &config));
}
