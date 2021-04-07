use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind};
use std::path::PathBuf;

use csv::Writer;
use serde::{Deserialize, Serialize};

use crate::agents::Agents;
use crate::disease::State;
use crate::shared::types::TimeStep;
use crate::shared::TIME_STEPS_PER_DAY;

#[derive(Serialize, Deserialize)]
struct Metric {
    time_step: TimeStep,
    num_susceptible: usize,
    num_presymptomatic: usize,
    num_infected: usize,
    num_recovered: usize,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    time_steps_per_day: u32,
}

pub fn intialise_reporting_files<P>(out_dir: P, iteration: usize, replace: bool) -> Result<Writer<File>, Box<dyn Error>>
    where P: Into<PathBuf>
{
    let mut out_path = out_dir.into();
    assert!(out_path.extension().is_none());
    out_path.push(iteration.to_string());
    if out_path.exists() { assert!(out_path.is_dir()) };

    fs::create_dir_all(&out_path)?;

    create_param_file(out_path.clone(), replace)?;
    create_report_file(out_path.clone(), replace)
}

fn create_param_file(mut param_path: PathBuf, replace: bool) -> Result<(), Box<dyn Error>> {
    param_path.push("parameters");
    param_path.set_extension("json");

    if !replace && param_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating parameters file: {}", param_path.display());
        serde_json::to_writer_pretty(&File::create(param_path)?, &Parameters {
            time_steps_per_day: TIME_STEPS_PER_DAY
        })?;
        Ok(())
    }
}

fn create_report_file(mut report_path: PathBuf, replace: bool) -> Result<Writer<File>, Box<dyn Error>> {
    report_path.push("report");
    report_path.set_extension("csv");

    if !replace && report_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating report file: {}", report_path.display());
        Writer::from_path(report_path).map_err(|e| e.into())
    }
}

pub fn add_metric(report_writer: &mut Writer<File>, time_step: TimeStep, agents: &Agents) -> Result<(), io::Error> {
    let (mut num_susceptible, mut num_presymptomatic, mut num_infected, mut num_recovered) = (0, 0, 0, 0);
    for status in &agents.disease_statuses {
        match status.state {
            State::Susceptible => { num_susceptible += 1 }
            State::Presymptomatic => { num_presymptomatic += 1 }
            State::Infectious => { num_infected += 1 }
            State::Recovered => { num_recovered += 1 }
        }
    }

    let metric = Metric {
        time_step,
        num_susceptible,
        num_presymptomatic,
        num_infected,
        num_recovered
    };

    report_writer.serialize(metric)?;
    report_writer.flush()?;
    Ok(())
}