use std::collections::VecDeque;
use crate::activities::get_next_activity;

pub type ActivityIndex = VecDeque<Vec<Activity>>;

trait Update {
    fn update(&mut self, time_step: u16);
}

impl Update for ActivityIndex {
    fn update(&mut self, time_step: u16) {
        if let Some(mut activities) = self.pop_front() {
            activities.drain(..).for_each(|activity| {
                if let Some(next_activity) = get_next_activity(activity.agent_idx, time_step) {
                    self.get_mut_or_grow((next_activity.end_timestep - time_step) as usize).unwrap().push(next_activity);
                }
            });
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Activity {
    pub agent_idx: u32,
    // start_timestep: u16,
    pub end_timestep: u16,
}

trait VecDequeMutExt<T: Default> {
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

