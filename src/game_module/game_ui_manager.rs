use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{
    ITEM_ENERGY_BALL, ITEM_SPIRIT_BALL, MATERIAL_INTRO_IMAGE, RECORD_DEFAULT_INCREMENT,
};
use crate::game_module::game_service_locator::{get_character_manager, get_game_resources, get_game_scene_manager};
use crate::game_module::save_data::save_data::PlayerRecords;
use crate::game_module::widgets::controller_help::ControllerHelpWidget;
use crate::game_module::widgets::cooking_widget::CookingWidget;
use crate::game_module::widgets::cross_hair_widget::CrossHairWidget;
use crate::game_module::widgets::debug_ui_widget::DebugUIWidget;
use crate::game_module::widgets::game_menu_widget::{GameMenuTab, GameMenuWidget, InventoryWidget};
use crate::game_module::widgets::image_widget::ImageLayout;
use crate::game_module::widgets::item_acquire_notification::ItemAcquireNotificationWidget;
use crate::game_module::widgets::item_bar::{InventoryItemCreateInfoList, ItemBarWidget};
use crate::game_module::widgets::key_binding_widget::KeyBindingWidgetManager;
use crate::game_module::widgets::player_hud::PlayerHud;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::{QuestItemBase, QuestWidget};
use crate::game_module::widgets::target_status_bar::TargetStatusWidget;
use crate::game_module::widgets::text_box_widget::{
    TextBoxContent, TextBoxItemOption, TextBoxLayerType, TextBoxWidget,
};
use crate::game_module::widgets::time_of_day::TimeOfDayWidget;
use crate::game_module::widgets::toolbox_widget::ToolboxWidget;
use crate::game_module::widgets::world_map::WorldMapWidget;
use nalgebra::Vector2;
use rust_engine_3d::constants::DEVELOPMENT;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_engine_core, get_engine_resources, get_ui_manager};
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{UIComponentInstance, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use std::collections::HashSet;
use std::ffi::c_void;

pub type QuestItem<'a> = RcRefCell<dyn QuestItemBase<'a> + 'a>;

pub struct EditorUIManager<'a> {
    pub _editor_ui_layout: *const WidgetDefault<'a>,
    pub _window_size: Vector2<i32>,
    pub _need_to_refresh: bool,
}

pub struct GameUIManager<'a> {
    pub _game_ui_layout: *const WidgetDefault<'a>,
    pub _game_image: Option<Box<ImageLayout<'a>>>,
    pub _key_binding_widget_manager: Option<Box<KeyBindingWidgetManager<'a>>>,
    pub _cross_hair: Option<Box<CrossHairWidget<'a>>>,
    pub _game_menu_widget: Option<Box<GameMenuWidget<'a>>>,
    pub _player_hud: Option<Box<PlayerHud<'a>>>,
    pub _text_box_widget: Option<Box<TextBoxWidget<'a>>>,
    pub _controller_help_widget: Option<Box<ControllerHelpWidget<'a>>>,
    pub _target_status_bar: Option<Box<TargetStatusWidget<'a>>>,
    pub _time_of_day: Option<Box<TimeOfDayWidget<'a>>>,
    pub _item_bar_widget: Option<Box<ItemBarWidget<'a>>>,
    pub _item_acquire_notification_widget: Option<Box<ItemAcquireNotificationWidget<'a>>>,
    pub _toolbox_widget: Option<Box<ToolboxWidget<'a>>>,
    pub _cooking_widget: Option<Box<CookingWidget<'a>>>,
    pub _quest_widget: Option<Box<QuestWidget<'a>>>,
    pub _world_map_widget: Option<Box<WorldMapWidget<'a>>>,
    pub _debug_ui_widget: Option<Box<DebugUIWidget<'a>>>,
    pub _window_size: Vector2<i32>,
    pub _need_to_refresh: bool,
    pub _player_records: PlayerRecords,
}

