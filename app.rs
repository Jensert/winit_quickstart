use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

pub struct App {
    window: Option<Window>,
    size: PhysicalSize<u32>,
}
impl App {
    pub fn default() -> Self {
        Self {
            window: None,
            size: PhysicalSize::new(0, 0),
        }
    }
    pub fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Winit Window!"))
            .expect("Failed to create window");

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window();

        if window.id() == window_id {
            match event {
                WindowEvent::Resized(size) => {
                    println!("Resizing window");
                    self.size = size;
                }

                WindowEvent::RedrawRequested => {
                    self.window().request_redraw();
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                        event_loop.exit();
                    }
                }

                WindowEvent::CloseRequested => {
                    println!("Closing window");
                }
                _ => (),
            }
        }
    }
}
