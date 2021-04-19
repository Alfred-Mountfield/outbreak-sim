use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::shared::types::TimeStep;

pub mod types;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSimParams {
    pub time_steps_per_day: u32,
    pub sim_length_days: Option<u32>,
    pub seed_infection_chance: f32,
    pub walking_speed_kph: f32,
    pub cycling_speed_kph: f32,
    pub driving_speed_kph: f32,
}

impl Default for GlobalSimParams {
    fn default() -> Self {
        GlobalSimParams {
            time_steps_per_day: 48, // every half an hour
            sim_length_days: Some(60),
            seed_infection_chance: 0.001,
            walking_speed_kph: 5.0,
            cycling_speed_kph: 23.5,
            driving_speed_kph: 60.0,
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
        GLOBAL_PARAMS.get_unchecked().walking_speed_kph
    }
}

#[inline]
pub fn get_cycling_speed() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().cycling_speed_kph
    }
}

#[inline]
pub fn get_driving_speed() -> f32 {
    unsafe {
        GLOBAL_PARAMS.get_unchecked().driving_speed_kph
    }
}

pub fn set_up_global_params(params: GlobalSimParams) {
    if let Err(e) = GLOBAL_PARAMS.set(params) {
        eprintln!("global parameters had already been set");
        panic!("{:?}", e);
    }
}
