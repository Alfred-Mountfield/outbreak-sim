use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::disease;
use crate::disease::DiseaseStatus;
use crate::flatbuffer::Vec2;
use flatbuffers::Vector;

pub mod position;

pub struct Agents {
    pub num_agents: u32,
    pub positions: Vec<Vec2>,
    // pub household_container: Vec<u32>,
    // pub occupational_container: Vec<u32>, // workplace or school
    pub disease_statuses: Vec<DiseaseStatus>,
    rng: StdRng
}

impl Agents {
    pub fn new(agent_households: Vector<u32>, household_positions: &[Vec2]) -> Agents {
        let mut rng = StdRng::seed_from_u64(32);
        let num_agents = agent_households.len() as u32;

        let positions = agent_households.iter().filter_map(|idx| {
            household_positions.get(idx as usize)
        }).copied().collect();

        Agents {
            num_agents,
            positions,
            disease_statuses: disease::construct_disease_status_array(num_agents, &mut rng),
            rng
        }
    }

    pub fn update(&mut self) {
        // for coord in self.positions.iter_mut() {
        //     coord.update(&mut self.rng)
        // }
        for i in 0..self.disease_statuses.len() {
            DiseaseStatus::update(i, &mut self.disease_statuses, &self.positions);
        }
    }
}