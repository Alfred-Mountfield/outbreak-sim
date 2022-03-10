# Outbreak-Sim

A large-scale Epidemiological Agent-Based model.

[![DOI](https://zenodo.org/badge/317508446.svg)](https://zenodo.org/badge/latestdoi/317508446)

## Repository Layout

### Synthetic Environment Generator and Reporting Utilities ([.\python](python))

* [Synthetic Environment Generator](python/synthetic_environments) - A series of Python notebooks dedicated to
  the task of generating Synthetic Environments, the required input for Outbreak-Sim, which defines the starting state
  of the simulation, and contains components like agent, household, and workplace positions, agent attributes such as
  age and workplace allocations, etc.

* [Report Analysis](python/report_analysis) - A series of Python notebooks for parsing and analysing the
  report files created by simulation runs of Outbreak-Sim.

*Note: the rest of this README is dedicated to Outbreak-Sim, the Rust simulation runner, see the above README's for
detailed information on the other processes and components*

### Synthetic Environment Schema ([.\schema\model.fbs](schema/model.fbs))

A Flatbuffers schema describing the format of a Synthetic Environment data file. The schema is used for generating code
with the `./codegen.sh` script.

### Outbreak-Sim

The runner for the epidemiological simulations. Consists of a rust library, `outbreak_sim`, an example binary `main.rs` 
and a set of benchmarks in [.\benches](benches).

## Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* [FlatBuffer Compiler] (https://google.github.io/flatbuffers/flatbuffers_guide_building.html) that's accessible on the
  path as `flatc`

## Setup

*Note: Development has been tested on Windows using Windows Subsystem for Linux (WSL), other OS's and combinations may
work but are not supported.*

* run [`.\codegen.sh`](codegen.sh) to generate code using Flatbuffers from the schema found in
  [`.\schema\model.fbs`](schema/model.fbs)
* If not using one of the pre-supplied synthetic environment files found
  within [`.\python\synthetic_environments\examples`](python/synthetic_environments/examples)
  then follow the instructions within the [respective README](python/synthetic_environments/README.md) to generate a new
  one.

## Usage

* Modify or use [`.\src\main.rs`](src/main.rs) as an example in how to use the library. The example binary includes a
  rudimentary form of visualising the simulation as it runs. This will in the future be moved to a better implementation
  and made optional through a feature flag.
* The following are key parameters to be modified (inputting parameters is a work-in-progress and intended to be less
  involved and more explicit in the future):
    * The parameters of `outbreak_sim::Sim::new` require:
        * a path to the directory containing the synthetic environment file
        * the name of the file (without file-extension)
    * `outbreak_sim::reporting::intialise_reporting_files` requires, as a parameter, a path to a directory to write
      reports to
    * `outbreak_sim::Sim::new` currently has `transmission_chance` hard-coded in the function body. This should be
      modified as needed, however in future-development the `MixingStrategy` implementation will be continued, and this
      will move to a sensible place.
    * `outbreak_sim::shared` has the following global variables which are hard-coded and will be updated similarly as
      described above:
        * `SEED_INFECTION_CHANCE`: The chance of an agent being infected at the start of the simulation
        * `TIME_STEPS_PER_DAY`:
          The number of simulation time-steps in each day of in-simulation time. When containers in public transport
          routing is implemented this will need to be 1440 (a time-step being equivalent to a minute)
          whenever the feature is turned on. Currently this can be safely changed to a smaller number to speed up
          simulation speed.
