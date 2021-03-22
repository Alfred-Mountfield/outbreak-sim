use crate::activities::activity_index::{ActivityIndex, Activity};
use crate::shared::TIME_STEPS_PER_DAY;

mod activity_index;

pub struct Activities {
    activity_index: ActivityIndex,
}

// TODO Activity Generation
fn get_next_activity(agent_idx: u32, time_step: u16) -> Option<Activity> {
    Some(
        Activity {
            agent_idx,
            end_timestep: time_step + 12
        }
    )

}