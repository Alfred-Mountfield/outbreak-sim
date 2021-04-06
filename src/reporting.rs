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

#[derive(Serialize, Deserialize)]
struct Metric {
    time_step: TimeStep,
    num_susceptible: usize,
    num_infected: usize,
    num_recovered: usize,
}

pub fn create_report_file<P>(out_dir: P, iteration: usize, replace: bool) -> Result<Writer<File>, Box<dyn Error>>
    where P: Into<PathBuf>
{
    let mut out_path = out_dir.into();
    assert!(out_path.extension().is_none());
    if out_path.exists() { assert!(out_path.is_dir()) };

    fs::create_dir_all(&out_path)?;

    out_path.push(iteration.to_string());
    out_path.set_extension("csv");

    if !replace && out_path.exists() {
        Err(io::Error::from(ErrorKind::AlreadyExists).into())
    } else {
        println!("Creating report file: {}", out_path.display());
        Writer::from_path(out_path).map_err(|e| e.into())
    }
}

pub fn add_metric(report_writer: &mut Writer<File>, time_step: TimeStep, agents: &Agents) -> Result<(), io::Error> {
    let (mut num_susceptible, mut num_infected, mut num_recovered) = (0, 0, 0);
    for status in &agents.disease_statuses {
        match status.state {
            State::Susceptible => { num_susceptible += 1 }
            State::Infectious => { num_infected += 1 }
            State::Recovered => { num_recovered += 1 }
        }
    }

    let metric = Metric {
        time_step,
        num_susceptible,
        num_infected,
        num_recovered
    };

    report_writer.serialize(metric)?;
    report_writer.flush()?;
    Ok(())
}