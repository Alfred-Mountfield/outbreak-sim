use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use structopt::StructOpt;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use outbreak_sim::disease::State;
use outbreak_sim::reporting::{intialise_reporting_files, write_concluding_metrics, write_intermediary_metric};
use outbreak_sim::shared::types::TimeStep;

const SCREEN_WIDTH: u32 = 950;
const SCREEN_HEIGHT: u32 = 950;
const WORLD_WIDTH: u32 = 500;
const WORLD_HEIGHT: u32 = 500;

mod graphics;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the directory containing the model
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The model name
    model_name: String,
    iteration: usize,
}


fn main() -> Result<(), Error> {
    let args: Cli = Cli::from_args();
    let synthetic_environment_dir = args.path.to_owned();
    let model_name = args.model_name.to_owned();

    let mut time_step: TimeStep = 0;
    let time_steps_per_day: u32 = 48;
    let iterations_per_render: u32 = 30;

    let mut sim = outbreak_sim::SimBuilder::new(&synthetic_environment_dir, &model_name)
        .load_fast_graph_from_disk(true)
        .time_steps_per_day(time_steps_per_day)
        .build();


    let (mut intermediary_report_writer, concluding_report_file) = intialise_reporting_files("reports/".to_owned() + &model_name, args.iteration, true).unwrap();

    println!("{} Agents with a workplace", sim.agents.occupational_container.iter().filter(|idx| idx.is_some()).count());

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Outbreak Sim")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WORLD_WIDTH as u32, WORLD_WIDTH as u32, surface_texture)?
    };
    let mut world = graphics::WorldGrid::new_empty(WORLD_HEIGHT as usize, WORLD_WIDTH as usize);

    let start_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // User-ended simulation
        if let Event::LoopDestroyed = event {
            write_concluding_metrics(&concluding_report_file, time_step,
                                     Instant::now().duration_since(start_time),
                                     synthetic_environment_dir.join(model_name.to_owned() + ".txt"),
            ).unwrap()
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| ("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // if time_step % 5 == 0 {
            //     println!("Time step: {}", time_step);
            //     println!("Num infected: {}", sim.agents.disease_statuses.iter().filter(|&status| status.state == State::Infectious).count())
            // }
            if time_step % time_steps_per_day == 0 {
                println!("Day: {}", time_step / time_steps_per_day);
                println!("Num infected: {}", sim.agents.disease_statuses.iter().filter(|&status| status.state == State::Infectious).count())
            }

            // let mut time = Instant::now();

            // Update internal state and request a redraw
            for _ in 0..iterations_per_render {
                if sim.update(time_step).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                time_step += 1;
            }
            // println!("Took {:.2}s for {} steps", time.elapsed().as_secs_f64(), increment);

            write_intermediary_metric(&mut intermediary_report_writer, time_step, &sim.agents).unwrap();

            // time = Instant::now();
            world.update(&sim);

            // println!("Draw logic took {:.2}s", time.elapsed().as_secs_f64());

            window.request_redraw();
        }
    });
}
