mod status;
mod mixing;

pub use status::{State, DiseaseStatus, construct_disease_status_array};
pub use mixing::{Uniform, MixingStrategy};