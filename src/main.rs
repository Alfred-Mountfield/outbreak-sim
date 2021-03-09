use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::window::WindowBuilder;
use outbreak_sim::{routing, agents, read_buffer, get_root_as_model};
use std::time::Instant;
use rand::Rng;
use rayon::prelude::*;
use fast_paths::ShortestPath;
use outbreak_sim::routing::nodes_to_granular_grid;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const WORLD_WIDTH: u32 = 600;
const WORLD_HEIGHT: u32 = 600;

mod graphics;


fn main() -> Result<(), Error> {
    let bytes = read_buffer("python/synthetic_population/output/model_greater_manchester_1m.txt");
    // let bytes = read_buffer("python/synthetic_population/output/model_london_se_commuter_ring_8m.txt");
    let model = get_root_as_model(&bytes);
    let bounds = model.bounds().to_owned();
    let transit_graph = model.transit_graph();
    let fast_graph = routing::preprocess_graph(&transit_graph);
    let transit_node_grid = nodes_to_granular_grid(&transit_graph, &bounds);

    let agent_households = model.agents().household_index();
    let household_positions = model.households().pos();
    let agents = agents::Agents::new(agent_households, household_positions);


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

    event_loop.run(move |event, _, control_flow| {
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

            // Update internal state and request a redraw
            // agents.update();
            world.update(&agents, &bounds);
            window.request_redraw();
        }
    });
}
