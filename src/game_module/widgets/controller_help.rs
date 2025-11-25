use std::collections::HashMap;
use nalgebra::{Vector2};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController, InputControlType};
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::item_bar_widget::{ItemBarWidget, ITEM_UI_SIZE};

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_PADDING: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (250.0, 460.0);
const ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 24.0;

pub struct ControllerHelpWidget<'a> {
    pub _character_controller_help_widget: *const WidgetDefault<'a>,
    pub _keyboard_material_instance_map: HashMap<InputControlType, Option<RcRefCell<MaterialInstanceData<'a>>>>,
    pub _joystick_material_instance_map: HashMap<InputControlType, Option<RcRefCell<MaterialInstanceData<'a>>>>,
    pub _key_binding_widget_map: HashMap<InputControlType, KeyBindingWidget<'a>>,
    pub _is_keyboard_input_mode: bool,
}

pub struct KeyBindingWidget<'a> {
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _binding_icon_widget: *const WidgetDefault<'a>,
    pub _binding_name_widget: *const WidgetDefault<'a>,
}

impl<'a> KeyBindingWidget<'a> {
    pub fn create_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        widget_name: &str,
        key_binding_text: &str,
        offset: Option<Vector2<f32>>
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        if let Some(offset) = offset {
            ui_component.set_pos(offset.x, offset.y);
        }
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        parent_widget.add_widget(&layout_widget);

        let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_x(ITEM_SIZE);
        ui_component.set_size_y(ITEM_SIZE);
        layout_widget_mut.add_widget(&binding_icon_widget);

        let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_padding_left(10.0);
        ui_component.set_text(key_binding_text);
        layout_widget_mut.add_widget(&binding_name_widget);

