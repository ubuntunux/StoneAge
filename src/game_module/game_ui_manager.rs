use std::rc::Rc;

use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::ui::{UIComponentInstance, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

use crate::application::application::Application;
use crate::game_module::game_client::GameClient;
use crate::game_module::widgets::player_hud::PlayerHud;
use crate::game_module::widgets::image_widget::ImageLayout;
use crate::game_module::widgets::target_status_bar::TargetStatusWidget;

pub struct GameUIManager<'a> {
    pub _ui_manager: *const UIManager<'a>,
    pub _game_client: *const GameClient<'a>,
    pub _root_widget: *const WidgetDefault<'a>,
    pub _game_ui_layout: *const WidgetDefault<'a>,
    pub _game_image: Option<Box<ImageLayout<'a>>>,
    pub _player_hud: Option<Box<PlayerHud<'a>>>,
    pub _target_status_bar: Option<Box<TargetStatusWidget<'a>>>,
    pub _window_size: Vector2<i32>,
}

pub struct UISwitch<'a> {
    pub _ui_switch_widget: Rc<WidgetDefault<'a>>,
}


// Implementations
impl<'a> GameUIManager<'a> {
    pub fn create_game_ui_manager() -> Box<GameUIManager<'a>> {
        Box::new(GameUIManager {
            _ui_manager: std::ptr::null(),
            _game_client: std::ptr::null(),
            _root_widget: std::ptr::null(),
            _game_ui_layout: std::ptr::null(),
            _game_image: None,
            _target_status_bar: None,
            _player_hud: None,
            _window_size: Vector2::new(1024,768)
        })
    }

    pub fn game_ui_layout(&self) -> *const WidgetDefault<'a> {
        self._game_ui_layout
    }
}

impl<'a> GameUIManager<'a> {
    pub fn initialize_game_ui_manager(&mut self, engine_core: &EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_game_ui_manager");
        self._game_client = application.get_game_client();
        self._ui_manager = engine_core.get_ui_manager();
        self._root_widget = ptr_as_ref(self._ui_manager).get_root_ptr();
    }
    pub fn destroy_game_ui_manager(&mut self) {}
    pub fn get_game_client(&self) -> &GameClient<'a> {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient<'a> {
        ptr_as_mut(self._game_client)
    }
    pub fn get_ui_manager(&self) -> &UIManager<'a> {
        ptr_as_ref(self._ui_manager)
    }
    pub fn get_ui_manager_mut(&self) -> &mut UIManager<'a> {
        ptr_as_mut(self._ui_manager)
    }
    pub fn get_root_widget(&self) -> &WidgetDefault<'a> {
        ptr_as_ref(self._root_widget)
    }
    pub fn get_root_widget_mut(&self) -> &mut WidgetDefault<'a> {
        ptr_as_mut(self._root_widget)
    }
    pub fn is_visible_game_image(&self) -> bool {
        self._game_image.as_ref().unwrap().is_visible()
    }
    pub fn start_game_image_fadeout(&mut self, start_fadeout: bool) {
        self._game_image.as_mut().unwrap().start_fadeout(start_fadeout);
    }
    pub fn set_game_image_material_instance(&mut self, material_instance: &str, fadeout_time: f32) {
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        self._game_image.as_mut().unwrap().set_material_instance(
            game_resources,
            &self._window_size,
            material_instance,
            fadeout_time
        );
    }
    pub fn build_game_ui(&mut self, window_size: &Vector2<i32>) {
        log::info!("build_game_ui");
        self._window_size = window_size.clone();
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        let _engine_resources = game_resources.get_engine_resources();
        let root_widget_mut = ptr_as_mut(self._root_widget);

        // create layout
        let game_ui_layout = UIManager::create_widget("game ui layout", UIWidgetTypes::Default);
        let game_ui_layout_mut: &mut WidgetDefault = ptr_as_mut(game_ui_layout.as_ref());
        let ui_component: &mut UIComponentInstance = game_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        root_widget_mut.add_widget(&game_ui_layout);
        self._game_ui_layout = game_ui_layout.as_ref();
        self._game_image = Some(ImageLayout::create_image_layout(root_widget_mut, "ui/intro_image"));
        self._player_hud = Some(Box::new(PlayerHud::create_player_hud(game_ui_layout_mut)));
        self._target_status_bar = Some(Box::new(TargetStatusWidget::create_target_status_widget(game_ui_layout_mut)));
        self.changed_window_size();
    }
    pub fn show_ui(&mut self, show: bool) {
        if false == self._game_ui_layout.is_null() {
            let game_ui_layout_mut = ptr_as_mut(self._game_ui_layout);
            game_ui_layout_mut.get_ui_component_mut().set_visible(show);
        }
    }
    pub fn changed_window_size(&mut self) {
        log::info!("GameUIManager::changed_window_size: {:?}", self._window_size);
        self._game_image.as_mut().unwrap().changed_window_size(&self._window_size);
        self._player_hud.as_mut().unwrap().changed_window_size(&self._window_size);
        self._target_status_bar.as_mut().unwrap().changed_window_size(&self._window_size);
    }
    pub fn update_game_ui(&mut self, delta_time: f64) {
        let game_client = ptr_as_ref(self._game_client);
        let window_size = &game_client
            .get_application()
            .get_engine_core()
            ._window_size;

        // changed window size
        if self._window_size != *window_size {
            self._window_size = window_size.clone();
            self.changed_window_size();
        }

        // intro image
        if let Some(game_image) = self._game_image.as_mut() {
            game_image.update_image_layout(delta_time);
        }

        // player hud
        if let Some(player_hud) = self._player_hud.as_mut() {
            if game_client.get_character_manager().is_valid_player() {
                let player = game_client.get_character_manager().get_player().borrow();
                player_hud.update_status_widget(&player);
            }
        }

        // target status
        if let Some(target_status_bar) = self._target_status_bar.as_mut() {
            if game_client.get_character_manager().is_valid_target_character() {
                let target = game_client.get_character_manager().get_target_character().borrow();
                target_status_bar.update_status_widget(&target);
            } else {
                target_status_bar.fade_out_status_widget();
            }
        }
    }
}
