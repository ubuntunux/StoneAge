pub mod application;
pub mod game_module;
pub mod render_pass;

use crate::game_module::game_constants::DEPLOY;
use rust_engine_3d::constants::DEVELOPMENT;
use std::env;

pub fn main() {
    unsafe {
        // example)
        // cargo run --release -- development
        // cargo run --release -- deploy
        let args: Vec<String> = env::args().collect();
        for arg in args.iter().enumerate() {
            if arg.1 == "development" {
                DEVELOPMENT = true;
            } else if arg.1 == "deploy" {
                DEVELOPMENT = true;
                DEPLOY = true;
            }
        }
    }

    application::application::run_application();
}
