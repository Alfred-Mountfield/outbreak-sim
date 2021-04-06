use std::sync::atomic::{AtomicU16, Ordering};

use crate::shared::types::TimeStep;

pub mod types;

pub static SIMULATION_LENGTH_IN_DAYS: AtomicU16 = AtomicU16::new(30);

// TODO make mutable, OnceCell?, also make it easier to input like kph
pub static TIME_STEPS_PER_DAY: TimeStep =  1440; // every minute
pub static WALKING_SPEED: f32 = 80.5; //meters per minute
pub static CYCLING_SPEED: f32 = 350.0; // meters per minute
pub static DRIVING_SPEED: f32 = 700.0; // meters per minute

pub fn set_up_global_params(sim_length: u16) {
    SIMULATION_LENGTH_IN_DAYS.store(sim_length, Ordering::Relaxed);
}
