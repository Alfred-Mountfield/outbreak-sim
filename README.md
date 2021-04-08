# Outbreak-Sim

A large-scale Epidemiological Agent-Based model.


## Repository Layout
### Synthetic Environment Generator and Reporting Utilities ([.\python](python/README.md))

* [Synthetic Environment Generator](python/synthetic_environments/README.md) - A series of Python notebooks dedicated to 
  the task of generating Synthetic Environments, the required input for Outbreak-Sim, which defines the starting state 
  of the simulation, and contains components like agent, household, and workplace positions, agent attributes such as 
  age and workplace allocations, etc. 

* [Report Analysis](python/report_analysis/README.md) - A series of Python notebooks for parsing and analysing the 
report files created by simulation runs of Outbreak-Sim.

*Note: the rest of this README is dedicated to Outbreak-Sim, the Rust simulation runner, see the above README's for 
detailed information on the other processes and components*

### Synthetic Environment Schema ([.\schema\model.fbs](schema/model.fbs))

A Flatbuffers schema describing the format of a Synthetic Environment data file. The schema is used for generating code 
with the `./codegen.sh` script.

### Outbreak-Sim
The runner for the epidemiological simulations. Consists of a rust library, `outbreak_sim` and an example binary 
`main.rs`.

## Requirements
 * [Rust](https://www.rust-lang.org/tools/install)
 * [FlatBuffer Compiler] (https://google.github.io/flatbuffers/flatbuffers_guide_building.html) that's accessible on 
   the path as `flatc`

## Setup

*Note: Development has been tested on Windows using Windows Subsystem for Linux (WSL), other OS's and combinations may 
work but are not supported.*

  * run [`.\codegen.sh`](codegen.sh) to generate code using Flatbuffers from the schema found in 
    [`.\schema\model.fbs`](schema/model.fbs)
  * If not using one of the pre-supplied synthetic environment files found within [`.\python\synthetic_environments\examples`](python/synthetic_environments/examples) 
    then follow the instructions within the [respective README](python/synthetic_environments/README.md) to generate a new one.

## Usage

WIP
