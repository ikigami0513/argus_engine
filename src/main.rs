#[link(name = "shell32")]
extern "C" {}

mod core;
mod graphics;
mod world;

use crate::core::application::Application;

fn main() {
    let mut app = Application::new();
    app.run();
}