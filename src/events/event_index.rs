use std::collections::VecDeque;

use crate::events::get_next_event;
use crate::events::event::Event;

pub type EventIndex = VecDeque<Vec<Event>>;

trait Update {
    fn update(&mut self, time_step: u16);
}

impl Update for EventIndex {
    fn update(&mut self, time_step: u16) {
        if let Some(mut events) = self.pop_front() {
            events.drain(..).for_each(|event| {
                if let Some(next_event) = get_next_event(event.agent_idx, time_step) {
                    self.get_mut_or_grow((next_event.end_timestep - time_step) as usize).unwrap().push(next_event);
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

