use std::path::Path;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::disease;
use crate::disease::DiseaseStatus;
use crate::shared::Coord;

pub mod position;

pub struct Agents {
    pub num_agents: u64,
    pub positions: Vec<Coord>,
    pub disease_statuses: Vec<DiseaseStatus>,
    rng: StdRng
}

impl Agents {
    pub fn new(raster_path: &Path) -> Agents {
        let mut rng = StdRng::seed_from_u64(32);
        let positions = position::construct_pos_array_from_txt(raster_path);
        let num_agents = positions.len() as u64;
        Agents {
            num_agents,
            positions,
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