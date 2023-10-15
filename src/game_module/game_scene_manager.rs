use std::collections::HashMap;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::application::application::EngineApplication;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::scene_manager::{ProjectSceneManagerBase, SceneManager};
use rust_engine_3d::utilities::system;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};
use crate::game_module::character::character::{Character, CharacterData};
use crate::resource::project_resource::ProjectResources;

type CharacterMap = HashMap<String, RcRefCell<Character>>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene_data_name: String,
    pub _start_point: Vector3<f32>,
}

pub struct GameSceneManager {
    pub _scene_manager: *const SceneManager,
    pub _project_resources: *const ProjectResources,
    pub _engine_resources: *const EngineResources,
    pub _renderer_data: *const RendererData,
    pub _effect_manager: *const EffectManager,
    pub _game_scene_name: String,
    pub _character_map: CharacterMap,
}

impl ProjectSceneManagerBase for GameSceneManager {}

impl GameSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager> {
        Box::new(GameSceneManager {
            _scene_manager: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _engine_resources: std::ptr::null(),
            _renderer_data: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _game_scene_name: String::new(),
            _character_map: CharacterMap::new(),
        })
    }

    pub fn initialize_game_scene_manager(
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

        // load scene
        let game_scene_data = project_resources.get_game_scene_data(game_scene_data_name).borrow();
        let scene_data_name = &game_scene_data._scene_data_name;
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);

        // character
        //game_scene_data._start_point;
        self.create_character(String::from("Pickle"));
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn create_character(&mut self, object_name: String) -> RcRefCell<Character> {
        let new_object_name = system::generate_unique_name(&self._character_map, &object_name);
        let character_data = newRcRefCell(CharacterData::default());
        let render_object_data = self.get_scene_manager().get_skeletal_render_object("skeletal0").unwrap();
        let character = newRcRefCell(Character::create_character_instance(
            &new_object_name,
            &character_data,
            &render_object_data
        ));
        self._character_map
            .insert(new_object_name, character.clone());
        character
    }

    pub fn get_character(
        &self,
        object_name: &str,
    ) -> Option<&RcRefCell<Character>> {
        self._character_map.get(object_name)
    }

    pub fn remove_character(&mut self, object_name: &str) {
        self._character_map.remove(object_name);
    }

    pub fn update_game_scene_manager(
        &mut self,
        engine_application: &EngineApplication,
        delta_time: f64,
    ) {
        {
            let pickle = self._character_map.get("Pickle");
            let object = pickle.as_ref().unwrap().borrow();
            let mut object_mut = object._render_object.borrow_mut();
            let height = object_mut._transform_object.get_position().y;
            object_mut._transform_object.set_position(&Vector3::new(0.0, height + height * delta_time as f32 * 0.1, 0.0));
        }

        self.get_scene_manager_mut()
            .update_scene_manager(engine_application, delta_time);
    }
}
