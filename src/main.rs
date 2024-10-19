use graphics::window::Window;

mod graphics;

fn main() {
    let mut window = Window::new(600, 600, "App");
    window.init_gl();
    while !window.should_close() {
        window.update();
    }
}
