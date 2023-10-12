use nalgebra::{Vector2, Vector3};
use rust_engine_3d::application::application::EngineApplication;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::scene_manager::{ProjectSceneManagerBase, SceneManager};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use crate::resource::project_resource::ProjectResources;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene_data_name: String,
    pub _start_point: Vector3<f32>,
}

pub struct ProjectSceneManager {
    pub _scene_manager: *const SceneManager,
    pub _project_resources: *const ProjectResources,
    pub _engine_resources: *const EngineResources,
    pub _renderer_data: *const RendererData,
    pub _effect_manager: *const EffectManager,
    pub _game_scene_name: String,
}

impl ProjectSceneManagerBase for ProjectSceneManager {}

impl ProjectSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_project_scene_manager() -> Box<ProjectSceneManager> {
        Box::new(ProjectSceneManager {
            _scene_manager: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _engine_resources: std::ptr::null(),
            _renderer_data: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _game_scene_name: String::new(),
        })
    }

    pub fn initialize_project_scene_manager(
        &mut self,
        project_resources: &ProjectResources,
        scene_manager: &mut SceneManager,
        renderer_context: &RendererContext,
        effect_manager: &EffectManager,
        engine_resources: &EngineResources,
        window_size: &Vector2<i32>,
    ) {
        self._scene_manager = scene_manager;
        self._project_resources = project_resources;
        self._renderer_data = renderer_context.get_renderer_data();
        self._effect_manager = effect_manager;
        self._engine_resources = engine_resources;

        scene_manager.initialize_scene_manager(
            self as *const dyn ProjectSceneManagerBase,
            renderer_context,
            effect_manager,
            engine_resources,
            window_size,
        )
    }

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        self._game_scene_name = String::from(game_scene_data_name);
        let project_resources = ptr_as_ref(self._project_resources);

        if false == project_resources.has_game_scene_data(game_scene_data_name) {
            // TODO
        }

        let game_scene_data = project_resources.get_game_scene_data(game_scene_data_name);
        let scene_data_name = &game_scene_data.borrow()._scene_data_name;
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_project_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_project_scene_manager(
        &mut self,
        engine_application: &EngineApplication,
        delta_time: f64,
    ) {
        self.get_scene_manager_mut()
            .update_scene_manager(engine_application, delta_time);
    }
}
