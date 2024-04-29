use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};

use crate::application::application::Application;
use crate::game_module::actors::block::{Block, BlockCreateInfo};
use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::actors::foods::FoodManager;
use crate::game_module::game_constants::{BLOCK_HEIGHT, BLOCK_ID_NONE, BLOCK_WIDTH, SHOW_BLOCK};
use crate::game_module::game_resource::GameResources;

type BlockCreateInfoMap = HashMap<String, BlockCreateInfo>;
type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene_data_name: String,
    pub _blocks: BlockCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _characters: CharacterCreateInfoMap,
    pub _start_point: Vector3<f32>,
}

pub struct GameSceneManager {
    pub _effect_manager: *const EffectManager,
    pub _scene_manager: *const SceneManager,
    pub _game_resources: *const GameResources,
    pub _character_manager: *const CharacterManager,
    pub _food_manager: *const FoodManager,
    pub _game_scene_name: String,
    pub _blocks: HashMap<u64, RcRefCell<Block>>,
    pub _block_id_generator: u64,
    pub _map_min_pos: Vector2<f32>,
    pub _map_max_pos: Vector2<f32>,
    pub _map_size: Vector2<f32>,
    pub _block_nums: Vector2<i32>,
    pub _block_map: Vec<Vec<u64>>
}

