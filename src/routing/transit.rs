use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use fast_paths::{FastGraph, InputGraph};

use crate::TransitGraph;

// struct TransitNode {
//
// }
//
// pub fn parse_transit_schedules(transit_graph: &TransitGraph) {
//     transit_graph.edge_rides().iter().map(|edge_rides| {
//         edge_rides.rides().iter().map(|ride| {
//             ride.
//         })
//     });
// }

pub fn get_fast_graph(transit_graph: &TransitGraph) -> FastGraph {
    println!("Creating Contraction Hierarchies");
    let now = Instant::now();
    let fast_graph = preprocess_graph(transit_graph);
    println!("{:.6}s", now.elapsed().as_secs_f64());
    fast_graph
}

pub fn load_fast_graph_from_disk<P: AsRef<Path>>(file_name: P) -> Result<FastGraph, Box<dyn Error>> {
    let file = File::open(file_name)?;
    Ok(bincode::deserialize_from(file)?)
}

pub fn save_fast_graph_to_disk<P: AsRef<Path>>(fast_graph: &FastGraph, file_name: P) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_name)?;
    Ok(bincode::serialize_into(file, fast_graph)?)
}

/// Creates a fast_paths Graph from the FlatBuffers TransitGraph edges data
pub fn preprocess_graph(transit_graph: &TransitGraph) -> FastGraph {
    let mut input_graph = InputGraph::new();

    for edge in transit_graph.edges().iter() {
        input_graph.add_edge(edge.start_node_index() as usize, edge.end_node_index() as usize, edge.weight() as usize);
    }

    input_graph.freeze();

    fast_paths::prepare(&input_graph)
}
