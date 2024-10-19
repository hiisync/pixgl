use glfw::{Context, GlfwReceiver, WindowEvent};

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW.");

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Window {
            glfw,
            window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.window.swap_buffers();
    }

    pub fn process_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                _ => {}
            }
        }
    }
}