impl GameSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }

    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager> {
        Box::new(GameSceneManager {
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _food_manager: std::ptr::null(),
            _game_scene_name: String::new(),
            _blocks: HashMap::new(),
            _block_id_generator: 0,
            _map_min_pos: Vector2::new(f32::MAX, f32::MAX),
            _map_max_pos: Vector2::new(f32::MIN, f32::MIN),
            _map_size: Vector2::zeros(),
            _block_nums: Vector2::zeros(),
            _block_map: Vec::new()
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application,
        engine_core: &EngineCore,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._character_manager = application.get_character_manager();
        self._food_manager = application.get_food_manager();
        self._game_resources = application.get_game_resources();
        engine_core.get_scene_manager_mut().initialize_scene_manager(
            engine_core.get_renderer_context(),
            engine_core.get_effect_manager(),
            engine_core.get_engine_resources(),
            window_size,
        )
    }

    pub fn check_is_on_block(&self, prev_position: &Vector3<f32>, position: &Vector3<f32>) -> Option<Vector3<f32>> {
        if let Some(block_indices) = self.convert_pos_to_block_indices(&position) {
            if let Some(_block) = self.get_block_by_indices(&block_indices) {
                let block_ground_pos_y = self.convert_block_indices_to_pos(&block_indices).y + BLOCK_HEIGHT * 0.5;
                if block_ground_pos_y <= prev_position.y {
                    return Some(Vector3::new(position.x, block_ground_pos_y, position.z));
                }
            }
        }
        None
    }

    pub fn convert_block_indices_to_pos(&self, block_indices: &Vector2<usize>) -> Vector3<f32> {
        let x: f32 = self._map_min_pos.x + block_indices.x as f32 * BLOCK_WIDTH + BLOCK_WIDTH * 0.5;
        let y: f32 = self._map_min_pos.y + block_indices.y as f32 * BLOCK_HEIGHT + BLOCK_HEIGHT * 0.5;
        Vector3::new(x, y, 0.0)
    }

    pub fn convert_pos_to_block_indices(&self, block_pos: &Vector3<f32>) -> Option<Vector2<usize>> {
        let x: i32 = ((block_pos.x - self._map_min_pos.x) / BLOCK_WIDTH).floor() as i32;
        let y: i32 = ((block_pos.y - self._map_min_pos.y) / BLOCK_HEIGHT).floor() as i32;
        if 0 <= y && y < self._block_nums.y && 0 <= x && x < self._block_nums.x {
            return Some(Vector2::new(x as usize, y as usize));
        }
        None
    }

    pub fn set_block_id(&mut self, block_id: u64, block_pos: &Vector3<f32>) {
        if let Some(indices) = self.convert_pos_to_block_indices(block_pos) {
            log::info!("set_block_id: {:?} -> {:?}", block_pos, indices);
            self._block_map[indices.y][indices.x] = block_id;
        }
    }

    pub fn get_block(&self, pos: &Vector3<f32>) -> Option<&RcRefCell<Block>> {
        if let Some(indices) = self.convert_pos_to_block_indices(pos) {
            let block_id = self._block_map[indices.y][indices.x];
            return self._blocks.get(&block_id);
        }
        None
    }

    pub fn get_block_by_indices(&self, indices: &Vector2<usize>) -> Option<&RcRefCell<Block>> {
        let block_id = self._block_map[indices.y][indices.x];
        self._blocks.get(&block_id)
    }

    pub fn generate_block_id(&mut self) -> u64 {
        let id = self._block_id_generator;
        self._block_id_generator += 1;
        id
    }

    pub fn register_block(&mut self, block: &RcRefCell<Block>) {
        self._blocks.insert(block.borrow().get_block_id(), block.clone());
    }

    pub fn unregister_block(&mut self, block: &RcRefCell<Block>) {
        self._blocks.remove(&block.borrow().get_block_id());
    }

    pub fn create_block(&mut self, block_name: &str, block_create_info: &BlockCreateInfo) -> RcRefCell<Block> {
        let game_resources = ptr_as_ref(self._game_resources);
        let block_data = game_resources.get_block_data(block_create_info._block_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: block_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_static_render_object(
            block_name,
            &render_object_create_info
        );

        render_object_data.borrow_mut().set_render(SHOW_BLOCK);

        let block_id = self.generate_block_id();
        newRcRefCell(Block::create_block(
            block_id,
            block_name,
            block_data,
            &render_object_data,
            &block_create_info._position,
            &block_create_info._rotation,
            &block_create_info._scale
        ))
    }

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        log::info!("open_game_scene_data: {:?}", game_scene_data_name);
        self._game_scene_name = String::from(game_scene_data_name);
        let game_resources = ptr_as_ref(self._game_resources);

        if false == game_resources.has_game_scene_data(game_scene_data_name) {
            // TODO
        }

        // load scene
        let game_scene_data = game_resources.get_game_scene_data(game_scene_data_name).borrow();
        let scene_data_name = &game_scene_data._scene_data_name;
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);

        // create blocks
        self._map_min_pos = Vector2::new(f32::MAX, f32::MAX);
        self._map_max_pos = Vector2::new(f32::MIN, f32::MIN);
        for (block_name, block_create_info) in game_scene_data._blocks.iter() {
            let block = self.create_block(block_name, block_create_info);
            let block_ref = block.borrow();
            let pos = &block_ref._block_properties._position;
            if pos.x < self._map_min_pos.x { self._map_min_pos.x = pos.x; }
            if pos.y < self._map_min_pos.y { self._map_min_pos.y = pos.y; }
            if self._map_max_pos.x < pos.x { self._map_max_pos.x = pos.x; }
            if self._map_max_pos.y < pos.y { self._map_max_pos.y = pos.y; }
            self.register_block(&block);
        }

        // register block id
        self._map_min_pos.x -= BLOCK_WIDTH * 0.5;
        self._map_min_pos.y -= BLOCK_HEIGHT * 0.5;
        self._map_max_pos.x += BLOCK_WIDTH * 0.5;
        self._map_max_pos.y += BLOCK_HEIGHT * 0.5;
        self._map_size = self._map_max_pos - self._map_min_pos;
        self._block_nums.x = (self._map_size.x / BLOCK_WIDTH).ceil() as i32;
        self._block_nums.y = (self._map_size.y / BLOCK_HEIGHT).ceil() as i32;
        let none_blocks: Vec<u64> = vec![BLOCK_ID_NONE; self._block_nums.x as usize];
        self._block_map = vec![none_blocks; self._block_nums.y as usize];
        let blocks = self._blocks.clone();
        for (block_id, block) in blocks {
            let block_pos = block.borrow()._block_properties._position;
            self.set_block_id(block_id, &block_pos);
        }

        //log::info!("min: {:?}, max: {:?}, size: {:?}, nums: {:?}", self._map_min_pos, self._map_max_pos, self._map_size, self._block_nums);

        // create player
        let character_manager = ptr_as_mut(self._character_manager.clone());
        for (character_name, character_create_info) in game_scene_data._player.iter() {
            let _character = character_manager.create_character(character_name, character_create_info, true);
        }

        // create npc
        for (character_name, character_create_info) in game_scene_data._characters.iter() {
            let _character = character_manager.create_character(character_name, character_create_info, false);
        }
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_game_scene_manager(&mut self, delta_time: f64) {
        ptr_as_mut(self._food_manager).update_food_manager(delta_time);
        ptr_as_mut(self._character_manager).update_character_manager(delta_time);
    }
}
