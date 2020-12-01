use rand::rngs::StdRng;
use rand::{SeedableRng};
use position::Coord;
use crate::disease::DiseaseStatus;

pub mod disease;
pub mod position;

pub struct Agents {
    pub num_agents: u64,
    pub positions: Vec<Coord>,
    pub disease_statuses: Vec<DiseaseStatus>,
    rng: StdRng
}

impl Agents {
    pub fn new(num_agents: u64) -> Agents {
        let mut rng = StdRng::seed_from_u64(32);
        Agents {
            num_agents,
            positions: position::construct_pos_array(num_agents, &mut rng),
            disease_statuses: disease::construct_disease_status_array(num_agents, &mut rng),
            rng
        }
    }

    pub fn update(&mut self) {
        for coord in self.positions.iter_mut() {
            coord.update(&mut self.rng)
        }
        for i in 0..self.disease_statuses.len() {
            DiseaseStatus::update(i, &mut self.disease_statuses, &self.positions);
        }
    }
}

