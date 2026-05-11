use std::collections::HashMap;
use std::ffi::c_void;
use nalgebra::{Vector2};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController, InputControlType};
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::item_bar_widget::{ItemBarWidget};

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_PADDING: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (280.0, 460.0);
const ITEM_SIZE: f32 = 50.0;
const TEXT_WIDGET_WIDTH: f32 = 130.0;
const FONT_SIZE: f32 = 30.0;

pub struct ControllerHelpWidget<'a> {
    pub _character_controller_help_widget: *const WidgetDefault<'a>,
    pub _keyboard_material_instance_map: HashMap<InputControlType, Option<Vec<RcRefCell<MaterialInstanceData<'a>>>>>,
    pub _joystick_material_instance_map: HashMap<InputControlType, Option<Vec<RcRefCell<MaterialInstanceData<'a>>>>>,
    pub _key_binding_widget_map: HashMap<InputControlType, KeyBindingWidget<'a>>,
    pub _is_keyboard_input_mode: bool,
    pub _last_interaction_object_key: *const c_void,
}

pub struct KeyBindingWidget<'a> {
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _binding_name_widget: *const WidgetDefault<'a>,
    pub _binding_icon_widgets: Vec<*const WidgetDefault<'a>>
}

impl<'a> KeyBindingWidget<'a> {
    pub fn create_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        widget_count: usize,
        widget_name: &str,
        key_binding_text: &str,
        center_offset: Option<Vector2<f32>>,
        visible_frame: bool,
        text_widget_width: f32,
        text_widget_margin: f32,
        icon_widget_margin: f32
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        if let Some(center_offset) = center_offset {
            ui_component.set_halign(HorizontalAlign::CENTER);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_pos_hint_x(PosHintX::Center(center_offset.x));
            ui_component.set_pos_hint_y(PosHintY::Center(center_offset.y));
        }
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(0, 0, 0, if visible_frame { 128 } else { 0 }));
        parent_widget.add_widget(&layout_widget);

        let binding_name_widget = if key_binding_text.is_empty() {
            std::ptr::null()
        } else {
            let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
            ui_component.set_expandable_x(true);
            ui_component.set_size_x(text_widget_width);
            ui_component.set_size_y(ITEM_SIZE);
            ui_component.set_margin_left(text_widget_margin);
            ui_component.set_margin_right(text_widget_margin);
            ui_component.set_halign(HorizontalAlign::RIGHT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_font_size(FONT_SIZE);
            ui_component.set_font_color(get_color32(255, 255, 255, 255));
            ui_component.set_color(get_color32(255, 255, 255, 0));
            ui_component.set_text(key_binding_text);
            layout_widget_mut.add_widget(&binding_name_widget);
            ptr_as_ref(binding_name_widget.as_ref())
        };

        let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
        for _ in 0..widget_count {
            let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
            ui_component.set_halign(HorizontalAlign::LEFT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_margin_left(icon_widget_margin);
            ui_component.set_size_x(ITEM_SIZE);
            ui_component.set_size_y(ITEM_SIZE);
            layout_widget_mut.add_widget(&binding_icon_widget);
            binding_icon_widgets.push(binding_icon_widget.as_ref());
        }

        KeyBindingWidget {
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: binding_name_widget,
            _binding_icon_widgets: binding_icon_widgets
        }
    }

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
}
impl<'a> ControllerHelpWidget<'a> {
    pub fn create_controller_help_widget(
        root_widget: &mut WidgetDefault<'a>,
        item_bar_widget: &ItemBarWidget<'a>,
        game_resources: &GameResources<'a>,
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
        let camera_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "view_key_binding", "View", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let move_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 4, "move_key_binding", "Move", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let zoom_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 2, "zoom_key_binding", "Zoom", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "attack_key_binding", "Attack", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let power_attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "power_attack_key_binding", "Power Attack", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let sprint_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "sprint_key_binding", "Sprint", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let jump_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "jump_key_binding", "Jump", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let roll_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "roll_key_binding", "Roll", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);

