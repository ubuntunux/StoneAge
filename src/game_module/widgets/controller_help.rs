use std::collections::HashMap;
use std::ffi::c_void;
use nalgebra::Vector2;
use strum_macros::{Display, EnumIter};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController, KeyBindingType};
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::item_bar_widget::{ItemBarWidget};

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_PADDING: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (280.0, 460.0);
const KEY_BINDING_UI_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 30.0;
const TEXT_WIDGET_MARGIN: f32 = 20.0;
const ICON_WIDGET_MARGIN: f32 = -14.0;

#[derive(Hash, Eq, Clone, Copy, Debug, EnumIter, Display, PartialEq)]
pub enum KeyBindingGroup {
    PlayerControl,
    WorldMapControl,
    Inventory,
    Interaction,
    QuickSlot,
    WrapUpTheDayControl,
}

pub struct ControllerHelpWidget<'a> {
    pub _character_controller_help_widget: *const WidgetDefault<'a>,
    pub _keyboard_material_instance_map: HashMap<KeyBindingType, Option<Vec<RcRefCell<MaterialInstanceData<'a>>>>>,
    pub _joystick_material_instance_map: HashMap<KeyBindingType, Option<Vec<RcRefCell<MaterialInstanceData<'a>>>>>,
    pub _key_binding_widget_map: HashMap<KeyBindingType, KeyBindingWidget<'a>>,
    pub _is_keyboard_input_mode: bool,
    pub _last_interaction_object_key: *const c_void,
    pub _window_size: Vector2<i32>,
    pub _selected_item_index: usize,
}

pub struct KeyBindingWidget<'a> {
    pub _key_binding_type: KeyBindingType,
    pub _key_binding_group: KeyBindingGroup,
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _binding_name_widget: *const WidgetDefault<'a>,
    pub _binding_icon_widgets: Vec<*const WidgetDefault<'a>>
}

impl<'a> KeyBindingWidget<'a> {
    pub fn update_icon_material_instance(&mut self, material_instance_data: Option<Vec<RcRefCell<MaterialInstanceData<'a>>>>) {
        ptr_as_mut(self._layout_widget)._ui_component.set_visible(material_instance_data.is_some());
        let material_count = if let Some(material_instance_data_list) = material_instance_data.as_ref() {
            material_instance_data_list.len()
        } else {
            0
        };

        for (index, icon_widget) in self._binding_icon_widgets.iter().enumerate() {
            if index < material_count {
                ptr_as_mut(*icon_widget)._ui_component.set_visible(true);
                ptr_as_mut(*icon_widget)._ui_component.set_material_instance(
                    material_instance_data.as_ref().unwrap().get(index).map(|material_instance_data| material_instance_data.clone())
                );
            } else {
                ptr_as_mut(*icon_widget)._ui_component.set_visible(false);
            }
        }
    }

    pub fn create_player_control_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        widget_count: usize,
        key_binding_type: KeyBindingType,
        widget_name: &str,
        key_binding_text: &str
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        parent_widget.add_widget(&layout_widget);

        // icons
        let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
        for _ in 0..widget_count {
            let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
            ui_component.set_halign(HorizontalAlign::RIGHT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_margin_right(ICON_WIDGET_MARGIN);
            ui_component.set_size_x(KEY_BINDING_UI_SIZE);
            ui_component.set_size_y(KEY_BINDING_UI_SIZE);
            layout_widget_mut.add_widget(&binding_icon_widget);
            binding_icon_widgets.push(binding_icon_widget.as_ref());
        }

        // text widget
        let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_margin_left(TEXT_WIDGET_MARGIN);
        ui_component.set_margin_right(TEXT_WIDGET_MARGIN);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text(key_binding_text);
        layout_widget_mut.add_widget(&binding_name_widget);

        KeyBindingWidget {
            _key_binding_type: key_binding_type,
            _key_binding_group: KeyBindingGroup::PlayerControl,
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: binding_name_widget.as_ref(),
            _binding_icon_widgets: binding_icon_widgets
        }
    }

    pub fn create_inventory_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        key_binding_type: KeyBindingType,
        widget_name: &str,
        key_binding_text: &str
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        parent_widget.add_widget(&layout_widget);