        KeyBindingWidget {
            _layout_widget: layout_widget.as_ref(),
            _binding_icon_widget: binding_icon_widget.as_ref(),
            _binding_name_widget: binding_name_widget.as_ref(),
        }
    }

    pub fn update_icon_material_instance(&mut self, material_instance_data: Option<RcRefCell<MaterialInstanceData<'a>>>) {
        ptr_as_mut(self._layout_widget)._ui_component.set_visible(material_instance_data.is_some());
        ptr_as_mut(self._binding_icon_widget)._ui_component.set_material_instance(material_instance_data);
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
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_padding(10.0);
        ui_component.set_round(15.0);
        ui_component.set_size(MAIN_LAYOUT_SIZE.0, MAIN_LAYOUT_SIZE.1);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&character_controller_help_widget);
        let camera_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "view_key_binding", "View", None);
        let move_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "move_key_binding", "Move", None);
        let zoom_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "zoom_key_binding", "Zoom", None);
        let attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "attack_key_binding", "Attack", None);
        let power_attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "power_attack_key_binding", "Power Attack", None);
        let sprint_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "sprint_key_binding", "Sprint", None);
        let jump_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "jump_key_binding", "Jump", None);
        let roll_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "roll_key_binding", "Roll", None);
        let select_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "select_item_key_binding", "Select Item", None);

        // interaction
        let interaction_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "interaction_key_binding", "Interaction", None);

        // quick slot
        let use_current_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_selected_item_widget()._widget), "use_current_item_key_binding", "Use Item", Some(Vector2::new(0.0, -ITEM_UI_SIZE * 0.25)));
        let use_item01_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(0)._widget), "use_item01_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item02_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(1)._widget), "use_item02_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item03_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(2)._widget), "use_item03_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item04_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(3)._widget), "use_item04_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item05_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(4)._widget), "use_item05_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item06_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(5)._widget), "use_item06_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item07_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(6)._widget), "use_item07_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item08_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(7)._widget), "use_item08_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item09_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(8)._widget), "use_item09_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));
        let use_item10_key_binding_widget = KeyBindingWidget::create_key_binding_widget(ptr_as_mut(item_bar_widget.get_item_widget(9)._widget), "use_item10_key_binding", "", Some(Vector2::new(0.0, ITEM_UI_SIZE * 0.25)));

        let mut character_controller_help_widget = ControllerHelpWidget {
            _character_controller_help_widget: character_controller_help_widget_mut,
            _keyboard_material_instance_map: HashMap::from([
                (InputControlType::Attack, Some(engine_resources.get_material_instance_data("ui/controller/mouse_l").clone())),
                (InputControlType::PowerAttack, Some(engine_resources.get_material_instance_data("ui/controller/mouse_r").clone())),
                (InputControlType::Interaction, Some(engine_resources.get_material_instance_data("ui/controller/keycode_f").clone())),
                (InputControlType::CameraRotation, Some(engine_resources.get_material_instance_data("ui/controller/mouse").clone())),
                (InputControlType::Zoom, Some(engine_resources.get_material_instance_data("ui/controller/mouse_m").clone())),
                (InputControlType::Move, Some(engine_resources.get_material_instance_data("ui/controller/keycode_move").clone())),
                (InputControlType::Sprint, Some(engine_resources.get_material_instance_data("ui/controller/keycode_shift").clone())),
                (InputControlType::Jump, Some(engine_resources.get_material_instance_data("ui/controller/keycode_space").clone())),
                (InputControlType::Roll, Some(engine_resources.get_material_instance_data("ui/controller/keycode_alt").clone())),
                (InputControlType::SelectItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_q").clone())),
                (InputControlType::UseCurrentItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_c").clone())),
                (InputControlType::UseItem01, Some(engine_resources.get_material_instance_data("ui/controller/keycode_1").clone())),
                (InputControlType::UseItem02, Some(engine_resources.get_material_instance_data("ui/controller/keycode_2").clone())),
                (InputControlType::UseItem03, Some(engine_resources.get_material_instance_data("ui/controller/keycode_3").clone())),
                (InputControlType::UseItem04, Some(engine_resources.get_material_instance_data("ui/controller/keycode_4").clone())),
                (InputControlType::UseItem05, Some(engine_resources.get_material_instance_data("ui/controller/keycode_5").clone())),
                (InputControlType::UseItem06, Some(engine_resources.get_material_instance_data("ui/controller/keycode_6").clone())),
                (InputControlType::UseItem07, Some(engine_resources.get_material_instance_data("ui/controller/keycode_7").clone())),
                (InputControlType::UseItem08, Some(engine_resources.get_material_instance_data("ui/controller/keycode_8").clone())),
                (InputControlType::UseItem09, Some(engine_resources.get_material_instance_data("ui/controller/keycode_9").clone())),
                (InputControlType::UseItem10, Some(engine_resources.get_material_instance_data("ui/controller/keycode_0").clone()))
            ]),
            _joystick_material_instance_map: HashMap::from([
                (InputControlType::Attack, Some(engine_resources.get_material_instance_data("ui/controller/joystick_rb").clone())),
                (InputControlType::PowerAttack, Some(engine_resources.get_material_instance_data("ui/controller/joystick_rt").clone())),
                (InputControlType::Interaction, Some(engine_resources.get_material_instance_data("ui/controller/joystick_x").clone())),
                (InputControlType::CameraRotation, Some(engine_resources.get_material_instance_data("ui/controller/joystick_l_stick").clone())),
                (InputControlType::Zoom, Some(engine_resources.get_material_instance_data("ui/controller/joystick_up").clone())),
                (InputControlType::Move, Some(engine_resources.get_material_instance_data("ui/controller/joystick_r_stick").clone())),
                (InputControlType::Sprint, Some(engine_resources.get_material_instance_data("ui/controller/joystick_lb").clone())),
                (InputControlType::Jump, Some(engine_resources.get_material_instance_data("ui/controller/joystick_a").clone())),
                (InputControlType::Roll, Some(engine_resources.get_material_instance_data("ui/controller/joystick_b").clone())),
                (InputControlType::SelectItem, Some(engine_resources.get_material_instance_data("ui/controller/joystick_left").clone())),
                (InputControlType::UseCurrentItem, Some(engine_resources.get_material_instance_data("ui/controller/joystick_y").clone())),
                (InputControlType::UseItem01, None),
                (InputControlType::UseItem02, None),
                (InputControlType::UseItem03, None),
                (InputControlType::UseItem04, None),
                (InputControlType::UseItem05, None),
                (InputControlType::UseItem06, None),
                (InputControlType::UseItem07, None),
                (InputControlType::UseItem08, None),
                (InputControlType::UseItem09, None),
                (InputControlType::UseItem10, None)
            ]),
            _key_binding_widget_map: HashMap::from([
                (InputControlType::Attack, attack_key_binding_widget),
                (InputControlType::PowerAttack, power_attack_key_binding_widget),
                (InputControlType::Interaction, interaction_key_binding_widget),
                (InputControlType::CameraRotation, camera_key_binding_widget),
                (InputControlType::Zoom, zoom_key_binding_widget),
                (InputControlType::Move, move_key_binding_widget),
                (InputControlType::Sprint, sprint_key_binding_widget),
                (InputControlType::Jump, jump_key_binding_widget),
                (InputControlType::Roll, roll_key_binding_widget),
                (InputControlType::SelectItem, select_item_key_binding_widget),
                (InputControlType::UseCurrentItem, use_current_item_key_binding_widget),
                (InputControlType::UseItem01, use_item01_key_binding_widget),
                (InputControlType::UseItem02, use_item02_key_binding_widget),
                (InputControlType::UseItem03, use_item03_key_binding_widget),
                (InputControlType::UseItem04, use_item04_key_binding_widget),
                (InputControlType::UseItem05, use_item05_key_binding_widget),
                (InputControlType::UseItem06, use_item06_key_binding_widget),
                (InputControlType::UseItem07, use_item07_key_binding_widget),
                (InputControlType::UseItem08, use_item08_key_binding_widget),
                (InputControlType::UseItem09, use_item09_key_binding_widget),
                (InputControlType::UseItem10, use_item10_key_binding_widget)
            ]),
            _is_keyboard_input_mode: true,
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
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_size_x() - MAIN_LAYOUT_MARGIN);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - MAIN_LAYOUT_MARGIN);
    }

    pub fn update_key_binding_widgets(&mut self, is_keyboard_input_mode: bool) {
        let material_instance_map = if is_keyboard_input_mode {
            &self._keyboard_material_instance_map
        } else {
            &self._joystick_material_instance_map
        };

        for (input_control_type, key_binding_widget) in self._key_binding_widget_map.iter_mut() {
            let material_instance_data: Option<RcRefCell<MaterialInstanceData<'a>>> = material_instance_map.get(input_control_type).unwrap().clone();
            key_binding_widget.update_icon_material_instance(material_instance_data);
        }
    }

    pub fn update_controller_help_widget(&mut self, game_scene_manager: &GameSceneManager, game_controller: &GameController) {
        let is_keyboard_input_mode = game_controller.is_keyboard_input_mode();
        if self._is_keyboard_input_mode != is_keyboard_input_mode {
            self.update_key_binding_widgets(is_keyboard_input_mode);
            self._is_keyboard_input_mode = is_keyboard_input_mode;
        }

        // update interaction icon visibility
        let character_manager = game_scene_manager.get_character_manager();
        let interaction_widget = ptr_as_mut(self._key_binding_widget_map.get(&InputControlType::Interaction).unwrap()._layout_widget);
        let mut is_visible_interaction_widget = false;
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            if player.is_in_interaction_range() {
                let interaction_object = player.get_nearest_interaction_object();
                let position = interaction_object.get_position();
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let screen_position = main_camera.convert_world_to_screen(&position, false);
                interaction_widget.get_ui_component_mut().set_pos(screen_position.x, screen_position.y);
                is_visible_interaction_widget = true;
            }
        }
        interaction_widget._ui_component.set_visible(is_visible_interaction_widget);
    }
}
