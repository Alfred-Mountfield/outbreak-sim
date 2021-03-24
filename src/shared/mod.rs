use std::sync::atomic::{AtomicU16, Ordering};

pub static SIMULATION_LENGTH_IN_DAYS: AtomicU16 = AtomicU16::new(30);
pub static TIME_STEPS_PER_DAY: AtomicU16 = AtomicU16::new(48); // every minute
// TODO make mutable, OnceCell?, also make it easier to input like kph
pub static WALKING_SPEED: f32 = 80.5; //meters per minute
pub static CYCLING_SPEED: f32 = 350.0; // meters per minute
pub static DRIVING_SPEED: f32 = 700.0; // meters per minute

pub fn set_up_global_params(sim_length: u16, time_steps_per_day: u16) {
    SIMULATION_LENGTH_IN_DAYS.store(sim_length, Ordering::Relaxed);
    TIME_STEPS_PER_DAY.store(time_steps_per_day, Ordering::Relaxed);
}