
use std::thread;
use std::time::Duration;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{self, Event, WindowEvent};
use winit::event_loop::{self, ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


mod physics;
pub mod position;
pub mod vector2;
pub mod settings;

use settings::{WIDTH, HEIGHT};
use physics::PhysicsSimulation;


fn main() {

    let my_window_handler = MyWindowHandler::new();

}


pub struct MyWindowHandler {
    pub window: winit::window::Window,
    pub pixels: pixels::Pixels,
    pub input: winit_input_helper::WinitInputHelper,
}

impl MyWindowHandler {

    pub fn new() -> MyWindowHandler {

        let event_loop = EventLoop::new().unwrap();
        let input = WinitInputHelper::new();

        let window = {

            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

            WindowBuilder::new()
                .with_title("Hello!")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()

        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        let mut window_handler = MyWindowHandler {
            window,
            pixels,
            input,
        };

        window_handler.init_event_loop(event_loop);
        window_handler

    }

    fn init_event_loop(&mut self, event_loop: EventLoop<()>) {

        let mut sim = PhysicsSimulation::new();

        event_loop.run(|event, target| {

            match event {
        
                Event::WindowEvent { window_id, event } => {

                    match event {
                        WindowEvent::RedrawRequested => {

                            //sim.tick();
                            sim.draw(self.pixels.frame_mut());
                            self.pixels.render().unwrap();
                            
                            
                        },
                        WindowEvent::CloseRequested => {

                            panic!("Window closed");  

                        }
                        _ => {}
                    }

                }

                _ => {}
            }
            //self.window.request_redraw();

        }).unwrap();

    }

}
