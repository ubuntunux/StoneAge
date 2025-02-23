use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::{SceneDataCreateInfo, SceneManager};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};
use crate::application::application::Application;
use crate::game_module::actors::block::{Block, BlockCreateInfo};
use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::actors::items::ItemManager;
use crate::game_module::actors::props::{PropCreateInfo, PropManager};
use crate::game_module::game_resource::GameResources;

type BlockCreateInfoMap = HashMap<String, BlockCreateInfo>;
type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;
type PropCreateInfoMap = HashMap<String, PropCreateInfo>;
type BlockMap<'a> = HashMap<u64, RcRefCell<Block<'a>>>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene: SceneDataCreateInfo,
    pub _blocks: BlockCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _props: PropCreateInfoMap,
    pub _characters: CharacterCreateInfoMap,
    pub _start_point: Vector3<f32>,
}

pub struct GameSceneManager<'a> {
    pub _audio_manager: *const AudioManager<'a>,
    pub _effect_manager: *const EffectManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _character_manager: Box<CharacterManager<'a>>,
    pub _item_manager: Box<ItemManager<'a>>,
    pub _prop_manager: Box<PropManager<'a>>,
    pub _game_scene_name: String,
    pub _blocks: BlockMap<'a>,
    pub _block_id_generator: u64,
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>
}

impl<'a> GameSceneManager<'a> {
    pub fn get_scene_manager(&self) -> &SceneManager<'a> {
        ptr_as_ref(self._scene_manager)
    }

    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        self._character_manager.as_ref()
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager.as_ref())
    }

    pub fn get_prop_manager(&self) -> &PropManager<'a> {
        self._prop_manager.as_ref()
    }

    pub fn get_prop_manager_mut(&self) -> &mut PropManager<'a> {
        ptr_as_mut(self._prop_manager.as_ref())
    }

    pub fn get_item_manager(&self) -> &ItemManager<'a> {
        self._item_manager.as_ref()
    }

    pub fn get_item_manager_mut(&self) -> &mut ItemManager<'a> {
        ptr_as_mut(self._item_manager.as_ref())
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager<'a>> {
        Box::new(GameSceneManager {
            _audio_manager: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _game_scene_name: String::new(),
            _blocks: HashMap::new(),
            _block_id_generator: 0,
            _character_manager: CharacterManager::create_character_manager(),
            _item_manager: ItemManager::create_item_manager(),
            _prop_manager: PropManager::create_prop_manager(),
            _ambient_sound: None
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application<'a>,
        engine_core: &EngineCore<'a>,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._audio_manager = engine_core.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        engine_core.get_scene_manager_mut().initialize_scene_manager(
            engine_core.get_renderer_context(),
            engine_core.get_audio_manager(),
            engine_core.get_effect_manager(),
            engine_core.get_engine_resources(),
            window_size,
        );

        self._game_resources = application.get_game_resources();
        self._character_manager.initialize_character_manager(engine_core, application);
        self._item_manager.initialize_item_manager(engine_core, application);
        self._prop_manager.initialize_prop_manager(engine_core, application);
    }

    pub fn play_bgm(&mut self, audio_name: &str, volume: Option<f32>) {
        ptr_as_mut(self._audio_manager).play_bgm(audio_name, volume);
    }

    pub fn play_ambient_sound(&mut self, audio_name: &str, volume: Option<f32>) {
        self._ambient_sound = ptr_as_mut(self._audio_manager).play_audio_bank(audio_name, AudioLoop::LOOP, volume);
    }

    pub fn stop_bgm(&self) {
        ptr_as_mut(self._audio_manager).stop_bgm();
    }

    pub fn stop_ambient_sound(&mut self) {
        if let Some(audio_instance_refcell) = self._ambient_sound.as_ref() {
            ptr_as_mut(self._audio_manager).stop_audio_instance(audio_instance_refcell);
        }
        self._ambient_sound = None;
    }

    pub fn get_blocks(&self) -> &BlockMap<'a> {
        &self._blocks
    }

    pub fn check_is_on_block(&self, min: &Vector3<f32>, max: &Vector3<f32>) -> Option<Vector3<f32>> {
        for (_key, block) in self._blocks.iter() {
            let block_ref = block.borrow();
            let render_object_ref = block_ref._render_object.borrow();
            if render_object_ref._bounding_box.collide_bound_box_xy(min, max) {
                return Some(Vector3::new(
                    render_object_ref._bounding_box._center.x,
                    render_object_ref._bounding_box._max.y,
                    render_object_ref._bounding_box._center.z
                ));
            }
        }
        None
    }

    pub fn generate_block_id(&mut self) -> u64 {
        let id = self._block_id_generator;
        self._block_id_generator += 1;
        id
    }

    pub fn register_block(&mut self, block: RcRefCell<Block<'a>>) {
        self._blocks.insert(block.borrow().get_block_id(), block.clone());
    }

    pub fn unregister_block(&mut self, block: &RcRefCell<Block<'a>>) {
        self._blocks.remove(&block.borrow().get_block_id());
    }

    pub fn create_block(&mut self, block_name: &str, block_create_info: &BlockCreateInfo) -> RcRefCell<Block<'a>> {
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: block_create_info._block_data_name.clone(),
            _position: block_create_info._position.clone(),
            _rotation: block_create_info._rotation.clone(),
            _scale: block_create_info._scale.clone()
        };
        let render_object_data = self.get_scene_manager_mut().add_static_render_object(
            block_name,
            &render_object_create_info
        );

        let block_id = self.generate_block_id();
        newRcRefCell(Block::create_block(
            block_id,
            block_name,
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
        self.get_scene_manager_mut().create_scene_data(&game_scene_data._scene);

        // create blocks
        for (block_name, block_create_info) in game_scene_data._blocks.iter() {
            let block = self.create_block(block_name, block_create_info);
            self.register_block(block);
        }

        // create props
        for (prop_name, prop_create_info) in game_scene_data._props.iter() {
            self._prop_manager.create_prop(prop_name, prop_create_info);
        }

        // create player
        for (character_name, character_create_info) in game_scene_data._player.iter() {
            self._character_manager.create_character(character_name, character_create_info, true);
        }

        // create npc
        for (character_name, character_create_info) in game_scene_data._characters.iter() {
            self._character_manager.create_character(character_name, character_create_info, false);
        }

        // first update
        self.update_game_scene_manager(0.0);
        self.get_scene_manager_mut().update_scene_manager(0.0);
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_game_scene_manager(&mut self, delta_time: f64) {
        self._prop_manager.update_prop_manager(delta_time);
        self._item_manager.update_item_manager(delta_time);
        self._character_manager.update_character_manager(delta_time);
    }
}
