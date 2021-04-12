use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::disease;
use crate::disease::{DiseaseStatus, MixingStrategy};
use crate::flatbuffer::{Model};
use crate::containers::Containers;
use nonmax::NonMaxU64;

#[derive(Clone)]
pub struct Agents {
    pub num_agents: u32,
    pub household_container: Vec<u64>,
    pub occupational_container: Vec<Option<NonMaxU64>>,
    // workplace or school
    pub disease_statuses: Vec<DiseaseStatus>,
}

impl Agents {
    pub fn new<M>(model: &Model, containers: &mut Containers<M>) -> Agents
        where M: MixingStrategy + Send + Sync
    {
        let household_indices = model.agents().household_index();
        let workplace_indices = model.agents().workplace_index();

        let mut rng = StdRng::seed_from_u64(32);
        let num_agents = household_indices.len() as u32;

        let (household_container, workplace_container) = household_indices.iter().zip(workplace_indices.iter())
            .enumerate().map(|(agent_idx, (household_idx, workplace_idx))| {
            let household_container_idx = containers.get_household_idx(household_idx);
            let workplace_container_idx = match workplace_idx {
                u32::MAX => {
                    // TODO Update this, super hacky right now as people without workplaces don't have Event schedules so need to manually be placed in a container
                    containers.push_inhabitant_no_update(household_container_idx, agent_idx as u32);
                    None
                }
                _ => { NonMaxU64::new(containers.get_workplace_idx(workplace_idx)) }
            };
            (household_container_idx, workplace_container_idx)
        }).unzip();

        Agents {
            num_agents,
            household_container,
            occupational_container: workplace_container,
            disease_statuses: disease::construct_disease_status_array(num_agents, &mut rng),
        }
    }
}