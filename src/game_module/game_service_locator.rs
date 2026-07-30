use crate::application::application::Application;
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

static mut APPLICATION_PTR: *const Application<'static> = ptr::null();

pub fn set_application<'a>(application: *const Application<'a>) {
    unsafe {
        APPLICATION_PTR = application as *const Application<'static>;
    }
}

pub fn clear_game_service_locator() {
    unsafe {
        APPLICATION_PTR = ptr::null();
    }
}

// Global Getters
pub fn get_application<'a>() -> &'a Application<'a> {
    ptr_as_ref(unsafe { APPLICATION_PTR } as *const Application<'a>)
}

pub fn get_application_mut<'a>() -> &'a mut Application<'a> {
    ptr_as_mut(unsafe { APPLICATION_PTR } as *const Application<'a>)
}

pub fn get_game_client<'a>() -> &'a GameClient<'a> {
    get_application()._game_client.as_ref()
}

pub fn get_game_client_mut<'a>() -> &'a mut GameClient<'a> {
    ptr_as_mut(get_application()._game_client.as_ref())
}

pub fn get_game_scene_manager<'a>() -> &'a GameSceneManager<'a> {
    get_application()._game_scene_manager.as_ref()
}

pub fn get_game_scene_manager_mut<'a>() -> &'a mut GameSceneManager<'a> {
    ptr_as_mut(get_application()._game_scene_manager.as_ref())
}

pub fn get_character_manager<'a>() -> &'a CharacterManager<'a> {
    get_game_scene_manager()._character_manager.as_ref()
}

pub fn get_character_manager_mut<'a>() -> &'a mut CharacterManager<'a> {
    ptr_as_mut(get_game_scene_manager()._character_manager.as_ref())
}

pub fn get_item_manager<'a>() -> &'a ItemManager<'a> {
    get_game_scene_manager()._item_manager.as_ref()
}

pub fn get_item_manager_mut<'a>() -> &'a mut ItemManager<'a> {
    ptr_as_mut(get_game_scene_manager()._item_manager.as_ref())
}

pub fn get_prop_manager<'a>() -> &'a PropManager<'a> {
    get_game_scene_manager()._prop_manager.as_ref()
}

pub fn get_prop_manager_mut<'a>() -> &'a mut PropManager<'a> {
    ptr_as_mut(get_game_scene_manager()._prop_manager.as_ref())
}

pub fn get_game_resources<'a>() -> &'a GameResources<'a> {
    get_application()._game_resources.as_ref()
}

pub fn get_game_resources_mut<'a>() -> &'a mut GameResources<'a> {
    ptr_as_mut(get_application()._game_resources.as_ref())
}

pub fn get_game_ui_manager<'a>() -> &'a GameUIManager<'a> {
    get_application()._game_ui_manager.as_ref()
}

pub fn get_game_ui_manager_mut<'a>() -> &'a mut GameUIManager<'a> {
    ptr_as_mut(get_application()._game_ui_manager.as_ref())
}

pub fn get_editor_ui_manager<'a>() -> &'a EditorUIManager<'a> {
    get_application()._editor_ui_manager.as_ref()
}

pub fn get_editor_ui_manager_mut<'a>() -> &'a mut EditorUIManager<'a> {
    ptr_as_mut(get_application()._editor_ui_manager.as_ref())
}

pub fn get_game_controller<'a>() -> &'a GameController<'a> {
    get_application()._game_controller.as_ref()
}

pub fn get_game_controller_mut<'a>() -> &'a mut GameController<'a> {
    ptr_as_mut(get_application()._game_controller.as_ref())
}