impl<'a> EditorUIManager<'a> {
    pub fn create_editor_ui_manager() -> Box<EditorUIManager<'a>> {
        Box::new(EditorUIManager {
            _editor_ui_layout: std::ptr::null(),
            _window_size: Vector2::new(1024, 768),
            _need_to_refresh: true,
        })
    }

    pub fn initialize_editor_ui_manager(&mut self) {
        log::info!("initialize_editor_ui_manager");
    }

    pub fn destroy_editor_ui_manager(&mut self) {}

    pub fn get_editor_ui_layout(&self) -> *const WidgetDefault<'a> {
        self._editor_ui_layout
    }

    pub fn show_editor_ui(&mut self, show: bool) {
        if !self._editor_ui_layout.is_null() {
            let editor_ui_layout_mut = ptr_as_mut(self._editor_ui_layout);
            editor_ui_layout_mut.get_ui_component_mut().set_visible(show);
        }
    }

    pub fn build_editor_ui(&mut self, _window_size: &Vector2<i32>) {
        log::info!("build_editor_ui");
        let editor_ui_layout = UIManager::create_widget("editor ui layout", UIWidgetTypes::Default);
        let editor_ui_layout_mut: &mut WidgetDefault = ptr_as_mut(editor_ui_layout.as_ref());
        let ui_component: &mut UIComponentInstance = editor_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        let root_widget = get_ui_manager().get_root_ptr();
        ptr_as_mut(root_widget).add_widget(&editor_ui_layout);
        self._editor_ui_layout = editor_ui_layout.as_ref();
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        log::info!("EditorUIComponents::changed_window_size: {:?}", window_size);
        self._window_size = *window_size;
    }

    pub fn update_editor_ui(&mut self, _delta_time: f64) {
        let ui_manager = get_ui_manager();
        if self._need_to_refresh || self._window_size != ui_manager._window_size {
            self._window_size = ui_manager._window_size;
            self.changed_window_size(&ui_manager._window_size);
            self._need_to_refresh = false;
        }
    }
}

