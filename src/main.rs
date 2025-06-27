pub mod application;
pub mod game_module;
pub mod render_pass;

use std::env;
use rust_engine_3d::constants::DEVELOPMENT;

pub fn main() {
    unsafe {
        // example) cargo run --release -- development
        let args: Vec<String> = env::args().collect();
        for arg in args.iter().enumerate() {
            if arg.1 == "development" {
                DEVELOPMENT = true;
            }
        }
    }

    application::application::run_application();
}
