use fast_paths::{InputGraph, FastGraph};
use crate::flatbuffer::TransitGraph;

pub fn preprocess_graph(transit_graph: &mut TransitGraph) -> FastGraph {
    let mut input_graph = InputGraph::new();

    for edge in transit_graph.edges().expect("Transit Graph doesn't have any edges").into_iter() {
        input_graph.add_edge(edge.start_node_index() as usize, edge.end_node_index() as usize, edge.weight() as usize);
    }

    input_graph.freeze();

    fast_paths::prepare(&input_graph)
}