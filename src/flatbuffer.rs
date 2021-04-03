// import the flatbuffers runtime library
extern crate flatbuffers;

pub use model_generated::outbreak_sim::model::{
    Agents,
    Bounds,
    get_root_as_model,
    Households,
    Model,
    TransitEdge,
    TransitGraph,
    TransitNode,
    Vec2,
    Workplaces,
};

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./generated/model_generated.rs"]
mod model_generated;

pub fn read_buffer(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(bytes) => {
            bytes
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}