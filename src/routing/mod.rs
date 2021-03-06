use fast_paths::{InputGraph, FastGraph};
use crate::flatbuffer::TransitGraph;
use crate::routing::granular_grid::GranularGrid;
use crate::Bounds;
use crate::flatbuffer::Vec2;

mod granular_grid;

pub fn preprocess_graph(transit_graph: &mut TransitGraph) -> FastGraph {
    let mut input_graph = InputGraph::new();

    for edge in transit_graph.edges().expect("Transit Graph doesn't have any edges").into_iter() {
        input_graph.add_edge(edge.start_node_index() as usize, edge.end_node_index() as usize, edge.weight() as usize);
    }

    input_graph.freeze();

    fast_paths::prepare(&input_graph)
}

pub fn nodes_to_granular_grid(transit_graph: &TransitGraph, bounds: &Bounds) {
    let mut grid = GranularGrid::<usize>::new(200, 200);
    for (index, node) in transit_graph.nodes().unwrap().iter().enumerate() {
        let norm_x = node.pos().x() / bounds.max().x();
        let norm_y = node.pos().y() / bounds.max().y();
        grid[[norm_x as usize, norm_y as usize]].push(index)
    }
}