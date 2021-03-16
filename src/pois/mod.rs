use crate::flatbuffer::Vec2;
use crate::disease::{MixingStrategy, Uniform, DiseaseStatus};
use crate::agents::Agents;
use rayon::prelude::*;
use rand::rngs::ThreadRng;

/// A Spatial Area where agents spend time and mix
pub struct Container<'a, M> where M: MixingStrategy {
    pos: Vec2,
    inhabitants: Vec<u32>,
    mixing_strategy: &'a M,
}

pub struct Containers<'a, M> where M: MixingStrategy {
    elements: Vec<Container<'a, M>>,
    mixing_strategies: Vec<&'a M>,
    num_households: u32,
    num_workplaces: u32,
}

impl<'a, M> Containers<'a, M> where M: MixingStrategy {
    pub fn get(&self, idx: u64) -> Option<&Container<M>> {
        self.elements.get(idx as usize)
    }

    #[inline]
    pub fn get_household(&self, household_ind: u32) -> Option<&Container<M>> {
        self.elements.get(household_ind as usize)
    }

    #[inline]
    pub fn get_household_idx(&self, household_ind: u32) -> u64 {
        household_ind as u64
    }

    #[inline]
    pub fn get_workplace(&self, workplace_ind: u32) -> Option<&Container<M>> {
        self.elements.get(self.num_households as usize + workplace_ind as usize)
    }

    #[inline]
    pub fn get_workplace_idx(&self, workplace_ind: u32) -> u64 {
        self.num_households as u64 + workplace_ind as u64
    }

    pub fn push_inhabitant(&mut self, container_idx: u64, agent_idx: u32) {
        self.elements.get_mut(container_idx as usize).unwrap().inhabitants.push(agent_idx);
    }

    pub fn update(&self, agents: &mut Agents) {
        // TODO investigate https://stackoverflow.com/questions/55939552/simultaneous-mutable-access-to-arbitrary-indices-of-a-large-vector-that-are-guar
        let mut_refs = agents.disease_statuses.iter_mut()
            .collect::<Vec<&mut DiseaseStatus>>();

        let mut agents_to_containers: Vec<u32> = vec![u32::MAX; mut_refs.len()];
        self.elements.iter().enumerate().for_each(|(container_idx, container)| {
            container.inhabitants.iter()
                .for_each(|&agent_idx| agents_to_containers[agent_idx as usize] = (container_idx as u32))
        });

        // par_sort_unstable_by
        let mut sorted_disease_statuses: Vec<&mut DiseaseStatus> = {
            let mut enumerated_statuses: Vec<(usize, &mut DiseaseStatus)> = (0..mut_refs.len()).zip(mut_refs.into_iter()).collect();
            enumerated_statuses.par_sort_unstable_by(|a, b| {
                let a_idx = agents_to_containers.get(a.0).unwrap_or(&u32::MAX);
                let b_idx = agents_to_containers.get(b.0).unwrap_or(&u32::MAX);
                Ord::cmp(a_idx, b_idx)
            });
            enumerated_statuses.into_par_iter().map(|(idx, status)| status).collect()
        };

        let mut tail = sorted_disease_statuses.as_mut_slice();

        let mut container_to_disease_statuses: Vec<&mut [&mut DiseaseStatus]> = Vec::with_capacity(self.elements.len());

        for container in self.elements.iter() {
            let idx = container.inhabitants.len();
            let (left, right) = tail.split_at_mut(idx);
            tail = right;
            container_to_disease_statuses.push(left);
        }

        container_to_disease_statuses.into_par_iter().enumerate().for_each(|(idx, disease_statuses)| {
            let mut rng = ThreadRng::default();

            self.elements[idx].mixing_strategy.handle_transmission(disease_statuses, &mut rng)
        });
    }
}

impl<'a> Containers<'a, Uniform> {
    // TODO Investigate using arc to avoid having to pass in mixing_strategy
    pub fn new(household_positions: &[Vec2], workplace_positions: &[Vec2], mixing_strategy: &'a Uniform) -> Containers<'a, Uniform> {
        let containers = household_positions.iter().chain(workplace_positions).map(|pos| {
            return Container {
                pos: pos.clone(),
                inhabitants: Vec::new(),
                mixing_strategy,
            };
        }).collect();

        return Containers {
            elements: containers,
            mixing_strategies: vec![mixing_strategy],
            num_households: household_positions.len() as u32,
            num_workplaces: workplace_positions.len() as u32,
        };
    }
}

