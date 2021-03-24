use rand::{thread_rng, Rng};

use crate::events::event_index::{EventIndex, VecDequeMutExt, Update};
use crate::agents::Agents;
use crate::shared::TIME_STEPS_PER_DAY;
use std::sync::atomic::Ordering;
use crate::events::event::{Event, EventType};
use crate::disease::MixingStrategy;
use crate::pois::Containers;

mod event;
mod event_index;

pub struct Events {
    event_index: EventIndex,
}

impl Events {
    pub fn new(agents: &mut Agents) -> Self {
        let mut rng = thread_rng();
        let mut event_index = EventIndex::default();

        agents.occupational_container.iter()
            .enumerate()
            .filter(|(_, &container_idx)| container_idx.is_some())
            .map(|(agent_idx, container_idx)| {
                Event {
                    agent_idx: agent_idx as u32,
                    end_time_step: tmp_weighted_commute_time(&mut rng),
                    event_type: EventType::EnterContainer(container_idx.unwrap())
                }
            })
            .for_each(|event| {
                event_index.get_mut_or_grow(event.end_time_step as usize).unwrap().push(event);
            });

        Events {
            event_index
        }
    }

    pub fn update<M>(&mut self, time_step: u32, agents: &Agents, containers: &mut Containers<M>) where M: MixingStrategy {
        self.event_index.update(time_step, agents, containers);
    }
}

// hacky, unsupported, attempt to get some form of distributed commuting
#[inline]
fn tmp_weighted_commute_time<R>(rng: &mut R) -> u32
    where R: Rng + ?Sized
{
    let time_steps_per_hour: u32 = (TIME_STEPS_PER_DAY.load(Ordering::Relaxed) / 24) as u32;
    // commute start times range from 4am to 11am
    let earliest = 4 * time_steps_per_hour;
    let time_steps_range = 7 * time_steps_per_hour;

    earliest + (rng.gen::<f32>() * time_steps_range as f32) as u32
}