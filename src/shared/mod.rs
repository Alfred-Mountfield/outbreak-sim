use std::sync::atomic::{AtomicU16, Ordering};

use crate::shared::types::TimeStep;

pub mod types;

pub static SIMULATION_LENGTH_IN_DAYS: AtomicU16 = AtomicU16::new(30);

// TODO make mutable, OnceCell?, also make it easier to input like kph
pub static TIME_STEPS_PER_DAY: TimeStep =  360; // every minute
// pub static TIME_STEPS_PER_DAY: TimeStep =  48; // every half an hour
pub static WALKING_SPEED: f32 = (5.0 * 24.0) / TIME_STEPS_PER_DAY as f32; // kph converted to per timestep
pub static CYCLING_SPEED: f32 = (23.5 * 24.0) / TIME_STEPS_PER_DAY as f32; // kph converted to per timestep
pub static DRIVING_SPEED: f32 = (60.0 * 24.0) / TIME_STEPS_PER_DAY as f32; // kph converted to per timestep

pub fn set_up_global_params(sim_length: u16) {
    SIMULATION_LENGTH_IN_DAYS.store(sim_length, Ordering::Relaxed);
}
