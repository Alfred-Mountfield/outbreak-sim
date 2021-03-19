use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::disease;
use crate::disease::{DiseaseStatus, MixingStrategy};
use crate::flatbuffer::{Model};
use crate::pois::Containers;
use nonmax::NonMaxU64;


pub struct Agents {
    pub num_agents: u32,
    pub household_container: Vec<u64>,
    pub occupational_container: Vec<Option<NonMaxU64>>,
    // workplace or school
    pub disease_statuses: Vec<DiseaseStatus>,
    rng: StdRng,
}

impl Agents {
    pub fn new<M>(model: &Model, containers: &mut Containers<M>) -> Agents
        where M: MixingStrategy + Send + Sync
    {
        let household_indices = model.agents().household_index();
        let workplace_indices = model.agents().workplace_index();

        let mut rng = StdRng::seed_from_u64(32);
        let num_agents = household_indices.len() as u32;

        let household_container = household_indices.iter().enumerate()
            .map(|(agent_idx, household_idx)| {
                let container_idx = containers.get_household_idx(household_idx);
                containers.push_inhabitant(container_idx, agent_idx as u32);
                return container_idx;
            }).collect();

        let workplace_container = workplace_indices.iter().map(|idx| {
            match idx {
                u32::MAX => { None }
                _ => NonMaxU64::new(containers.get_workplace_idx(idx))
            }
        }).collect();

        Agents {
            num_agents,
            household_container,
            occupational_container: workplace_container,
            disease_statuses: disease::construct_disease_status_array(num_agents, &mut rng),
            rng,
        }
    }
}