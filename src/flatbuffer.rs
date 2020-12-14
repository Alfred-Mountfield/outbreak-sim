// import the flatbuffers runtime library
extern crate flatbuffers;

use std::fs::File;
use std::io::Read;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./generated/model_generated.rs"]
mod model_generated;

pub use model_generated::outbreak_sim::model::{
    get_root_as_model,
    Agents,
    Container,
    Households,
    Model,
    Vec2,
    Workplaces,
};

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