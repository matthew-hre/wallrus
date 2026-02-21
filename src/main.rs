mod application;
mod export;
mod gl_renderer;
mod palette;
mod shader;
mod shader_presets;
mod wallpaper;
mod window;

use application::WallrusApplication;

const APP_ID: &str = "com.megakode.Wallrus";

fn main() {
    let app = WallrusApplication::new(APP_ID);
    std::process::exit(app.run());
}