        // icons
        let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
        let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_right(ICON_WIDGET_MARGIN);
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        layout_widget_mut.add_widget(&binding_icon_widget);
        binding_icon_widgets.push(binding_icon_widget.as_ref());

        // text widget
        let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_margin_left(TEXT_WIDGET_MARGIN);
        ui_component.set_margin_right(TEXT_WIDGET_MARGIN);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text(key_binding_text);
        layout_widget_mut.add_widget(&binding_name_widget);

        KeyBindingWidget {
            _key_binding_type: key_binding_type,
            _key_binding_group: KeyBindingGroup::Inventory,
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: binding_name_widget.as_ref(),
            _binding_icon_widgets: binding_icon_widgets
        }
    }

    pub fn create_interaction_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        key_binding_type:  KeyBindingType,
        widget_name: &str,
        key_binding_text: &str
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        parent_widget.add_widget(&layout_widget);

        // icons
        let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
        let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_right(ICON_WIDGET_MARGIN);
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        layout_widget_mut.add_widget(&binding_icon_widget);
        binding_icon_widgets.push(binding_icon_widget.as_ref());

        // text widget
        let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_margin_left(TEXT_WIDGET_MARGIN);
        ui_component.set_margin_right(TEXT_WIDGET_MARGIN);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text(key_binding_text);
        layout_widget_mut.add_widget(&binding_name_widget);

        KeyBindingWidget {
            _key_binding_type: key_binding_type,
            _key_binding_group: KeyBindingGroup::Interaction,
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: binding_name_widget.as_ref(),
            _binding_icon_widgets: binding_icon_widgets
        }
    }

    pub fn create_quick_slot_key_binding_widget(parent_widget: &mut WidgetDefault<'a>, key_binding_type: KeyBindingType, widget_name: &str) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Top(1.0));
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        parent_widget.add_widget(&layout_widget);

        // icons
        let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
        let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        layout_widget_mut.add_widget(&binding_icon_widget);
        binding_icon_widgets.push(binding_icon_widget.as_ref());

        KeyBindingWidget {
            _key_binding_type: key_binding_type,
            _key_binding_group: KeyBindingGroup::QuickSlot,
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: std::ptr::null(),
            _binding_icon_widgets: binding_icon_widgets
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layout_widget).get_ui_component_mut();
        if self._key_binding_group == KeyBindingGroup::Inventory {
            if self._key_binding_type == KeyBindingType::SelectPrevItem {
                ui_component.set_pos_x((window_size.x as f32 - ItemBarWidget::get_item_bar_width()) * 0.5 - ui_component.get_ui_size().x - TEXT_WIDGET_MARGIN);
                ui_component.set_pos_y(window_size.y as f32 - (ItemBarWidget::get_item_bar_center_y() + ui_component.get_ui_size().y * 0.5));
            } else if self._key_binding_type == KeyBindingType::SelectNextItem {
                ui_component.set_pos_x((window_size.x as f32 + ItemBarWidget::get_item_bar_width()) * 0.5 + TEXT_WIDGET_MARGIN);
                ui_component.set_pos_y(window_size.y as f32 - (ItemBarWidget::get_item_bar_center_y() + ui_component.get_ui_size().y * 0.5));
            } else if self._key_binding_type == KeyBindingType::UseItem {
                ui_component.set_pos_y(window_size.y as f32 - (ItemBarWidget::get_item_bar_pos_top() + ui_component.get_ui_size().y));
            } else if self._key_binding_type == KeyBindingType::DropItem {
                ui_component.set_pos_y(window_size.y as f32 - (ItemBarWidget::get_item_bar_pos_top() + ui_component.get_ui_size().y * 2.0));
            } else {
                assert!(false, "not implemented: {:?}", self._key_binding_type);
            }
        }
    }
}

impl<'a> ControllerHelpWidget<'a> {
    pub fn create_controller_help_widget(
        root_widget: &mut WidgetDefault<'a>,
        item_bar_widget: &ItemBarWidget<'a>,
        game_resources: &GameResources<'a>,
        window_size: &Vector2<i32>
    ) -> ControllerHelpWidget<'a> {
        let engine_resources = game_resources.get_engine_resources();

