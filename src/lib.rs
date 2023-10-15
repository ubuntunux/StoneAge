pub mod project_module;
pub mod game_module;
pub mod render_pass;
pub mod resource;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    project_module::project_application::run_project_application();
}
