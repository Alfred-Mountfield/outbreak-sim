use crate::flatbuffer::Vec2;
use crate::disease::{MixingStrategy, Uniform, DiseaseStatus};
use crate::agents::Agents;
use rayon::prelude::IntoParallelRefIterator;
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

    pub fn update(&self, agents: &mut Agents) {
        // TODO create a vec of &mut DiseaseStatus, then re-order? and slice into containers

        let mut_refs = agents.disease_statuses.iter_mut()
            .map(|mut status| status)
            .collect::<Vec<&mut DiseaseStatus>>();

        let mut agents_to_containers: Vec<u32> = Vec::with_capacity(mut_refs.len());
        self.elements.iter().enumerate().for_each(|(container_idx, container)| {
            container.inhabitants.iter()
                .for_each(|&agent_idx| agents_to_containers[agent_idx as usize] = (container_idx as u32))
        });

        // par_sort_unstable_by
        let mut sorted_disease_statuses: Vec<(usize, &mut DiseaseStatus)> = (0..mut_refs.len()).zip(mut_refs.into_iter()).collect();
        sorted_disease_statuses.sort_by(|a, b|
                Ord::cmp(&agents_to_containers[a.0], &agents_to_containers[b.0])
            );

        self.elements.iter().for_each(|container| {
            let mut rng = ThreadRng::default();

            container.mixing_strategy.handle_transmission(agents, container.inhabitants.as_slice(), &mut rng)
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

