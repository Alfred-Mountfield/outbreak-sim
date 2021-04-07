# Outbreak-Sim

A large-scale Epidemiological Agent-Based model.


## Repository Layout
### Synthetic Environment Generator and Reporting Utilities ([./python](python/README.md))

* [Synthetic Environment Generator](python/synthetic_environments/README.md) - A series of Python notebooks dedicated to 
  the task of generating Synthetic Environments, the required input for Outbreak-Sim. 

* [Report Analysis](python/report_analysis/README.md) - A series of Python notebooks for parsing and analysing the 
report files created by simulation runs of Outbreak-Sim.

*Note: the rest of this README is dedicated to Outbreak-Sim, the Rust simulation runner, see the above README's for 
detailed information on the other processes and components*


### Outbreak-Sim
The runner for the epidemiological simulations. Consists of a rust library, `outbreak_sim` and an example binary 
`main.rs`.

## Requirements

 * [Rust](https://www.rust-lang.org/tools/install)

## Setup

WIP

* run `.\codegen.sh` to generate code using Flatbuffers from the schema found in `.\schema\model.fbs`

## Usage

WIP
