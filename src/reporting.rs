use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::Duration;

use csv::Writer;
use serde::{Deserialize, Serialize};

use crate::agents::Agents;
use crate::disease::{State};
use crate::shared::GLOBAL_PARAMS;
use crate::shared::types::TimeStep;

/// An insight into a simulation's state _during_ simulation
#[derive(Serialize, Deserialize)]
struct IntermediaryMetric {
    time_step: TimeStep,
    num_susceptible: usize,
    num_exposed: usize,
    num_infectious: usize,
    num_recovered: usize,
}

/// A description of a simulation
#[derive(Serialize, Deserialize)]
struct ConcludingMetric {
    total_time_steps: TimeStep,
    simulation_execution_time_in_secs: f64,
    synthetic_environment_path: PathBuf,
}

#[inline]
pub fn intialise_reporting_files<P>(out_dir: P, iteration: usize, replace: bool) -> Result<(Writer<File>, File), Box<dyn Error>>
    where P: Into<PathBuf>
{
    let mut out_path = out_dir.into();
    assert!(out_path.extension().is_none());
    out_path.push(iteration.to_string());
    if out_path.exists() { assert!(out_path.is_dir()) };

    fs::create_dir_all(&out_path)?;

    create_param_file(out_path.clone(), replace)?;
    let intermediary_report_writer = create_intermediary_report_file(out_path.clone(), replace)?;
    let concluding_report_file = create_concluding_report_file(out_path, replace)?;

    Ok((intermediary_report_writer, concluding_report_file))
}

#[inline]
fn create_param_file(mut param_path: PathBuf, replace: bool) -> Result<(), Box<dyn Error>> {
    param_path.push("parameters");
    param_path.set_extension("json");

    if !replace && param_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating parameters file: {}", param_path.display());
        serde_json::to_writer_pretty(&File::create(param_path)?, GLOBAL_PARAMS.get().unwrap())?;
        Ok(())
    }
}

#[inline]
fn create_intermediary_report_file(mut report_path: PathBuf, replace: bool) -> Result<Writer<File>, Box<dyn Error>> {
    report_path.push("intermediary");
    report_path.set_extension("csv");

    if !replace && report_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating intermediary report file: {}", report_path.display());
        Writer::from_path(report_path).map_err(|e| e.into())
    }
}

#[inline]
fn create_concluding_report_file(mut report_path: PathBuf, replace: bool) -> Result<File, Box<dyn Error>> {
    report_path.push("concluding");
    report_path.set_extension("json");

    if !replace && report_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating concluding report file: {}", report_path.display());
        File::create(report_path).map_err(|e| e.into())
    }
}

#[inline]
pub fn write_intermediary_metric(report_writer: &mut Writer<File>, time_step: TimeStep, agents: &Agents) -> Result<(), io::Error> {
    let (mut num_susceptible, mut num_exposed, mut num_infectious, mut num_recovered) = (0, 0, 0, 0);
    for status in &agents.disease_statuses {
        match status.state {
            State::Susceptible => { num_susceptible += 1 }
            State::Exposed => { num_exposed += 1 }
            State::Infectious => { num_infectious += 1 }
            State::Recovered => { num_recovered += 1 }
        }
    }

    let metric = IntermediaryMetric {
        time_step,
        num_susceptible,
        num_exposed,
        num_infectious,
        num_recovered,
    };

    report_writer.serialize(metric)?;
    report_writer.flush()?;
    Ok(())
}

#[inline]
pub fn write_concluding_metrics(report_file: &File, time_step: TimeStep, exec_time: Duration,
                                synthetic_environment_path: PathBuf) -> Result<(), io::Error> {
    let metric = ConcludingMetric {
        total_time_steps: time_step,
        simulation_execution_time_in_secs: exec_time.as_secs_f64(),
        synthetic_environment_path: synthetic_environment_path.canonicalize()?,
    };
    serde_json::to_writer_pretty(report_file, &metric)?;
    Ok(())
}