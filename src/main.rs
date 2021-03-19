use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use outbreak_sim::disease::State;

const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 1000;
const WORLD_WIDTH: u32 = 800;
const WORLD_HEIGHT: u32 = 800;

mod graphics;


fn main() -> Result<(), Error> {
    let model_name = "model_tower_hamlets";
    // let model_name = "model_greater_manchester";
    // let model_name = "model_london_se_commuter_ring";

    let mut sim = outbreak_sim::Sim::new(model_name, true);

    let mut timestep: u32 = 0;
    let increment: u32 = 5;

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

            if timestep % 5 == 0 {
                println!("Timestep: {}", timestep);
                println!("Num infected: {}", sim.agents.disease_statuses.iter().filter(|&status| status.state == State::Infectious).count())
            }

            // let mut time = Instant::now();

            // Update internal state and request a redraw
            timestep += increment;
            for _ in 0..increment {
                sim.update();
            }
            // println!("Took {:.2}s for {} steps", time.elapsed().as_secs_f64(), increment);
            // time = Instant::now();

            world.update(&sim);

            // println!("Draw logic took {:.2}s", time.elapsed().as_secs_f64());

            window.request_redraw();
        }
    });
}
