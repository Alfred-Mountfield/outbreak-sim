use crate::shared::types::TimeStep;

pub mod types;

// TODO make mutable, OnceCell?, also make it easier to input like kph
pub static SIMULATION_LENGTH_IN_DAYS: Option<u32> = Some(730);

// pub static TIME_STEPS_PER_DAY: TimeStep =  1440; // every minute
pub static TIME_STEPS_PER_DAY: TimeStep = 48; // every half an hour

pub static SEED_INFECTION_CHANCE: f32 = 0.0001;

// kph converted to per timestep
pub static WALKING_SPEED: f32 = (5.0 * 24.0) / TIME_STEPS_PER_DAY as f32;
pub static CYCLING_SPEED: f32 = (23.5 * 24.0) / TIME_STEPS_PER_DAY as f32;
pub static DRIVING_SPEED: f32 = (60.0 * 24.0) / TIME_STEPS_PER_DAY as f32;

// TODO
// pub fn set_up_global_params(sim_length: u32) {
// }
