use std::fmt;
use std::path::PathBuf;

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
use crate::shared::{GlobalSimParams, set_up_global_params, get_time_steps_per_day, get_simulation_length_in_days};
use crate::routing::transit::{load_fast_graph_from_disk, save_fast_graph_to_disk};

// TODO Revisit public access
pub mod agents;
pub mod containers;
pub mod disease;
pub mod shared;
pub mod routing;
pub mod events;
pub mod reporting;
mod flatbuffer;

#[derive(Debug, Clone)]
pub struct EndOfSimulationError;

impl fmt::Display for EndOfSimulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "simulation has reached the maximum {} days", get_time_steps_per_day())
    }
}

#[derive(Clone)]
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
    fn new<P>(synthetic_environment_dir: P, model_name: &str, load_cached_fast_graph: bool, global_params: GlobalSimParams) -> Self
        where P: Into<PathBuf>
    {
        set_up_global_params(global_params);
        let mut synthetic_environment_file = synthetic_environment_dir.into().join(model_name);
        synthetic_environment_file.set_extension("txt");
        let bytes = read_buffer(synthetic_environment_file.as_path());
        let model = get_root_as_model(&bytes);

        let transmission_chance = 0.00005 * 24.0 / get_time_steps_per_day() as f32;
        let mixing_strategy = Uniform { transmission_chance };
        // TODO Ensure that this is non-inclusive
        let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

        let mut containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), mixing_strategy);
        let mut agents = agents::Agents::new(&model, &mut containers);
        let events = events::Events::new(&mut agents);

        let fast_graph = match load_cached_fast_graph {
            true => {
                load_fast_graph_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap()
            }
            false => {
                let fast_graph = get_fast_graph(&model.transit_graph());
                save_fast_graph_to_disk(&fast_graph, &*("fast_paths/".to_string() + model_name + ".fp")).unwrap();
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

    pub fn update(&mut self, time_step: TimeStep) -> Result<(), EndOfSimulationError> {
        if get_simulation_length_in_days().is_some() && time_step >= get_simulation_length_in_days().unwrap() * get_time_steps_per_day() {
            return Err(EndOfSimulationError)
        }
        let mut fast_path_calculator = fast_paths::create_calculator(&self.fast_graph);
        self.events.update(time_step, &mut self.agents, &mut self.containers, &self.transit_granular_grid, &self.fast_graph, &mut fast_path_calculator);

        Ok(())
    }
}

pub struct SimBuilder<'a, P: Into<PathBuf>> {
    global_params: GlobalSimParams,
    synthetic_environment_dir: P,
    model_name: &'a str,
    load_fast_graph_from_disk: bool,
    walking_speed_kph: f32,
    cycling_speed_kph: f32,
    driving_speed_kph :f32,
}

impl<'a, P> SimBuilder<'a, P> where P: Into<PathBuf>{
    pub fn new(synthetic_environment_dir: P, model_name: &'a str) -> Self {
        SimBuilder {
            global_params: GlobalSimParams::default(),
            synthetic_environment_dir,
            model_name,
            load_fast_graph_from_disk: false,
            walking_speed_kph: 5.0,
            cycling_speed_kph: 23.5,
            driving_speed_kph: 60.0,
        }
    }

    pub fn time_steps_per_day(mut self, time_steps: u32) -> Self {
        self.global_params.time_steps_per_day = time_steps;
        self
    }
    
    pub fn sim_length_days(mut self, days: Option<u32>) -> Self {
        self.global_params.sim_length_days = days;
        self
    }
    
    pub fn seed_infection_chance(mut self, seed_infection_chance: f32) -> Self {
        self.global_params.seed_infection_chance = seed_infection_chance;
        self
    }
    
    pub fn walking_speed_kph(mut self, walking_speed_kph: f32) -> Self {
        self.walking_speed_kph = walking_speed_kph;
        self
    }
    
    pub fn cycling_speed_kph(mut self, cycling_speed_kph: f32) -> Self {
        self.cycling_speed_kph = cycling_speed_kph;
        self
    }
    
    pub fn driving_speed_kph(mut self, driving_speed_kph: f32) -> Self {
        self.driving_speed_kph = driving_speed_kph;
        self
    }
    
    pub fn load_fast_graph_from_disk(mut self, load_from_disk: bool) -> Self {
        self.load_fast_graph_from_disk = load_from_disk;
        self
    }
    
    pub fn build(mut self) -> Sim<Uniform> {
        self.global_params.walking_speed = self.walking_speed_kph * 1000.0 * 24.0 / self.global_params.time_steps_per_day as f32;
        self.global_params.cycling_speed = self.cycling_speed_kph * 1000.0 * 24.0 / self.global_params.time_steps_per_day as f32;
        self.global_params.driving_speed = self.driving_speed_kph * 1000.0 * 24.0 / self.global_params.time_steps_per_day as f32;
        Sim::new(self.synthetic_environment_dir, self.model_name, self.load_fast_graph_from_disk, self.global_params)
    }
}
