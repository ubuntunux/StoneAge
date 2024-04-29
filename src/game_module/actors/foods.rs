use std::collections::HashMap;
use nalgebra::Vector3;

use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};

use crate::application::application::Application;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type FoodMap = HashMap<u64, RcRefCell<Food>>;

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum FoodDataType {
    Meat,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct FoodCreateInfo {
    pub _food_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct FoodData {
    pub _food_type: FoodDataType,
    pub _model_data_name: String,
}

pub struct FoodProperties {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>
}

pub struct Food {
    pub _food_name: String,
    pub _food_id: u64,
    pub _food_data: RcRefCell<FoodData>,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _food_properties: Box<FoodProperties>
}

pub struct FoodManager {
    pub _game_client: *const GameClient,
    pub _game_scene_manager: *const GameSceneManager,
    pub _game_resources: *const GameResources,
    pub _audio_manager: *const AudioManager,
    pub _scene_manager: *const SceneManager,
    pub _id_generator: u64,
    pub _foods: FoodMap
}

// Implementations
impl Default for FoodCreateInfo {
    fn default() -> Self {
        FoodCreateInfo {
            _food_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for FoodData {
    fn default() -> Self {
        FoodData {
            _food_type: FoodDataType::Meat,
            _model_data_name: String::new(),
        }
    }
}

impl Food {
    pub fn create_food(
        food_id: u64,
        food_name: &str,
        food_data: &RcRefCell<FoodData>,
        render_object: &RcRefCell<RenderObjectData>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>) -> Food {
        let mut food = Food {
            _food_name: String::from(food_name),
            _food_id: food_id,
            _food_data: food_data.clone(),
            _render_object: render_object.clone(),
            _food_properties: Box::from(FoodProperties {
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
            }),
        };
        food.update_transform();
        food
    }

    pub fn get_food_id(&self) -> u64 {
        self._food_id
    }

    pub fn update_transform(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&self._food_properties._position);
        render_object._transform_object.set_rotation(&self._food_properties._rotation);
        render_object._transform_object.set_scale(&self._food_properties._scale);
    }
}

impl FoodManager {
    pub fn create_food_manager() -> Box<FoodManager> {
        Box::new(FoodManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _id_generator: 0,
            _foods: HashMap::new(),
        })
    }

    pub fn initialize_food_manager(&mut self, application: &Application) {
        log::info!("initialize_food_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = application.get_game_scene_manager().get_scene_manager();
    }
    pub fn destroy_food_manager(&mut self) {

    }
    pub fn get_game_client(&self) -> *const GameClient { self._game_client }
    pub fn get_game_scene_manager(&self) -> *const GameSceneManager { self._game_scene_manager }
    pub fn get_audio_manager(&self) -> *const AudioManager { self._audio_manager }
    pub fn get_scene_manager(&self) -> *const SceneManager { self._scene_manager }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }
    pub fn get_food(&self, food_id: u64) -> Option<&RcRefCell<Food>> {
        self._foods.get(&food_id)
    }
    pub fn create_food(&mut self, food_name: &str, food_create_info: &FoodCreateInfo) -> RcRefCell<Food> {
        let game_resources = ptr_as_ref(self._game_resources);
        let food_data = game_resources.get_food_data(food_create_info._food_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: food_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = ptr_as_ref(self.get_game_scene_manager()).get_scene_manager_mut().add_static_render_object(
            food_name,
            &render_object_create_info
        );
        let id = self.generate_id();
        let food = newRcRefCell(Food::create_food(
            id,
            food_name,
            food_data,
            &render_object_data,
            &food_create_info._position,
            &food_create_info._rotation,
            &food_create_info._scale
        ));
        self._foods.insert(id, food.clone());
        food
    }
    pub fn remove_food(&mut self, food: &RcRefCell<Food>) {
        self._foods.remove(&food.borrow().get_food_id());
        ptr_as_mut(self.get_scene_manager()).remove_skeletal_render_object(&food.borrow()._food_name);
    }
    pub fn play_audio(&self, audio_name_bank: &str) {
        ptr_as_mut(self.get_audio_manager()).create_audio_instance_from_bank(audio_name_bank, AudioLoop::ONCE);
    }

    pub fn play_effect(&self, effect_name: &str, effect_create_info: &EffectCreateInfo) {
        ptr_as_mut(self.get_scene_manager()).add_effect(effect_name, effect_create_info);
    }

    pub fn update_food_manager(&mut self, _delta_time: f64) {
        for food in self._foods.values() {
            let mut food_mut = food.borrow_mut();
            food_mut.update_transform();
        }

        // let mut dead_foods: Vec<RcRefCell<Food>> = Vec::new();
        // let player = ptr_as_ref(self._player.as_ref().unwrap().as_ptr());
        // if player.is_attacking() {
        //     for food in self._foods.values() {
        //         let mut food_mut = food.borrow_mut();
        //         if food_mut._is_alive {
        //             if food_mut._food_id != player._food_id {
        //                 if food_mut.collide_bound_box(&player.get_attack_point()) {
        //                     food_mut.set_damage(player.get_attack_point(), player.get_power());
        //                     if false == food_mut._is_alive {
        //                         dead_foods.push(food.clone());
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // for food in dead_foods.iter_mut() {
        //     self.remove_food(food);
        // }
    }
}