impl<'a> GameUIManager<'a> {
    pub fn create_game_ui_manager() -> Box<GameUIManager<'a>> {
        Box::new(GameUIManager {
            _game_ui_layout: std::ptr::null(),
            _game_image: None,
            _key_binding_widget_manager: None,
            _cross_hair: None,
            _game_menu_widget: None,
            _text_box_widget: None,
            _target_status_bar: None,
            _time_of_day: None,
            _item_bar_widget: None,
            _item_acquire_notification_widget: None,
            _player_hud: None,
            _controller_help_widget: None,
            _toolbox_widget: None,
            _cooking_widget: None,
            _quest_widget: None,
            _world_map_widget: None,
            _debug_ui_widget: None,
            _window_size: Vector2::new(0, 0),
            _need_to_refresh: true,
            _player_records: PlayerRecords::default(),
        })
    }

    pub fn initialize_game_ui_manager(&mut self) {
        log::info!("initialize_game_ui_manager");
    }

    pub fn destroy_game_ui_manager(&mut self) {}

    pub fn get_root_widget(&self) -> &WidgetDefault<'a> {
        ptr_as_ref(get_ui_manager().get_root_ptr())
    }

    pub fn get_root_widget_mut(&self) -> &mut WidgetDefault<'a> {
        ptr_as_mut(get_ui_manager().get_root_ptr())
    }

    pub fn get_item_bar_widget(&self) -> &ItemBarWidget<'a> {
        self._item_bar_widget.as_ref().unwrap()
    }

    pub fn build_game_ui(&mut self, window_size: &Vector2<i32>) {
        log::info!("build_game_ui");
        let _game_scene_manager = get_game_scene_manager();
        let root_widget = ptr_as_mut(get_ui_manager().get_root_ptr());

        // game ui layer
        let game_ui_layout = UIManager::create_widget("game ui layout", UIWidgetTypes::Default);
        let game_ui_layout_mut: &mut WidgetDefault = ptr_as_mut(game_ui_layout.as_ref());
        let ui_component: &mut UIComponentInstance = game_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        root_widget.add_widget(&game_ui_layout);

        self._game_ui_layout = game_ui_layout.as_ref();
        self._key_binding_widget_manager = Some(Box::new(KeyBindingWidgetManager::default()));
        self._player_hud = Some(Box::new(PlayerHud::create_player_hud(game_ui_layout_mut)));
        self._item_bar_widget = Some(Box::new(ItemBarWidget::create_item_bar_widget(
            self._key_binding_widget_manager.as_ref().unwrap().as_ref(),
            game_ui_layout_mut,
            window_size,
        )));
        self._target_status_bar = Some(Box::new(TargetStatusWidget::create_target_status_widget(
            game_ui_layout_mut,
        )));
        self._toolbox_widget = Some(Box::new(ToolboxWidget::create_toolbox_widget(game_ui_layout_mut)));
        self._cooking_widget = Some(Box::new(CookingWidget::create_cooking_widget(game_ui_layout_mut)));
        self._world_map_widget = Some(WorldMapWidget::create_world_map_widget(game_ui_layout_mut, window_size));
        self._time_of_day = Some(Box::new(TimeOfDayWidget::create_time_of_day_widget(game_ui_layout_mut)));
        self._controller_help_widget = Some(Box::new(ControllerHelpWidget::create_controller_help_widget(
            self._key_binding_widget_manager.as_ref().unwrap().as_ref(),
            game_ui_layout_mut,
            window_size,
        )));
        self._item_acquire_notification_widget = Some(ItemAcquireNotificationWidget::create(game_ui_layout_mut));
        self._quest_widget = Some(Box::new(QuestWidget::create_quest_widget(game_ui_layout_mut)));

        // test box
        self._text_box_widget = Some(Box::new(TextBoxWidget::create_text_box_widget(root_widget)));

        // game menu layer
        let game_menu_layout = UIManager::create_widget("game menu layout", UIWidgetTypes::Default);
        let game_menu_layout_mut: &mut WidgetDefault = ptr_as_mut(game_menu_layout.as_ref());
        let ui_component: &mut UIComponentInstance = game_menu_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        root_widget.add_widget(&game_menu_layout);

        self._game_menu_widget = Some(GameMenuWidget::create_game_menu_widget(game_menu_layout_mut));

        // root layer
        self._cross_hair = Some(Box::new(CrossHairWidget::create_cross_hair(root_widget)));
        self._game_image = Some(ImageLayout::create_image_layout(
            root_widget,
            window_size,
            MATERIAL_INTRO_IMAGE,
        ));
        unsafe {
            if DEVELOPMENT {
                self._debug_ui_widget = Some(DebugUIWidget::create_debug_ui_widget(root_widget));
                self._debug_ui_widget.as_mut().unwrap().set_enable(false);
            }
        }

        self.set_cross_hair_visible(false);
    }

    pub fn clear_player_records(&mut self) {
        self._player_records.reset();
    }

    pub fn clear_game_ui(&mut self) {
        self.clear_inventory_items();
        self.clear_quests();
        self.clear_text_box_widgets();
        self.set_controls_visibility(true);
        self.clear_player_records();
    }

    pub fn get_game_ui_layout(&self) -> *const WidgetDefault<'a> {
        self._game_ui_layout
    }

    pub fn show_game_ui(&mut self, show: bool) {
        if !self._game_ui_layout.is_null() {
            let game_ui_layout_mut = ptr_as_mut(self._game_ui_layout);
            game_ui_layout_mut.get_ui_component_mut().set_visible(show);
        }
        self._text_box_widget.as_mut().unwrap().set_text_box_layer_visible(TextBoxLayerType::GamePlayLayer, show);
    }

    // game image widget
    pub fn is_done_manual_fade_out(&self) -> bool {
        self._game_image.as_ref().unwrap().is_done_manual_fade_out()
    }

    pub fn is_done_game_image_progress(&self) -> bool {
        self._game_image.as_ref().unwrap().is_done_game_image_progress()
    }

    pub fn set_auto_fade_inout(&mut self, auto_fade_inout: bool) {
        self._game_image.as_mut().unwrap().set_auto_fade_inout(auto_fade_inout);
    }

    pub fn set_image_manual_fade_inout(&mut self, material_instance_name: &str, fadeout_time: f32) {
        self.set_game_image(material_instance_name, fadeout_time, false)
    }

    pub fn set_image_auto_fade_inout(&mut self, material_instance_name: &str, fadeout_time: f32) {
        self.set_game_image(material_instance_name, fadeout_time, true)
    }

    pub fn set_game_image(&mut self, material_instance_name: &str, fadeout_time: f32, auto_fade_inout: bool) {
        let material_instance = if get_engine_resources().has_material_instance_data(material_instance_name) {
            Some(get_engine_resources().get_material_instance_data(material_instance_name).clone())
        } else {
            None
        };

        self._game_image.as_mut().unwrap().set_game_image(material_instance, fadeout_time, auto_fade_inout);
    }

    pub fn set_game_image_fade_speed(&mut self, fade_speed: f32) {
        self._game_image.as_mut().unwrap().set_game_image_fade_speed(fade_speed);
    }

    // debug text
    pub fn show_debug_ui(&mut self, enable: bool) {
        self._debug_ui_widget.as_mut().unwrap().set_enable(enable);
    }

    // game menu
    pub fn is_opened_game_debug_menu(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu() && game_menu_widget.get_active_tab() == GameMenuTab::DebugMenu
        } else {
            false
        }
    }
    pub fn open_game_debug_menu(&mut self) {
        self.open_game_menu(Some(GameMenuTab::DebugMenu));
    }
    pub fn update_game_debug_menu_widget(
        &mut self,
        _joystick_input_data: &JoystickInputData,
        _keyboard_input_data: &KeyboardInputData,
    ) {
    }

    // game menu
    pub fn get_inventory_widget_mut(&mut self) -> Option<&mut InventoryWidget<'a>> {
        self._game_menu_widget.as_mut().map(|menu| menu._inventory_widget.as_mut())
    }
    pub fn is_opened_game_menu(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu()
        } else {
            false
        }
    }
    pub fn open_game_menu(&mut self, tab: Option<GameMenuTab>) {
        if let Some(game_menu_widget) = self._game_menu_widget.as_mut() {
            game_menu_widget.open_game_menu(tab);
        }
        self.set_cross_hair_visible(true);
    }
    pub fn close_game_menu(&mut self) {
        if let Some(game_menu_widget) = self._game_menu_widget.as_mut() {
            game_menu_widget.close_game_menu();
        }
        self.set_cross_hair_visible(false);
    }
    pub fn update_game_menu_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        player: &RcRefCell<Character>,
    ) {
        if let Some(game_menu_widget) = self._game_menu_widget.as_mut() {
            game_menu_widget.update_game_menu_widget(
                time_data,
                joystick_input_data,
                keyboard_input_data,
                mouse_move_data,
                mouse_input_data,
                mouse_delta,
                player,
            );
        }
    }

    // cross-hair
    pub fn get_cross_hair_visible(&self) -> bool {
        if let Some(cross_hair) = self._cross_hair.as_ref() {
            return cross_hair.get_cross_hair_visible();
        }
        false
    }

    pub fn set_cross_hair_visible(&mut self, visible: bool) {
        if let Some(cross_hair) = self._cross_hair.as_mut() {
            cross_hair.update_cross_hair_visible(visible);
        }
    }

    // world map
    pub fn is_opened_world_map(&self) -> bool {
        self._world_map_widget.as_ref().unwrap().is_opened_world_map()
    }
    pub fn open_world_map(&mut self) {
        self._world_map_widget.as_mut().unwrap().open_world_map();
    }
    pub fn is_requested_close_world_map(&self) -> bool {
        self._world_map_widget.as_ref().unwrap().is_requested_close_world_map()
    }
    pub fn close_world_map(&mut self) {
        self._world_map_widget.as_mut().unwrap().close_world_map();
    }
    pub fn get_selected_world_map_stage_data_name(&self) -> &String {
        self._world_map_widget.as_ref().unwrap().get_selected_world_map_stage_data_name()
    }
    pub fn set_selected_world_map_stage(&mut self, selected_stage_name: &String) {
        self._world_map_widget.as_mut().unwrap().set_selected_world_map_stage(selected_stage_name);
    }
    pub fn unset_selected_world_map_stage(&mut self) {
        self._world_map_widget.as_mut().unwrap().set_selected_world_map_stage(&String::default());
    }
    pub fn update_world_map_widget(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if let Some(world_map_widget) = self._world_map_widget.as_mut() {
            world_map_widget.update_world_map(joystick_input_data, keyboard_input_data);
        }
    }

    // item bar
    pub fn get_item_count(&self, item_data_name: &str) -> usize {
        self._item_bar_widget.as_ref().unwrap().get_item_count(item_data_name)
    }

    pub fn add_item(&mut self, item_data_name: &str, item_count: usize, show_notification: bool) -> bool {
        let result = self._item_bar_widget.as_mut().unwrap().add_item(item_data_name, item_count);
        if result {
            self.notify_item_acquired(item_data_name, item_count as u32, show_notification);
        }
        result
    }

    pub fn get_player_records(&self) -> &PlayerRecords {
        &self._player_records
    }

    pub fn get_player_records_mut(&mut self) -> &mut PlayerRecords {
        &mut self._player_records
    }

    pub fn notify_item_acquired(&mut self, item_data_name: &str, amount: u32, show_notification: bool) {
        self._player_records.add_item_count(amount);

        let game_resources = get_game_resources();
        if game_resources.has_item_data(item_data_name) {
            let item_type = game_resources.get_item_data(item_data_name).borrow()._item_type;
            let item_type_str = format!("{:?}", item_type);
            self._player_records.add_item_type_count(&item_type_str, amount);
        }

        if item_data_name.contains("energy_ball") || item_data_name == ITEM_ENERGY_BALL {
            self._player_records.add_energy_balls(amount);
        } else if item_data_name.contains("spirit_ball") || item_data_name == ITEM_SPIRIT_BALL {
            self._player_records.add_spirit_balls(amount);
        }

        if show_notification {
            if let Some(notification_widget) = self._item_acquire_notification_widget.as_mut() {
                notification_widget.notify_item_acquired(item_data_name);
            }
        }
    }

    pub fn notify_item_crafted(&mut self) {
        self._player_records.add_craft_count(RECORD_DEFAULT_INCREMENT);
    }

    pub fn notify_player_died(&mut self) {
        self._player_records.add_death_count(RECORD_DEFAULT_INCREMENT);
    }

    pub fn notify_monster_killed(&mut self, monster_data_name: &str) {
        self._player_records.add_monster_kill(monster_data_name);
    }

    pub fn notify_tamed(&mut self) {
        self._player_records.add_taming_count(RECORD_DEFAULT_INCREMENT);
    }

    pub fn notify_friend_made(&mut self) {
        self._player_records.add_friend_count(RECORD_DEFAULT_INCREMENT);
    }

    pub fn notify_map_visited(&mut self, map_name: &str) {
        self._player_records.record_map_visit(map_name);
    }

    pub fn get_inventory_item_create_infos(&self) -> InventoryItemCreateInfoList {
        if let Some(item_bar_widget) = self._item_bar_widget.as_ref() {
            item_bar_widget.get_inventory_item_create_infos()
        } else {
            Default::default()
        }
    }

    pub fn find_eatable_inventory_item_data_name(&self) -> Option<String> {
        let item_create_info_rows = self.get_inventory_item_create_infos();
        let game_resources = get_game_resources();
        for (_row, items) in item_create_info_rows {
            for info in items {
                if info._item_count > 0 {
                    let item_data = game_resources.get_item_data(&info._item_data_name);
                    if item_data.borrow()._item_type.is_eatable() {
                        return Some(info._item_data_name);
                    }
                }
            }
        }
        None
    }

    pub fn clear_inventory_items(&mut self) {
        if let Some(item_bar_widget) = self._item_bar_widget.as_mut() {
            item_bar_widget.clear_item_bar_widget();
        }
    }

    pub fn remove_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        self._item_bar_widget.as_mut().unwrap().remove_item(item_data_name, item_count)
    }

    pub fn get_selected_inventory_item_data_name(&self) -> &str {
        self._item_bar_widget.as_ref().unwrap().get_selected_item_data_name()
    }

    pub fn get_selected_inventory_item_name(&self) -> &str {
        self._item_bar_widget.as_ref().unwrap().get_selected_item_name()
    }

    pub fn get_selected_inventory_item_data_type(&self) -> ItemDataType {
        self._item_bar_widget.as_ref().unwrap().get_selected_item_data_type()
    }

    pub fn get_selected_inventory_item_index(&self) -> usize {
        self._item_bar_widget.as_ref().unwrap().get_selected_item_index()
    }

    pub fn select_next_item(&mut self) {
        self._item_bar_widget.as_mut().unwrap().select_next_item()
    }

    pub fn select_previous_item(&mut self) {
        self._item_bar_widget.as_mut().unwrap().select_previous_item()
    }

    pub fn select_item(&mut self, item_index: usize) {
        self._item_bar_widget.as_mut().unwrap().select_item(item_index);
        if let Some(inventory_widget) = self.get_inventory_widget_mut() {
            inventory_widget.refresh_inventory_widget();
        }
    }

    pub fn select_quick_slot(&mut self, quick_index: usize) {
        if let Some(item_bar_widget) = self._item_bar_widget.as_mut() {
            item_bar_widget.select_quick_slot(quick_index);
        }
        if let Some(inventory_widget) = self.get_inventory_widget_mut() {
            inventory_widget.refresh_inventory_widget();
        }
    }

    pub fn switch_quick_slot_row(&mut self) {
        if let Some(item_bar_widget) = self._item_bar_widget.as_mut() {
            item_bar_widget.switch_active_row();
        }
        if let Some(inventory_widget) = self.get_inventory_widget_mut() {
            inventory_widget.refresh_inventory_widget();
        }
    }

    pub fn add_item_at_slot(&mut self, slot_index: usize, item_data_name: &str, item_count: usize) -> bool {
        if let Some(item_bar) = self._item_bar_widget.as_mut() {
            let result = item_bar.add_item_at_slot(slot_index, item_data_name, item_count);
            if let Some(inventory_widget) = self.get_inventory_widget_mut() {
                inventory_widget.refresh_inventory_widget();
            }
            result
        } else {
            false
        }
    }

    pub fn get_selected_quick_slot_row_col(&self) -> Option<(usize, usize)> {
        if let Some(item_bar) = self._item_bar_widget.as_ref() {
            item_bar.get_selected_quick_slot_row_col()
        } else {
            None
        }
    }

    pub fn swap_inventory_slots(&mut self, src_slot_index: usize, dst_slot_index: usize) -> bool {
        let result = if let Some(item_bar_widget) = self._item_bar_widget.as_mut() {
            item_bar_widget.swap_inventory_slots(src_slot_index, dst_slot_index)
        } else {
            false
        };
        if let Some(inventory_widget) = self.get_inventory_widget_mut() {
            inventory_widget.refresh_inventory_widget();
        }
        result
    }

    pub fn get_inventory_rows(&self) -> usize {
        if let Some(item_bar) = self._item_bar_widget.as_ref() {
            item_bar.get_inventory_rows()
        } else {
            2
        }
    }

    pub fn set_inventory_rows(&mut self, rows: usize) {
        if let Some(item_bar) = self._item_bar_widget.as_mut() {
            item_bar.set_inventory_rows(rows);
            if let Some(inventory_widget) = self.get_inventory_widget_mut() {
                inventory_widget.refresh_inventory_widget();
            }
        }
    }

    // inventory
    pub fn open_inventory(&mut self) {
        self.open_game_menu(Some(GameMenuTab::Inventory));
    }

    pub fn close_inventory(&mut self) {
        self.close_game_menu();
    }

    pub fn is_opened_inventory(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu() && game_menu_widget.get_active_tab() == GameMenuTab::Inventory
        } else {
            false
        }
    }

    // taming
    pub fn open_taming_menu(&mut self) {
        self.open_game_menu(Some(GameMenuTab::TamingList));
    }

    pub fn close_taming_menu(&mut self) {
        self.close_game_menu();
    }

    pub fn is_opened_taming_menu(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu() && game_menu_widget.get_active_tab() == GameMenuTab::TamingList
        } else {
            false
        }
    }

    // friendly npc
    pub fn open_friendly_npc_menu(&mut self) {
        self.open_game_menu(Some(GameMenuTab::FriendlyNpcList));
    }

    pub fn close_friendly_npc_menu(&mut self) {
        self.close_game_menu();
    }

    pub fn is_opened_friendly_npc_menu(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu() && game_menu_widget.get_active_tab() == GameMenuTab::FriendlyNpcList
        } else {
            false
        }
    }

    // quest
    pub fn clear_quests(&mut self) {
        if let Some(quest_widget) = self._quest_widget.as_mut() {
            quest_widget.clear_quests();
        }
    }

    pub fn add_quest(&mut self, title: Option<String>) -> RcRefCell<QuestTitle<'a>> {
        self._quest_widget.as_mut().unwrap().add_quest(title)
    }

    pub fn load_quest(&mut self, title: Option<String>) -> RcRefCell<QuestTitle<'a>> {
        self._quest_widget.as_mut().unwrap().load_quest(title)
    }

    // text box
    pub fn set_text_box_visible(&mut self, visible: bool) {
        self._text_box_widget.as_mut().unwrap().set_text_box_visible(visible);
    }

    pub fn has_text_box_item(&self, key: *const c_void) -> bool {
        self._text_box_widget.as_ref().unwrap().has_text_box_item(key)
    }

    pub fn add_text_box_item(
        &mut self,
        actor: ActorWrapper<'a>,
        contents: &Vec<TextBoxContent>,
        option: &TextBoxItemOption,
    ) {
        self._text_box_widget.as_mut().unwrap().add_text_box_item(actor, contents, option);
    }

    pub fn remove_text_box_item(&mut self, key: *const c_void) {
        self._text_box_widget.as_mut().unwrap().remove_text_box_item(key);
    }

    pub fn clear_text_box_widgets(&mut self) {
        if let Some(text_box_widget) = self._text_box_widget.as_mut() {
            text_box_widget.clear_text_box_widget();
        }
    }

    // toolbox
    pub fn open_toolbox(&mut self) {
        self._toolbox_widget.as_mut().unwrap().open_toolbox();
        self.set_cross_hair_visible(true);
    }
    pub fn close_toolbox(&mut self) {
        self.set_cross_hair_visible(false);
        self._toolbox_widget.as_mut().unwrap().close_toolbox();
    }
    pub fn is_opened_toolbox(&self) -> bool {
        self._toolbox_widget.as_ref().unwrap().is_opened_toolbox()
    }
    pub fn get_unlocked_toolbox_items(&self) -> HashSet<String> {
        if let Some(toolbox_widget) = self._toolbox_widget.as_ref() {
            toolbox_widget.get_unlocked_items()
        } else {
            HashSet::new()
        }
    }
    pub fn load_unlocked_toolbox_items(&mut self, unlocked_set: &HashSet<String>) {
        if let Some(toolbox_widget) = self._toolbox_widget.as_mut() {
            toolbox_widget.load_unlocked_items(unlocked_set);
        }
    }
    pub fn get_last_opened_toolbox_tab(&self) -> String {
        if let Some(toolbox_widget) = self._toolbox_widget.as_ref() {
            toolbox_widget.get_last_opened_tab()
        } else {
            "".to_string()
        }
    }
    pub fn set_last_opened_toolbox_tab(&mut self, tab_name: &str) {
        if let Some(toolbox_widget) = self._toolbox_widget.as_mut() {
            toolbox_widget.set_last_opened_tab(tab_name);
        }
    }
    pub fn update_toolbox_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        player: &RcRefCell<Character>,
    ) {
        if let Some(toolbox_widget) = self._toolbox_widget.as_mut() {
            toolbox_widget.update_toolbox_widget(
                time_data,
                joystick_input_data,
                keyboard_input_data,
                mouse_move_data,
                mouse_input_data,
                mouse_delta,
                player,
            );
        }
    }

    // cooking widget
    pub fn open_cooking(&mut self) {
        self._cooking_widget.as_mut().unwrap().open_cooking();
        self.set_cross_hair_visible(true);
    }
    pub fn close_cooking(&mut self) {
        self.set_cross_hair_visible(false);
        self._cooking_widget.as_mut().unwrap().close_cooking();
    }
    pub fn is_opened_cooking(&self) -> bool {
        self._cooking_widget.as_ref().unwrap().is_opened_cooking()
    }
    pub fn update_cooking_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        player: &RcRefCell<Character>,
    ) {
        if let Some(cooking_widget) = self._cooking_widget.as_mut() {
            cooking_widget.update_cooking_widget(
                time_data,
                joystick_input_data,
                keyboard_input_data,
                mouse_move_data,
                mouse_input_data,
                mouse_delta,
                player,
            );
        }
    }

    // craft widget
    pub fn open_craft(&mut self) {
        self.open_game_menu(Some(GameMenuTab::Craft));
    }
    pub fn close_craft(&mut self) {
        self.close_game_menu();
    }
    pub fn is_opened_craft(&self) -> bool {
        if let Some(game_menu_widget) = self._game_menu_widget.as_ref() {
            game_menu_widget.is_opened_game_menu() && game_menu_widget.get_active_tab() == GameMenuTab::Craft
        } else {
            false
        }
    }
    pub fn update_craft_widget(
        &mut self,
        _time_data: &TimeData,
        _joystick_input_data: &JoystickInputData,
        _keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character>,
    ) {
    }

    // controller help widget
    pub fn get_controls_visibility(&self) -> bool {
        if let Some(controller_help_widget) = self._controller_help_widget.as_ref() {
            controller_help_widget.get_controls_visibility()
        } else {
            true
        }
    }

    pub fn set_controls_visibility(&mut self, is_visible: bool) {
        if let Some(controller_help_widget) = self._controller_help_widget.as_mut() {
            controller_help_widget.set_controls_visibility(is_visible);
        }
    }

    pub fn toggle_controls_visibility(&mut self) {
        if let Some(controller_help_widget) = self._controller_help_widget.as_mut() {
            controller_help_widget.toggle_controls_visibility();
        }
    }

    // game ui manager
    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        log::info!("GameUIComponents::changed_window_size: {:?}", window_size);

        self._game_image.as_mut().unwrap().changed_window_size(window_size);
        self._player_hud.as_mut().unwrap().changed_window_size(window_size);
        self._controller_help_widget.as_mut().unwrap().changed_window_size(window_size);
        self._text_box_widget.as_mut().unwrap().changed_window_size(window_size);
        self._target_status_bar.as_mut().unwrap().changed_window_size(window_size);
        self._time_of_day.as_mut().unwrap().changed_window_size(window_size);
        self._item_bar_widget.as_mut().unwrap().changed_window_size(window_size);
        self._game_menu_widget.as_mut().unwrap().changed_window_size(window_size);
    }

    pub fn update_game_ui(&mut self, delta_time: f64) {
        let ui_manager = get_ui_manager();
        let engine_core = get_engine_core();
        let mouse_pos = &engine_core._mouse_move_data._mouse_pos;

        // changed window size
        if self._need_to_refresh || self._window_size != ui_manager._window_size {
            self._window_size = ui_manager._window_size;
            self.changed_window_size(&ui_manager._window_size);
            self._need_to_refresh = false;
        }

        if let Some(key_binding_widget_manager) = self._key_binding_widget_manager.as_mut() {
            key_binding_widget_manager.update_key_binding_widget_manager(engine_core.is_keyboard_input_mode());
        }

        if let Some(cross_hair) = self._cross_hair.as_mut()
            && cross_hair.get_cross_hair_visible()
        {
            cross_hair.update_cross_hair(mouse_pos);
        }

        if let Some(game_image) = self._game_image.as_mut() {
            game_image.update_game_image(delta_time, false);
        }

        if let Some(player_hud) = self._player_hud.as_mut()
            && get_character_manager().is_valid_player()
        {
            let player = get_character_manager().get_player().borrow();
            player_hud.update_status_widget(&player, delta_time);
        }

        if let Some(target_status_bar) = self._target_status_bar.as_mut() {
            if get_character_manager().is_valid_target_character() {
                let target = get_character_manager().get_target_character().borrow();
                target_status_bar.update_status_widget(&target, delta_time);
            } else {
                target_status_bar.fade_out_status_widget();
            }
        }

        if let Some(controller_help_widget) = self._controller_help_widget.as_mut() {
            controller_help_widget.update_controller_help_widget();
        }

        if let Some(item_bar_widget) = self._item_bar_widget.as_mut() {
            item_bar_widget.update_item_bar_widget();
        }

        if let Some(notification_widget) = self._item_acquire_notification_widget.as_mut() {
            notification_widget.update(delta_time as f32);
        }

        if let Some(text_box_widget) = self._text_box_widget.as_mut() {
            text_box_widget.update_text_box_widget(delta_time as f32);
        }

        if let Some(time_of_day) = self._time_of_day.as_mut() {
            time_of_day.update_time_of_day_widget();
        }

        if let Some(quest_widget) = self._quest_widget.as_mut() {
            quest_widget.update_quest_widget(delta_time as f32);
        }

        if let Some(debug_ui_widget) = self._debug_ui_widget.as_mut() {
            debug_ui_widget.update_debug_ui_widget();
        }
    }

    pub fn trigger_stamina_warning(&mut self) {
        if let Some(player_hud) = self._player_hud.as_mut() {
            player_hud.trigger_stamina_warning();
        }
    }
}
