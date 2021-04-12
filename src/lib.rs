use fast_paths::FastGraph;

pub use flatbuffer::Bounds;
pub use flatbuffer::get_root_as_model;
pub use flatbuffer::Model;
pub use flatbuffer::read_buffer;
pub use flatbuffer::TransitGraph;
pub use flatbuffer::Vec2;
use routing::transit::get_fast_graph;
use shared::types::TimeStep;

use crate::agents::Agents;
use crate::containers::Containers;
use crate::disease::{MixingStrategy, Uniform};
use crate::events::Events;
use crate::routing::{GranularGrid, nodes_to_granular_grid};
use crate::shared::TIME_STEPS_PER_DAY;
use std::path::PathBuf;

// TODO Revisit public access
pub mod agents;
pub mod containers;
pub mod disease;
pub mod shared;
pub mod routing;
pub mod events;
pub mod reporting;
mod flatbuffer;

// TODO static Cell<> for global params

// TODO derive Clone when FastGraphs updates
pub struct Sim<M: MixingStrategy> {
    pub agents: Agents,
    pub events: Events,
    pub containers: Containers<M>,
    pub bounds: Bounds,
    pub fast_graph: FastGraph,
    pub transit_granular_grid: GranularGrid<usize>,
}

impl Sim<Uniform> {
    // TODO Builder pattern for input params?
    pub fn new<P>(synthetic_environment_dir: P, model_name: &str, load_fast_graph_from_disk: bool) -> Self
        where P: Into<PathBuf>
    {
        // set_up_global_params();
        let mut synthetic_environment_file = synthetic_environment_dir.into();
        synthetic_environment_file.push(model_name);
        synthetic_environment_file.set_extension("txt");
        let bytes = read_buffer(synthetic_environment_file.as_path());
        let model = get_root_as_model(&bytes);

        let transmission_chance = 0.01 * 24.0 / TIME_STEPS_PER_DAY as f32;
        let mixing_strategy = Uniform { transmission_chance };
        // TODO Ensure that this is non-inclusive
        let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

        let mut containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), mixing_strategy);
        let mut agents = agents::Agents::new(&model, &mut containers);
        let events = events::Events::new(&mut agents);

        let fast_graph = match load_fast_graph_from_disk {
            true => {
                fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap()
            }
            false => {
                let fast_graph = get_fast_graph(&model.transit_graph());
                fast_paths::save_to_disk(&fast_graph, &*("fast_paths/".to_string() + model_name + ".fp")).unwrap();
                fast_graph
            }
        };

        let transit_granular_grid = nodes_to_granular_grid(&model.transit_graph(), &bounds, 100);

        Self {
            agents,
            events,
            containers,
            bounds,
            fast_graph,
            transit_granular_grid,
        }
    }

    pub fn update(&mut self, time_step: TimeStep) {
        let mut fast_path_calculator = fast_paths::create_calculator(&self.fast_graph);
        self.events.update(time_step, &mut self.agents, &mut self.containers, &self.transit_granular_grid, &self.fast_graph, &mut fast_path_calculator);
    }
}