use crate::game_module::actors::character::manager::CharacterManager;
use crate::game_module::actors::items::ItemManager;
use crate::game_module::actors::props::PropManager;

use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::{EditorUIManager, GameUIManager};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use std::ptr;

pub struct GameServiceLocator {
    pub _game_client: *const GameClient<'static>,
    pub _game_resources: *const GameResources<'static>,
    pub _game_scene_manager: *const GameSceneManager<'static>,
    pub _character_manager: *const CharacterManager<'static>,
    pub _item_manager: *const ItemManager<'static>,
    pub _prop_manager: *const PropManager<'static>,
    pub _game_ui_manager: *const GameUIManager<'static>,
    pub _editor_ui_manager: *const EditorUIManager<'static>,
    pub _game_controller: *const GameController<'static>,
}

impl Default for GameServiceLocator {
    fn default() -> Self {
        Self {
            _game_client: ptr::null(),
            _game_resources: ptr::null(),
            _game_scene_manager: ptr::null(),
            _character_manager: ptr::null(),
            _item_manager: ptr::null(),
            _prop_manager: ptr::null(),
            _game_ui_manager: ptr::null(),
            _editor_ui_manager: ptr::null(),
            _game_controller: ptr::null(),
        }
    }
}

static mut GAME_SERVICE_LOCATOR: GameServiceLocator = GameServiceLocator {
    _game_client: ptr::null(),
    _game_resources: ptr::null(),
    _game_scene_manager: ptr::null(),
    _character_manager: ptr::null(),
    _item_manager: ptr::null(),
    _prop_manager: ptr::null(),
    _game_ui_manager: ptr::null(),
    _editor_ui_manager: ptr::null(),
    _game_controller: ptr::null(),
};

pub fn set_game_resources(game_resources: *const GameResources<'static>) {
    let locator = get_game_service_locator_mut();
    locator._game_resources = game_resources;
}

pub fn get_game_service_locator() -> &'static GameServiceLocator {
    ptr_as_ref(std::ptr::addr_of!(GAME_SERVICE_LOCATOR))
}

pub fn get_game_service_locator_mut() -> &'static mut GameServiceLocator {
    ptr_as_mut(std::ptr::addr_of!(GAME_SERVICE_LOCATOR))
}

pub fn register_game_service_locator<'a>(
    game_client: *const GameClient<'a>,
    game_resources: *const GameResources<'a>,
    game_scene_manager: *const GameSceneManager<'a>,
    character_manager: *const CharacterManager<'a>,
    item_manager: *const ItemManager<'a>,
    prop_manager: *const PropManager<'a>,
    game_ui_manager: *const GameUIManager<'a>,
    editor_ui_manager: *const EditorUIManager<'a>,
    game_controller: *const GameController<'a>,
) {
    let locator = get_game_service_locator_mut();
    locator._game_client = game_client as *const GameClient<'static>;
    locator._game_resources = game_resources as *const GameResources<'static>;
    locator._game_scene_manager = game_scene_manager as *const GameSceneManager<'static>;
    locator._character_manager = character_manager as *const CharacterManager<'static>;
    locator._item_manager = item_manager as *const ItemManager<'static>;
    locator._prop_manager = prop_manager as *const PropManager<'static>;
    locator._game_ui_manager = game_ui_manager as *const GameUIManager<'static>;
    locator._editor_ui_manager = editor_ui_manager as *const EditorUIManager<'static>;
    locator._game_controller = game_controller as *const GameController<'static>;
}

pub fn clear_game_service_locator() {
    let locator = get_game_service_locator_mut();
    *locator = GameServiceLocator::default();
}

// Global Getters
pub fn get_game_client<'a>() -> &'a GameClient<'a> {
    ptr_as_ref(get_game_service_locator()._game_client as *const GameClient<'a>)
}

pub fn get_game_client_mut<'a>() -> &'a mut GameClient<'a> {
    ptr_as_mut(get_game_service_locator()._game_client as *const GameClient<'a>)
}

pub fn get_game_scene_manager<'a>() -> &'a GameSceneManager<'a> {
    ptr_as_ref(get_game_service_locator()._game_scene_manager as *const GameSceneManager<'a>)
}

pub fn get_game_scene_manager_mut<'a>() -> &'a mut GameSceneManager<'a> {
    ptr_as_mut(get_game_service_locator()._game_scene_manager as *const GameSceneManager<'a>)
}

pub fn get_character_manager<'a>() -> &'a CharacterManager<'a> {
    ptr_as_ref(get_game_service_locator()._character_manager as *const CharacterManager<'a>)
}

pub fn get_character_manager_mut<'a>() -> &'a mut CharacterManager<'a> {
    ptr_as_mut(get_game_service_locator()._character_manager as *const CharacterManager<'a>)
}

pub fn get_item_manager<'a>() -> &'a ItemManager<'a> {
    ptr_as_ref(get_game_service_locator()._item_manager as *const ItemManager<'a>)
}

pub fn get_item_manager_mut<'a>() -> &'a mut ItemManager<'a> {
    ptr_as_mut(get_game_service_locator()._item_manager as *const ItemManager<'a>)
}

pub fn get_prop_manager<'a>() -> &'a PropManager<'a> {
    ptr_as_ref(get_game_service_locator()._prop_manager as *const PropManager<'a>)
}

pub fn get_prop_manager_mut<'a>() -> &'a mut PropManager<'a> {
    ptr_as_mut(get_game_service_locator()._prop_manager as *const PropManager<'a>)
}

pub fn get_game_resources<'a>() -> &'a GameResources<'a> {
    ptr_as_ref(get_game_service_locator()._game_resources as *const GameResources<'a>)
}

pub fn get_game_resources_mut<'a>() -> &'a mut GameResources<'a> {
    ptr_as_mut(get_game_service_locator()._game_resources as *const GameResources<'a>)
}

pub fn get_game_ui_manager<'a>() -> &'a GameUIManager<'a> {
    ptr_as_ref(get_game_service_locator()._game_ui_manager as *const GameUIManager<'a>)
}

pub fn get_game_ui_manager_mut<'a>() -> &'a mut GameUIManager<'a> {
    ptr_as_mut(get_game_service_locator()._game_ui_manager as *const GameUIManager<'a>)
}

pub fn get_editor_ui_manager<'a>() -> &'a EditorUIManager<'a> {
    ptr_as_ref(get_game_service_locator()._editor_ui_manager as *const EditorUIManager<'a>)
}

pub fn get_editor_ui_manager_mut<'a>() -> &'a mut EditorUIManager<'a> {
    ptr_as_mut(get_game_service_locator()._editor_ui_manager as *const EditorUIManager<'a>)
}

pub fn get_game_controller<'a>() -> &'a GameController<'a> {
    ptr_as_ref(get_game_service_locator()._game_controller as *const GameController<'a>)
}

pub fn get_game_controller_mut<'a>() -> &'a mut GameController<'a> {
    ptr_as_mut(get_game_service_locator()._game_controller as *const GameController<'a>)
}
