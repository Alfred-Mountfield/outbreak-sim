// import the flatbuffers runtime library
extern crate flatbuffers;

pub use model_generated::outbreak_sim::model::{
    Agents,
    Bounds,
    get_root_as_model,
    Households,
    Model,
    TransitGraph,
    TransitEdge,
    TransitNode,
    TransitEdgeRides,
    TransitRide,
    Vec2,
    Workplaces,
};
use std::path::Path;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./generated/model_generated.rs"]
mod model_generated;

pub fn read_buffer(path: &Path) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(bytes) => {
            bytes
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
            } else if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Couldn't find file: {}", path.display());
            }
            panic!("{}", e);
        }
    }
}