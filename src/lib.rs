use std::time::Instant;

pub use flatbuffer::Bounds;
pub use flatbuffer::get_root_as_model;
pub use flatbuffer::Model;
pub use flatbuffer::read_buffer;
pub use flatbuffer::TransitGraph;
pub use flatbuffer::Vec2;

use crate::agents::Agents;
use crate::disease::{MixingStrategy, Uniform};
use crate::pois::Containers;
use fast_paths::FastGraph;

// TODO Revisit public access
pub mod agents;
pub mod pois;
pub mod disease;
pub mod shared;
pub mod routing;
mod flatbuffer;

pub struct Sim<M: MixingStrategy> {
    pub agents: Agents,
    pub containers: Containers<M>,
    pub bounds: Bounds,
    pub fast_graph: FastGraph
}

impl Sim<Uniform> {
    pub fn new(model_name: &str, load_fast_graph_from_disk: bool) -> Self {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let mixing_strategy = Uniform { transmission_chance: 0.04 };
        let model = get_root_as_model(&bytes);
        // TODO Ensure that this is non-inclusive
        let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

        let mut containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), mixing_strategy);
        let mut agents = agents::Agents::new(&model, &mut containers);

        let fast_graph = match load_fast_graph_from_disk {
            true => {
                let transit_graph = model.transit_graph();
                println!("Creating Contraction Hierarchies");
                let now = Instant::now();
                let fast_graph = routing::preprocess_graph(&transit_graph);
                println!("{:.6}s", now.elapsed().as_secs_f64());
                fast_paths::save_to_disk(&fast_graph, &*("fast_paths/".to_string() + model_name + ".fp")).unwrap();
                fast_graph
            }
            false => {
                fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap()
            }
        };

        let workplace_indices = model.agents().workplace_index().safe_slice();

        let num_commuting_agents = workplace_indices.iter()
            .filter(|&workplace_idx| {
                return *workplace_idx != u32::MAX;
            }).count();
        println!("{} Agents with a workplace", num_commuting_agents);

        Self {
            agents,
            containers,
            bounds,
            fast_graph
        }
    }

    pub fn update(&mut self) {
        self.containers.update(&mut self.agents);
    }
}