use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::sync::Mutex;

use rand::rngs::ThreadRng;
use rayon::prelude::*;

use crate::agents::Agents;
use crate::disease::{DiseaseStatus, MixingStrategy, Uniform};
use crate::flatbuffer::Vec2;

/// A Spatial Area where agents spend time and mix
pub struct Container<M: MixingStrategy> {
    pub pos: Vec2,
    pub inhabitants: Vec<u32>,
    mixing_strategy: M,
}

pub struct Containers<M: MixingStrategy> {
    elements: Vec<Container<M>>,
    num_households: u32,
    num_workplaces: u32,
}

struct DiseaseStatusPointer(*mut DiseaseStatus);

unsafe impl Send for DiseaseStatusPointer {}

unsafe impl Sync for DiseaseStatusPointer {}

impl<M: MixingStrategy> Containers<M> {
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

    #[inline]
    pub fn remove_inhabitant(&mut self, container_idx: u64, agent_idx: u32) {
        let container = self.elements.get_mut(container_idx as usize).unwrap();
        container.inhabitants.swap_remove(
            container.inhabitants.iter().position(|idx| *idx == agent_idx).expect("Couldn't find given agent index in container")
        );
    }

    #[inline]
    pub fn push_inhabitant(&mut self, container_idx: u64, agent_idx: u32) {
        self.elements.get_mut(container_idx as usize).unwrap().inhabitants.push(agent_idx);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn update(&self, agents: &mut Agents) {
        let start = DiseaseStatusPointer(agents.disease_statuses.as_mut_ptr());
        let mut unique_indices = Mutex::new(BTreeSet::new());

        self.elements.par_iter().for_each(|container| {
            let mut mut_refs = container.inhabitants.iter().map(|&idx| {
                debug_assert!({
                    let mut unique_indices = unique_indices.lock().unwrap();
                    unique_indices.insert(idx)
                });
                // Inspired by (taken from) https://stackoverflow.com/a/56009251/14687716
                // As the reasoning in the post explains, this relies on ensuring indices are unique and retrieving the
                // raw pointer avoids aliasing
                unsafe { &mut *start.0.add(idx as usize) }
            }).collect::<Vec<&mut DiseaseStatus>>();

            container.mixing_strategy.handle_transmission(mut_refs.as_mut_slice(), &mut ThreadRng::default());
        })
    }
}

impl Containers<Uniform> {
    // TODO Investigate options to avoid ownership and duplication of mixing strategy, maybe use an enum or callback for mixing_strategy type to avoid needing lifetime params
    pub fn new(household_positions: &[Vec2], workplace_positions: &[Vec2], mixing_strategy: Uniform) -> Self {
        let containers = household_positions.iter().chain(workplace_positions).map(|pos| {
            Container {
                pos: pos.clone(),
                inhabitants: Vec::new(),
                mixing_strategy: mixing_strategy.clone(),
            }
        }).collect();

        Self {
            elements: containers,
            num_households: household_positions.len() as u32,
            num_workplaces: workplace_positions.len() as u32,
        }
    }
}

