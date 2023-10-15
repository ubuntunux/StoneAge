pub mod project_module;
pub mod game_module;
pub mod render_pass;
pub mod resource;

pub fn main() {
    project_module::project_application::run_project_application();
}
