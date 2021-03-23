use rand::{thread_rng, Rng};

use crate::events::event_index::{EventIndex, VecDequeMutExt};
use crate::agents::Agents;
use crate::shared::TIME_STEPS_PER_DAY;
use std::sync::atomic::Ordering;
use crate::events::event::{Event, EventType};

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
                    end_timestep: tmp_weighted_commute_time(&mut rng),
                    event_type: EventType::EnterContainer(container_idx.unwrap())
                }
            })
            .for_each(|event| {
                event_index.get_mut_or_grow(event.agent_idx as usize).unwrap().push(event);
            });

        Events {
            event_index
        }
    }
}

// hacky, unsupported, attempt to get some form of distributed commuting
#[inline]
fn tmp_weighted_commute_time<R>(rng: &mut R) -> u16
    where R: Rng + ?Sized
{
    let time_steps_per_hour: u16 = TIME_STEPS_PER_DAY.load(Ordering::Relaxed) / 24;
    // commute start times range from 4am to 11am
    let earliest = 4 * time_steps_per_hour;
    let time_steps_range = 7 * time_steps_per_hour;

    earliest + (rng.gen::<f32>() * time_steps_range as f32) as u16
}

// TODO Event Generation
#[inline]
fn get_next_event(agent_idx: u32, time_step: u16) -> Option<Event> {
    Some(
        Event {
            agent_idx,
            end_timestep: time_step + 12,
            event_type: unimplemented!()
        }
    )
}