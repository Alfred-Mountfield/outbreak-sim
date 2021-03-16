// TODO Revisit public access
pub mod agents;
pub mod pois;
pub mod disease;
pub mod shared;
pub mod routing;
mod flatbuffer;

pub use flatbuffer::Model;
pub use flatbuffer::read_buffer;
pub use flatbuffer::get_root_as_model;
pub use flatbuffer::Bounds;
pub use flatbuffer::Vec2;
pub use flatbuffer::TransitGraph;
use crate::disease::{Uniform, MixingStrategy};
use std::time::Instant;
use crate::pois::Containers;
use crate::agents::Agents;

pub struct Sim<M>
    where M: MixingStrategy
{
    pub agents: Agents,
    pub containers: Containers<M>,
    pub bounds: Bounds
}

impl Sim<Uniform> {
    pub fn new(model_name: &str) -> Self {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let mixing_strategy = Uniform { transmission_chance: 0.04 };
        let model = get_root_as_model(&bytes);
        // TODO Ensure that this is non-inclusive
        let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

        let mut containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), mixing_strategy);
        let mut agents = agents::Agents::new(&model, &mut containers);

        let transit_graph = model.transit_graph();

        // println!("Creating Contraction Hierarchies");
        // let now = Instant::now();
        // let fast_graph = routing::preprocess_graph(&transit_graph);
        // println!("{:.6}s", now.elapsed().as_secs_f64());
        // fast_paths::save_to_disk(&fast_graph, &*("fast_paths/".to_string() + model_name + ".fp")).unwrap();

        let fast_graph = fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap();
        let workplace_indices = model.agents().workplace_index().safe_slice();

        let num_commuting_agents = workplace_indices.iter()
            .filter(|&workplace_idx| {
                return *workplace_idx != u32::MAX;
            }).count();
        println!("{} Agents with a workplace", num_commuting_agents);

        Self {
            agents,
            containers,
            bounds
        }
    }

    pub fn update(&mut self) {
        self.containers.update(&mut self.agents);
    }
}