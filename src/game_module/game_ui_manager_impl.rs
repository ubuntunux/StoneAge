use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::ui::{UIComponentInstance, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use crate::application::application::Application;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{MATERIAL_CROSS_HAIR, MATERIAL_INTRO_IMAGE};
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::widgets::controller_help::ControllerHelpWidget;
use crate::game_module::widgets::cross_hair_widget::CrossHairWidget;
use crate::game_module::widgets::image_widget::ImageLayout;
use crate::game_module::widgets::item_bar_widget::ItemBarWidget;
use crate::game_module::widgets::player_hud::PlayerHud;
use crate::game_module::widgets::target_status_bar::TargetStatusWidget;
use crate::game_module::widgets::time_of_day::TimeOfDayWidget;

impl<'a> GameUIManager<'a> {
    pub fn create_game_ui_manager() -> Box<GameUIManager<'a>> {
        Box::new(GameUIManager {
            _ui_manager: std::ptr::null(),
            _game_client: std::ptr::null(),
            _root_widget: std::ptr::null(),
            _game_ui_layout: std::ptr::null(),
            _game_image: None,
            _cross_hair: None,
            _target_status_bar: None,
            _time_of_day: None,
            _item_bar_widget: None,
            _player_hud: None,
            _controller_help_widget: None,
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

    pub fn destroy_game_ui_manager(&mut self) {
    }

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

    pub fn is_done_manual_fade_out(&self) -> bool {
        self._game_image.as_ref().unwrap().is_done_manual_fade_out()
    }

    pub fn is_done_game_image_progress(&self) -> bool {
        self._game_image.as_ref().unwrap().is_done_game_image_progress()
    }

    pub fn set_auto_fade_inout(&mut self, auto_fade_inout: bool) {
        self._game_image.as_mut().unwrap().set_auto_fade_inout(auto_fade_inout);
    }

    pub fn set_game_image(&mut self, material_instance_name: &str, fadeout_time: f32, auto_fade_inout: bool) {
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        let material_instance = if game_resources.get_engine_resources().has_material_instance_data(material_instance_name) {
            Some(game_resources.get_engine_resources().get_material_instance_data(material_instance_name).clone())
        } else {
            None
        };

        self._game_image.as_mut().unwrap().set_game_image(&game_resources, material_instance, fadeout_time, auto_fade_inout);
    }

    pub fn set_game_image_fade_speed(&mut self, fade_speed: f32) {
        self._game_image.as_mut().unwrap().set_game_image_fade_speed(fade_speed);
    }

    pub fn build_game_ui(&mut self, window_size: &Vector2<i32>) {
        log::info!("build_game_ui");
        self._window_size = window_size.clone();
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        let engine_resources = game_resources.get_engine_resources();
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
        self._game_image = Some(ImageLayout::create_image_layout(root_widget_mut, &self._window_size, MATERIAL_INTRO_IMAGE));

        let cross_hair_material_instance = game_resources.get_engine_resources().get_material_instance_data(MATERIAL_CROSS_HAIR);
        self._cross_hair = Some(Box::new(CrossHairWidget::create_cross_hair(game_ui_layout_mut, cross_hair_material_instance)));
        self._player_hud = Some(Box::new(PlayerHud::create_player_hud(game_ui_layout_mut)));
        self._controller_help_widget = Some(Box::new(ControllerHelpWidget::create_controller_help_widget(game_ui_layout_mut)));
        self._target_status_bar = Some(Box::new(TargetStatusWidget::create_target_status_widget(game_ui_layout_mut)));
        self._time_of_day = Some(Box::new(TimeOfDayWidget::create_time_of_day_widget(game_ui_layout_mut)));
        self._item_bar_widget = Some(Box::new(ItemBarWidget::create_item_bar_widget(engine_resources, game_ui_layout_mut)));
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
        self._controller_help_widget.as_mut().unwrap().changed_window_size(&self._window_size);
        self._target_status_bar.as_mut().unwrap().changed_window_size(&self._window_size);
        self._time_of_day.as_mut().unwrap().changed_window_size(&self._window_size);
        self._item_bar_widget.as_mut().unwrap().changed_window_size(&self._window_size);
    }

    pub fn add_item(&mut self, item_data_type: &ItemDataType, item_count: usize) -> bool {
        self._item_bar_widget.as_mut().unwrap().add_item(item_data_type, item_count)
    }

    pub fn update_game_ui(&mut self, delta_time: f64) {
        let game_client = ptr_as_ref(self._game_client);
        let game_scene_manager = game_client.get_game_scene_manager();
        let window_size = &game_client.get_application().get_engine_core()._window_size;

        // changed window size
        if self._window_size != *window_size {
            self._window_size = window_size.clone();
            self.changed_window_size();
        }

        if let Some(cross_hair) = self._cross_hair.as_mut() {
            // TEST: hide cross hair
            cross_hair.update_cross_hair_visible(false);

            if cross_hair.get_cross_hair_visible() {
                let engine_core = game_client.get_engine_core();
                cross_hair.update_cross_hair(&engine_core._mouse_move_data._mouse_pos);
            }
        }

        // game image
        if let Some(game_image) = self._game_image.as_mut() {
            game_image.update_game_image(delta_time, false);
        }

        // player hud
        if let Some(player_hud) = self._player_hud.as_mut() {
            if game_scene_manager.get_character_manager().is_valid_player() {
                let player = game_scene_manager.get_character_manager().get_player().borrow();
                player_hud.update_status_widget(&player);
            }
        }

        // target status
        if let Some(target_status_bar) = self._target_status_bar.as_mut() {
            if game_scene_manager.get_character_manager().is_valid_target_character() {
                let target = game_scene_manager.get_character_manager().get_target_character().borrow();
                target_status_bar.update_status_widget(&target);
            } else {
                target_status_bar.fade_out_status_widget();
            }
        }

        // controller_help_widget
        if let Some(controller_help_widget) = self._controller_help_widget.as_mut() {
            controller_help_widget.update_controller_help_widget(game_scene_manager);
        }

        // time of day
        if let Some(time_of_day) = self._time_of_day.as_mut() {
            time_of_day.update_time_of_day_widget(game_scene_manager);
        }
    }
}