        // inventory
        let select_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 2, "select_item_key_binding", "Select Item", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let drop_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "drop_item_key_binding", "Drop Item", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);
        let use_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, 1, "use_item_key_binding", "Use Item", None, false, TEXT_WIDGET_WIDTH, 20.0, -14.0);

        // interaction
        let interaction_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, 1, "interaction_key_binding", "Interaction", None, true, 0.0, 20.0, -14.0);
        let enter_gate_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, 1, "enter_gate_key_binding", "Enter", None, true, 0.0, 20.0, -14.0);
        let gathering_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, 1, "gathering_key_binding", "Gathering", None, true, 0.0, 20.0, -14.0);

        // quick slot
        let use_item01_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(0)._widget), 1, "use_item01_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item02_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(1)._widget), 1, "use_item02_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item03_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(2)._widget), 1, "use_item03_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item04_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(3)._widget), 1, "use_item04_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item05_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(4)._widget), 1, "use_item05_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item06_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(5)._widget), 1, "use_item06_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item07_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(6)._widget), 1, "use_item07_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item08_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(7)._widget), 1, "use_item08_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item09_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(8)._widget), 1, "use_item09_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);
        let use_item10_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(9)._widget), 1, "use_item10_key_binding", "", Some(Vector2::new(0.5, 1.3)), false, 0.0, 0.0, 0.0);

        let mut character_controller_help_widget = ControllerHelpWidget {
            _character_controller_help_widget: character_controller_help_widget_mut,
            _keyboard_material_instance_map: HashMap::from([
                (InputControlType::Attack, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_l").clone()])),
                (InputControlType::PowerAttack, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_r").clone()])),
                (InputControlType::Interaction, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()])),
                (InputControlType::EnterGate, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_w").clone()])),
                (InputControlType::Gathering, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_l").clone()])),
                (InputControlType::CameraRotation, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse").clone()])),
                (InputControlType::Zoom, Some(vec![engine_resources.get_material_instance_data("ui/controller/mouse_m").clone()])),
                (InputControlType::Move, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/keycode_a").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_s").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_d").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_w").clone(),
                ])),
                (InputControlType::Sprint, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_shift").clone()])),
                (InputControlType::Jump, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_space").clone()])),
                (InputControlType::Roll, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_alt").clone()])),
                (InputControlType::SelectItem, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/keycode_q").clone(),
                    engine_resources.get_material_instance_data("ui/controller/keycode_e").clone()
                ])),
                (InputControlType::DropItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()])),
                (InputControlType::UseItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_c").clone()])),
                (InputControlType::SelectItem01, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_1").clone()])),
                (InputControlType::SelectItem02, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_2").clone()])),
                (InputControlType::SelectItem03, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_3").clone()])),
                (InputControlType::SelectItem04, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_4").clone()])),
                (InputControlType::SelectItem05, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_5").clone()])),
                (InputControlType::SelectItem06, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_6").clone()])),
                (InputControlType::SelectItem07, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_7").clone()])),
                (InputControlType::SelectItem08, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_8").clone()])),
                (InputControlType::SelectItem09, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_9").clone()])),
                (InputControlType::SelectItem10, Some(vec![engine_resources.get_material_instance_data("ui/controller/keycode_0").clone()]))
            ]),
            _joystick_material_instance_map: HashMap::from([
                (InputControlType::Attack, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rb").clone()])),
                (InputControlType::PowerAttack, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rt").clone()])),
                (InputControlType::Interaction, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()])),
                (InputControlType::EnterGate, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_r_stick").clone()])),
                (InputControlType::Gathering, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_rb").clone()])),
                (InputControlType::CameraRotation, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_l_stick").clone()])),
                (InputControlType::Zoom, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/joystick_up").clone(),
                    engine_resources.get_material_instance_data("ui/controller/joystick_down").clone(),
                ])),
                (InputControlType::Move, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_r_stick").clone()])),
                (InputControlType::Sprint, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_lb").clone()])),
                (InputControlType::Jump, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_a").clone()])),
                (InputControlType::Roll, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_b").clone()])),
                (InputControlType::SelectItem, Some(vec![
                    engine_resources.get_material_instance_data("ui/controller/joystick_left").clone(),
                    engine_resources.get_material_instance_data("ui/controller/joystick_right").clone()
                ])),
                (InputControlType::DropItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()])),
                (InputControlType::UseItem, Some(vec![engine_resources.get_material_instance_data("ui/controller/joystick_y").clone()])),
                (InputControlType::SelectItem01, None),
                (InputControlType::SelectItem02, None),
                (InputControlType::SelectItem03, None),
                (InputControlType::SelectItem04, None),
                (InputControlType::SelectItem05, None),
                (InputControlType::SelectItem06, None),
                (InputControlType::SelectItem07, None),
                (InputControlType::SelectItem08, None),
                (InputControlType::SelectItem09, None),
                (InputControlType::SelectItem10, None)
            ]),
            _key_binding_widget_map: HashMap::from([
                (InputControlType::Attack, attack_key_binding_widget),
                (InputControlType::PowerAttack, power_attack_key_binding_widget),
                (InputControlType::Interaction, interaction_key_binding_widget),
                (InputControlType::EnterGate, enter_gate_key_binding_widget),
                (InputControlType::Gathering, gathering_key_binding_widget),
                (InputControlType::CameraRotation, camera_key_binding_widget),
                (InputControlType::Zoom, zoom_key_binding_widget),
                (InputControlType::Move, move_key_binding_widget),
                (InputControlType::Sprint, sprint_key_binding_widget),
                (InputControlType::Jump, jump_key_binding_widget),
                (InputControlType::Roll, roll_key_binding_widget),
                (InputControlType::SelectItem, select_item_key_binding_widget),
                (InputControlType::DropItem, drop_item_key_binding_widget),
                (InputControlType::UseItem, use_item_key_binding_widget),
                (InputControlType::SelectItem01, use_item01_key_binding_widget),
                (InputControlType::SelectItem02, use_item02_key_binding_widget),
                (InputControlType::SelectItem03, use_item03_key_binding_widget),
                (InputControlType::SelectItem04, use_item04_key_binding_widget),
                (InputControlType::SelectItem05, use_item05_key_binding_widget),
                (InputControlType::SelectItem06, use_item06_key_binding_widget),
                (InputControlType::SelectItem07, use_item07_key_binding_widget),
                (InputControlType::SelectItem08, use_item08_key_binding_widget),
                (InputControlType::SelectItem09, use_item09_key_binding_widget),
                (InputControlType::SelectItem10, use_item10_key_binding_widget)
            ]),
            _is_keyboard_input_mode: true,
            _last_interaction_object_key: std::ptr::null(),
        };

        character_controller_help_widget.update_key_binding_widgets( character_controller_help_widget._is_keyboard_input_mode );
        character_controller_help_widget
    }

    pub fn get_key_binding_widget(&self, input_control_type: &InputControlType) -> &KeyBindingWidget<'a> {
        self._key_binding_widget_map.get(input_control_type).unwrap()
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._character_controller_help_widget).get_ui_component_mut();
        ui_component.set_size_y(ui_component.get_num_children() as f32 * ITEM_SIZE + MAIN_LAYOUT_PADDING * 2.0);
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_ui_size().x - MAIN_LAYOUT_MARGIN * 2.0);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_ui_size().y - MAIN_LAYOUT_MARGIN * 2.0);
    }

    pub fn update_custom_key_binding_widgets(&mut self, key_binding_widget_type: InputControlType, material_instance_type: InputControlType) {
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

        for (input_control_type, key_binding_widget) in self._key_binding_widget_map.iter_mut() {
            let material_instance_data: Option<Vec<RcRefCell<MaterialInstanceData<'a>>>> = material_instance_map.get(input_control_type).unwrap().clone();
            key_binding_widget.update_icon_material_instance(material_instance_data);
        }
    }

    pub fn update_interaction_widget(&mut self, game_scene_manager: &GameSceneManager) {
        let mut matched_input_control_type = InputControlType::None;
        let mut interaction_name: String = String::new();
        let character_manager = game_scene_manager.get_character_manager();
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            if player.is_in_interaction_range() {
                let interaction_object = player.get_nearest_interaction_object();
                (matched_input_control_type, interaction_name) = match interaction_object {
                    InteractionObject::PropBed(_) => (InputControlType::Interaction, String::from("Sleep")),
                    InteractionObject::PropPickup(prop) => (InputControlType::Interaction, format!("Pick up a {}", prop.borrow()._prop_data.borrow()._name.as_str())),
                    InteractionObject::PropMonolith(_) => (InputControlType::Interaction, String::from("Open Toolbox")),
                    InteractionObject::PropTable(_) => (InputControlType::Interaction, String::from("Sit Down")),
                    InteractionObject::Npc(npc) => {
                        if player.get_attached_item_data_type().is_eatable() {
                            (InputControlType::Interaction, format!("Give a {} to {}", player.get_attached_item().as_ref().unwrap().borrow()._item_data.borrow()._name.as_str(), npc.borrow()._character_data.borrow()._name.as_str()))
                        } else {
                            (InputControlType::Interaction, format!("Interaction with {}", npc.borrow()._character_data.borrow()._name.as_str()))
                        }
                    },
                    InteractionObject::PropGate(_) => (InputControlType::None, String::from("Enter Gate")),
                    InteractionObject::PropGathering(prop) => (InputControlType::Gathering, format!("Hit the {}", prop.borrow()._prop_data.borrow()._name.as_str())),
                    _ => (InputControlType::Interaction, String::from("interaction"))
                };
            }
        }

        const INTERACTION_WIDGETS: [InputControlType; 3]= [InputControlType::Interaction, InputControlType::EnterGate, InputControlType::Gathering];
        for input_control_type in INTERACTION_WIDGETS.iter() {
            let interaction_key_binding_widget = self._key_binding_widget_map.get(&input_control_type).unwrap();
            let interaction_widget = ptr_as_mut(interaction_key_binding_widget._layout_widget);
            if *input_control_type == matched_input_control_type {
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

    pub fn update_controller_help_widget(&mut self, game_scene_manager: &GameSceneManager, game_controller: &GameController) {
        let is_keyboard_input_mode = game_controller.is_keyboard_input_mode();
        if self._is_keyboard_input_mode != is_keyboard_input_mode {
            self.update_key_binding_widgets(is_keyboard_input_mode);
            self._is_keyboard_input_mode = is_keyboard_input_mode;
        }

        // update interaction icon visibility
        self.update_interaction_widget(game_scene_manager);
    }
}
