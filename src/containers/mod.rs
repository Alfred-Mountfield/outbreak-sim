use rand::rngs::ThreadRng;

use crate::agents::Agents;
use crate::disease::{DiseaseStatus, MixingStrategy, Uniform};
use crate::flatbuffer::Vec2;
use crate::shared::types::TimeStep;

/// A Spatial Area where agents spend time and mix
pub struct Container<M: MixingStrategy> {
    pub pos: Vec2,
    pub inhabitants: Vec<u32>,
    mixing_strategy: M,
    last_update: TimeStep,
}

impl<M: MixingStrategy> Container<M> {
    fn update(&mut self, agents: &mut Agents, time_step: TimeStep) {
        let start = DiseaseStatusPointer(agents.disease_statuses.as_mut_ptr());
        let mut mut_refs = self.inhabitants.iter().map(|&idx| {
            // Inspired by (taken from) https://stackoverflow.com/a/56009251/14687716
            // As the reasoning in the post explains, this relies on ensuring indices are unique and retrieving the
            // raw pointer avoids aliasing. A person should only be in one container at a time and the vec doesn't
            // get re-ordered so the indices are always unique
            // TODO Find a way to *ensure* this stays true in the code
            unsafe { &mut *start.0.add(idx as usize) }
        }).collect::<Vec<&mut DiseaseStatus>>();

        let time_steps_since_update = time_step - self.last_update;
        self.mixing_strategy.handle_transmission(mut_refs.as_mut_slice(), &mut ThreadRng::default(), time_steps_since_update);

        self.last_update = time_step;
    }
}

pub struct Containers<M: MixingStrategy> {
    elements: Vec<Container<M>>,
    num_households: u32,
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
    pub fn remove_inhabitant(&mut self, container_idx: u64, agent_idx: u32, time_step: TimeStep, agents: &mut Agents) {
        let container = self.elements.get_mut(container_idx as usize).unwrap();
        if time_step > container.last_update {
            container.update(agents, time_step);
        }
        container.inhabitants.swap_remove(
            container.inhabitants.iter().position(|idx| *idx == agent_idx).expect("Couldn't find given agent index in container")
        );
    }

    #[inline]
    pub(crate) fn push_inhabitant_no_update(&mut self, container_idx: u64, agent_idx: u32) {
        let container = self.elements.get_mut(container_idx as usize).unwrap();
        container.inhabitants.push(agent_idx);
    }

    #[inline]
    pub fn push_inhabitant(&mut self, container_idx: u64, agent_idx: u32, time_step: TimeStep, agents: &mut Agents) {
        let container = self.elements.get_mut(container_idx as usize).unwrap();
        if time_step > container.last_update {
            container.update(agents, time_step);
        }
        container.inhabitants.push(agent_idx);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.elements.is_empty() }
}

impl Containers<Uniform> {
    // TODO Investigate options to avoid ownership and duplication of mixing strategy, maybe use an enum or callback for mixing_strategy type to avoid needing lifetime params
    pub fn new(household_positions: &[Vec2], workplace_positions: &[Vec2], mixing_strategy: Uniform) -> Self {
        let containers = household_positions.iter().chain(workplace_positions).map(|pos| {
            Container {
                pos: *pos,
                inhabitants: Vec::new(),
                mixing_strategy: mixing_strategy.clone(),
                last_update: 0,
            }
        }).collect();

        Self {
            elements: containers,
            num_households: household_positions.len() as u32,
        }
    }
}

