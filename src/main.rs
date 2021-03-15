use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use outbreak_sim::{agents, get_root_as_model, read_buffer, routing};
use outbreak_sim::pois::create_containers;
use outbreak_sim::disease::Uniform;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const WORLD_WIDTH: u32 = 600;
const WORLD_HEIGHT: u32 = 600;

mod graphics;


fn main() -> Result<(), Error> {
    // let model_name = "model_greater_manchester";
    // let model_name = "model_london_se_commuter_ring";
    let model_name = "model_tower_hamlets";
    let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
    let model = get_root_as_model(&bytes);
    // TODO Ensure that this is non-inclusive
    let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

    let agent_households = model.agents().household_index();
    let household_positions = model.households().pos();
    let agents = agents::Agents::new(agent_households, household_positions);

    let transit_graph = model.transit_graph();

    println!("Creating Contraction Hierarchies");
    let now = Instant::now();
    let fast_graph = routing::preprocess_graph(&transit_graph);
    println!("{:.6}s", now.elapsed().as_secs_f64());
    fast_paths::save_to_disk(&fast_graph, &*("fast_paths/".to_string() + model_name + ".fp")).unwrap();

    let fast_graph = fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap();

    let workplace_indices = model.agents().workplace_index().unwrap().safe_slice();
    let workplace_positions = model.workplaces().pos();

    let containers = create_containers::<Uniform>();

    let num_commuting_agents = agents.positions.iter().zip(workplace_indices.iter())
        .filter(|(&pos, &workplace_idx)| {
            return workplace_idx != u32::MAX
    }).count();
    println!("{} Agents with a workplace", num_commuting_agents);

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
