use std::collections::VecDeque;

use crate::events::event::{Event};
use crate::disease::MixingStrategy;
use crate::pois::Containers;
use crate::agents::Agents;

pub type EventIndex = VecDeque<Vec<Event>>;

pub trait Update {
    fn update<M>(&mut self, time_step: u32, agents: &Agents, containers: &mut Containers<M>) where M: MixingStrategy;
}

impl Update for EventIndex {
    fn update<M>(&mut self, time_step: u32, agents: &Agents, containers: &mut Containers<M>) where M: MixingStrategy {
        if let Some(mut events) = self.pop_front() {
            events.drain(..).for_each(|event| {
                if let Some(next_event) = event.handle(agents, containers) {
                    self.get_mut_or_grow((next_event.end_time_step - time_step) as usize).unwrap().push(next_event);
                }
            });
        }
    }
}

pub trait VecDequeMutExt<T: Default> {
    fn get_or_grow(&mut self, index: usize) -> Option<&T>;
    fn get_mut_or_grow(&mut self, index: usize) -> Option<&mut T>;
}

impl<T: Default> VecDequeMutExt<T> for VecDeque<T> {
    fn get_or_grow(&mut self, index: usize) -> Option<&T> {
        if index >= self.len() {
            self.resize_with(index + 1, Default::default);
        }
        self.get(index)
    }
    fn get_mut_or_grow(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            self.resize_with(index + 1, Default::default);
        }
        self.get_mut(index)
    }
}

