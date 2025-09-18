use crate::app::App;
use winit::event_loop::EventLoop;
mod app;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();

    let res = event_loop.run_app(&mut app);

    match res {
        Ok(_) => {
            println!("Program finished without errors!");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}
