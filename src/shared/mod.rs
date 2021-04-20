use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::shared::types::TimeStep;

pub mod types;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSimParams {
    pub time_steps_per_day: u32,
    pub sim_length_days: Option<u32>,
    pub seed_infection_chance: f32,
    /// spatial unit per time-step
    pub walking_speed: f32,
    /// spatial unit per time-step
    pub cycling_speed: f32,
    /// spatial unit per time-step
    pub driving_speed: f32,
}

impl Default for GlobalSimParams {
    fn default() -> Self {
        GlobalSimParams {
            time_steps_per_day: 48, // every half an hour
            sim_length_days: Some(60),
            seed_infection_chance: 0.01,
            walking_speed: 5.0 * 1000.0 * 24.0 / 48.0,
            cycling_speed: 23.5 * 1000.0 * 24.0 / 48.0,
            driving_speed: 60.0 * 1000.0 * 24.0 / 48.0,
        }
    }
}

pub static GLOBAL_PARAMS: OnceCell<GlobalSimParams> = OnceCell::new();

#[inline]
pub fn get_simulation_length_in_days() -> Option<u32> {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().sim_length_days
    }
}

#[inline]
pub fn get_time_steps_per_day() -> TimeStep {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().time_steps_per_day
    }
}

#[inline]
pub fn get_seed_infection_chance() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().seed_infection_chance
    }
}

#[inline]
pub fn get_walking_speed() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().walking_speed
    }
}

#[inline]
pub fn get_cycling_speed() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().cycling_speed
    }
}

#[inline]
pub fn get_driving_speed() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().driving_speed
    }
}

pub fn set_up_global_params(params: GlobalSimParams) {
    if let Err(e) = GLOBAL_PARAMS.set(params) {
        eprintln!("global parameters had already been set");
        panic!("{:?}", e);
    }
}