        // character_controller_help_widget
        let character_controller_help_widget = UIManager::create_widget("character_controller_help_widget", UIWidgetTypes::Default);
        let character_controller_help_widget_mut = ptr_as_mut(character_controller_help_widget.as_ref());
        let ui_component = ptr_as_mut(character_controller_help_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_round(15.0);
        ui_component.set_size(MAIN_LAYOUT_SIZE.0, MAIN_LAYOUT_SIZE.1);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&character_controller_help_widget);

        // player control ui
        let camera_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::CameraRotation, "view_key_binding", "View");
        let move_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 4, KeyBindingType::Move, "move_key_binding", "Move");
        let zoom_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::Zoom, "zoom_key_binding", "Zoom");
        let attack_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::Attack, "attack_key_binding", "Attack");
        let power_attack_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::PowerAttack, "power_attack_key_binding", "Power Attack");
        let sprint_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::Sprint, "sprint_key_binding", "Sprint");
        let jump_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::Jump, "jump_key_binding", "Jump");
        let roll_key_binding_widget = KeyBindingWidget::create_player_control_key_binding_widget(character_controller_help_widget_mut, 1, KeyBindingType::Roll, "roll_key_binding", "Roll");

        // inventory
        let select_prev_item_key_binding_widget = KeyBindingWidget::create_inventory_key_binding_widget(root_widget, KeyBindingType::SelectPrevItem, "select_prev_item_key_binding", "Previous Item");
        let select_next_item_key_binding_widget = KeyBindingWidget::create_inventory_key_binding_widget(root_widget, KeyBindingType::SelectNextItem, "select_next_item_key_binding", "Next Item");
        let drop_item_key_binding_widget = KeyBindingWidget::create_inventory_key_binding_widget(root_widget, KeyBindingType::DropItem, "drop_item_key_binding", "Drop Item");
        let use_item_key_binding_widget = KeyBindingWidget::create_inventory_key_binding_widget(root_widget, KeyBindingType::UseItem, "use_item_key_binding", "Use Item");

        // interaction
        let interaction_key_binding_widget = KeyBindingWidget::create_interaction_key_binding_widget(root_widget, KeyBindingType::Interaction, "interaction_key_binding", "Interaction");
        let enter_gate_key_binding_widget = KeyBindingWidget::create_interaction_key_binding_widget(root_widget, KeyBindingType::EnterGate, "enter_gate_key_binding", "Enter");
        let gathering_key_binding_widget = KeyBindingWidget::create_interaction_key_binding_widget(root_widget, KeyBindingType::Gathering, "gathering_key_binding", "Gathering");

        // quick slot
        let select_item01_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(0)._widget), KeyBindingType::SelectItem01, "select_item01_key_binding");
        let select_item02_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(1)._widget), KeyBindingType::SelectItem02, "select_item02_key_binding");
        let select_item03_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(2)._widget), KeyBindingType::SelectItem03, "select_item03_key_binding");
        let select_item04_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(3)._widget), KeyBindingType::SelectItem04, "select_item04_key_binding");
        let select_item05_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(4)._widget), KeyBindingType::SelectItem05, "select_item05_key_binding");
        let select_item06_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(5)._widget), KeyBindingType::SelectItem06, "select_item06_key_binding");
        let select_item07_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(6)._widget), KeyBindingType::SelectItem07, "select_item07_key_binding");
        let select_item08_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(7)._widget), KeyBindingType::SelectItem08, "select_item08_key_binding");
        let select_item09_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(8)._widget), KeyBindingType::SelectItem09, "select_item09_key_binding");
        let select_item10_key_binding_widget = KeyBindingWidget::create_quick_slot_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(9)._widget), KeyBindingType::SelectItem10, "select_item10_key_binding");

        let mut character_controller_help_widget = ControllerHelpWidget {
            _character_controller_help_widget: character_controller_help_widget_mut,
            _keyboard_material_instance_map: HashMap::from([
                (KeyBindingType::Attack, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_l").clone()])),
                (KeyBindingType::PowerAttack, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_r").clone()])),
                (KeyBindingType::Interaction, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()])),
                (KeyBindingType::EnterGate, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_w").clone()])),
                (KeyBindingType::Gathering, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_l").clone()])),
                (KeyBindingType::CameraRotation, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse").clone()])),
                (KeyBindingType::Zoom, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_m").clone()])),
                (KeyBindingType::Move, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/keycode_a").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_s").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_d").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_w").clone(),
                ])),
                (KeyBindingType::Sprint, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_shift").clone()])),
                (KeyBindingType::Jump, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_space").clone()])),
                (KeyBindingType::Roll, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_alt").clone()])),
                (KeyBindingType::SelectPrevItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_q").clone()])),
                (KeyBindingType::SelectNextItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_e").clone()])),               (KeyBindingType::DropItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()])),
                (KeyBindingType::UseItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_c").clone()])),
                (KeyBindingType::SelectItem01, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_1").clone()])),
                (KeyBindingType::SelectItem02, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_2").clone()])),
                (KeyBindingType::SelectItem03, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_3").clone()])),
                (KeyBindingType::SelectItem04, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_4").clone()])),
                (KeyBindingType::SelectItem05, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_5").clone()])),
                (KeyBindingType::SelectItem06, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_6").clone()])),
                (KeyBindingType::SelectItem07, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_7").clone()])),
                (KeyBindingType::SelectItem08, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_8").clone()])),
                (KeyBindingType::SelectItem09, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_9").clone()])),
                (KeyBindingType::SelectItem10, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_0").clone()]))
            ]),
            _joystick_material_instance_map: HashMap::from([
                (KeyBindingType::Attack, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rb").clone()])),
                (KeyBindingType::PowerAttack, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rt").clone()])),
                (KeyBindingType::Interaction, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()])),
                (KeyBindingType::EnterGate, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_r_stick").clone()])),
                (KeyBindingType::Gathering, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rb").clone()])),
                (KeyBindingType::CameraRotation, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_l_stick").clone()])),
                (KeyBindingType::Zoom, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/joystick_up").clone(),
                    engine_resources.get_material_instance_data("ui/controller/joystick_down").clone(),
                ])),
                (KeyBindingType::Move, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_r_stick").clone()])),
                (KeyBindingType::Sprint, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_lb").clone()])),
                (KeyBindingType::Jump, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_a").clone()])),
                (KeyBindingType::Roll, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_b").clone()])),
                (KeyBindingType::SelectPrevItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_left").clone()])),
                (KeyBindingType::SelectNextItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_right").clone()])),
                (KeyBindingType::DropItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()])),
                (KeyBindingType::UseItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_y").clone()])),
                (KeyBindingType::SelectItem01, None),
                (KeyBindingType::SelectItem02, None),
                (KeyBindingType::SelectItem03, None),
                (KeyBindingType::SelectItem04, None),
                (KeyBindingType::SelectItem05, None),
                (KeyBindingType::SelectItem06, None),
                (KeyBindingType::SelectItem07, None),
                (KeyBindingType::SelectItem08, None),
                (KeyBindingType::SelectItem09, None),
                (KeyBindingType::SelectItem10, None)
            ]),
            _key_binding_widget_map: HashMap::from([
                (attack_key_binding_widget._key_binding_type, attack_key_binding_widget),
                (power_attack_key_binding_widget._key_binding_type, power_attack_key_binding_widget),
                (interaction_key_binding_widget._key_binding_type, interaction_key_binding_widget),
                (enter_gate_key_binding_widget._key_binding_type, enter_gate_key_binding_widget),
                (gathering_key_binding_widget._key_binding_type, gathering_key_binding_widget),
                (camera_key_binding_widget._key_binding_type, camera_key_binding_widget),
                (zoom_key_binding_widget._key_binding_type, zoom_key_binding_widget),
                (move_key_binding_widget._key_binding_type, move_key_binding_widget),
                (sprint_key_binding_widget._key_binding_type, sprint_key_binding_widget),
                (jump_key_binding_widget._key_binding_type, jump_key_binding_widget),
                (roll_key_binding_widget._key_binding_type, roll_key_binding_widget),
                (select_prev_item_key_binding_widget._key_binding_type, select_prev_item_key_binding_widget),
                (select_next_item_key_binding_widget._key_binding_type, select_next_item_key_binding_widget),
                (drop_item_key_binding_widget._key_binding_type, drop_item_key_binding_widget),
                (use_item_key_binding_widget._key_binding_type, use_item_key_binding_widget),
                (select_item01_key_binding_widget._key_binding_type, select_item01_key_binding_widget),
                (select_item02_key_binding_widget._key_binding_type, select_item02_key_binding_widget),
                (select_item03_key_binding_widget._key_binding_type, select_item03_key_binding_widget),
                (select_item04_key_binding_widget._key_binding_type, select_item04_key_binding_widget),
                (select_item05_key_binding_widget._key_binding_type, select_item05_key_binding_widget),
                (select_item06_key_binding_widget._key_binding_type, select_item06_key_binding_widget),
                (select_item07_key_binding_widget._key_binding_type, select_item07_key_binding_widget),
                (select_item08_key_binding_widget._key_binding_type, select_item08_key_binding_widget),
                (select_item09_key_binding_widget._key_binding_type, select_item09_key_binding_widget),
                (select_item10_key_binding_widget._key_binding_type, select_item10_key_binding_widget)
            ]),
            _is_keyboard_input_mode: true,
            _last_interaction_object_key: std::ptr::null(),
            _window_size: window_size.clone(),
            _selected_item_index: usize::MAX,
        };

        character_controller_help_widget.update_key_binding_widgets( character_controller_help_widget._is_keyboard_input_mode );
        character_controller_help_widget
    }

    pub fn get_key_binding_widget(&self, key_binding_type: &KeyBindingType) -> &KeyBindingWidget<'a> {
        self._key_binding_widget_map.get(key_binding_type).unwrap()
    }

    pub fn changed_window_size(&mut self, game_scene_manager: &GameSceneManager, window_size: &Vector2<i32>) {
        self._window_size = window_size.clone();

        let ui_component = ptr_as_mut(self._character_controller_help_widget).get_ui_component_mut();
        ui_component.set_size_y(ui_component.get_num_children() as f32 * KEY_BINDING_UI_SIZE + MAIN_LAYOUT_PADDING * 2.0);
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_ui_size().x - MAIN_LAYOUT_MARGIN * 2.0);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_ui_size().y - MAIN_LAYOUT_MARGIN * 2.0);

        for key_binding_widget in self._key_binding_widget_map.values_mut() {
            key_binding_widget.changed_window_size(window_size);
        }

        self.update_selected_item_helper_widget(game_scene_manager, true);
    }

    pub fn update_custom_key_binding_widgets(&mut self, key_binding_widget_type: KeyBindingType, material_instance_type: KeyBindingType) {
        let material_instance_map = if self._is_keyboard_input_mode {
            &self._keyboard_material_instance_map
        } else {
            &self._joystick_material_instance_map
        };

        let key_binding_widget = self._key_binding_widget_map.get_mut(&key_binding_widget_type).unwrap();
        let material_instance_data: Option<Vec<RcRefCell<MaterialInstanceData<'a>>>> = material_instance_map.get(&material_instance_type).unwrap().clone();
        key_binding_widget.update_icon_material_instance(material_instance_data);
    }

    pub fn update_key_binding_widgets(&mut self, is_keyboard_input_mode: bool) {
        let material_instance_map = if is_keyboard_input_mode {
            &self._keyboard_material_instance_map
        } else {
            &self._joystick_material_instance_map
        };

        for (key_binding_type, key_binding_widget) in self._key_binding_widget_map.iter_mut() {
            let material_instance_data: Option<Vec<RcRefCell<MaterialInstanceData<'a>>>> = material_instance_map.get(key_binding_type).unwrap().clone();
            key_binding_widget.update_icon_material_instance(material_instance_data);
        }
    }

    pub fn update_interaction_widget(&mut self, game_scene_manager: &GameSceneManager) {
        let mut matched_key_binding_type = KeyBindingType::None;
        let mut interaction_name: String = String::new();
        let character_manager = game_scene_manager.get_character_manager();
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            if player.is_in_interaction_range() {
                let interaction_object = player.get_nearest_interaction_object();
                (matched_key_binding_type, interaction_name) = match interaction_object {
                    InteractionObject::PropBed(_) => (KeyBindingType::Interaction, String::from("Wrap up the day")),
                    InteractionObject::PropPickup(prop) => (KeyBindingType::Interaction, format!("Pick up a {}", prop.borrow()._prop_data.borrow()._name.as_str())),
                    InteractionObject::PropMonolith(_) => (KeyBindingType::Interaction, String::from("Open Toolbox")),
                    InteractionObject::PropTable(_) => (KeyBindingType::Interaction, String::from("Sit Down")),
                    InteractionObject::Npc(npc) => {
                        if player.get_attached_item_data_type().is_eatable() {
                            (KeyBindingType::Interaction, format!("Give a {} to {}", player.get_attached_item().as_ref().unwrap().borrow()._item_data.borrow()._name.as_str(), npc.borrow()._character_data.borrow()._name.as_str()))
                        } else {
                            (KeyBindingType::Interaction, format!("Interaction with {}", npc.borrow()._character_data.borrow()._name.as_str()))
                        }
                    },
                    InteractionObject::PropGate(_) => (KeyBindingType::None, String::from("Enter Gate")),
                    InteractionObject::PropGathering(prop) => (KeyBindingType::Gathering, format!("Hit the {}", prop.borrow()._prop_data.borrow()._name.as_str())),
                    _ => (KeyBindingType::Interaction, String::from("interaction"))
                };
            }
        }

        const INTERACTION_WIDGETS: [KeyBindingType; 3]= [KeyBindingType::Interaction, KeyBindingType::EnterGate, KeyBindingType::Gathering];
        for key_binding_type in INTERACTION_WIDGETS.iter() {
            let mut enable_interaction = true;
            if character_manager.is_valid_player() {
                let player = character_manager.get_player().borrow();
                if player.is_alive() == false ||
                    player.is_action(ActionAnimationState::Sleep) ||
                    player.is_action(ActionAnimationState::SleepNoSnoring) ||
                    player.is_action(ActionAnimationState::LayingDown) ||
                    player.is_action(ActionAnimationState::WakeUp) {
                    enable_interaction = false;
                }
            }

            let interaction_key_binding_widget = self._key_binding_widget_map.get(&key_binding_type).unwrap();
            let interaction_widget = ptr_as_mut(interaction_key_binding_widget._layout_widget);
            if enable_interaction && *key_binding_type == matched_key_binding_type {
                let player = character_manager.get_player().borrow();
                let interaction_object = player.get_nearest_interaction_object();
                let position = interaction_object.get_position();
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let screen_position = main_camera.convert_world_to_screen(&position, true);
                interaction_widget._ui_component.set_pos(screen_position.x, screen_position.y);
                interaction_widget._ui_component.set_visible(true);
                ptr_as_mut(interaction_key_binding_widget._binding_name_widget)._ui_component.set_text(interaction_name.as_str());
                self._last_interaction_object_key = interaction_object.get_key();
            } else {
                interaction_widget._ui_component.set_visible(false);
            }
        }
    }

    pub fn update_selected_item_helper_widget(&mut self, game_scene_manager: &GameSceneManager, force_update: bool) {
        let selected_item_index = game_scene_manager.get_game_ui_manager().get_selected_inventory_item_index();
        if self._selected_item_index != selected_item_index || force_update {
            let _item_name = game_scene_manager.get_game_ui_manager().get_selected_inventory_item_name();
            let pos_x = self._window_size.x as f32 * 0.5 + ItemBarWidget::get_selected_item_pos_left(selected_item_index);

            let use_item_widget = self.get_key_binding_widget(&KeyBindingType::UseItem);
            ptr_as_mut(use_item_widget._layout_widget).get_ui_component_mut().set_pos_x(pos_x);

            let drop_item_widget = self.get_key_binding_widget(&KeyBindingType::DropItem);
            ptr_as_mut(drop_item_widget._layout_widget).get_ui_component_mut().set_pos_x(pos_x);

            self._selected_item_index = selected_item_index;
        }
    }

    pub fn update_controller_help_widget(&mut self, game_scene_manager: &GameSceneManager, game_controller: &GameController) {
        let is_keyboard_input_mode = game_controller.is_keyboard_input_mode();
        if self._is_keyboard_input_mode != is_keyboard_input_mode {
            self.update_key_binding_widgets(is_keyboard_input_mode);
            self._is_keyboard_input_mode = is_keyboard_input_mode;
        }

        self.update_interaction_widget(game_scene_manager);

        self.update_selected_item_helper_widget(game_scene_manager, false);
    }
}
