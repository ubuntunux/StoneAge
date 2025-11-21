use std::collections::HashMap;
use indexmap::IndexSet;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::actors::character::InteractionObject;
use crate::game_module::game_controller::InputControlType;
use crate::game_module::game_resource::GameResources;

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (250.0, 420.0);
const ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 24.0;

pub struct ControllerHelpWidget<'a> {
    pub _character_controller_help_widget: *const WidgetDefault<'a>,
    pub _keyboard_material_instance_map: HashMap<InputControlType, Option<RcRefCell<MaterialInstanceData<'a>>>>,
    pub _joystick_material_instance_map: HashMap<InputControlType, Option<RcRefCell<MaterialInstanceData<'a>>>>,
    pub _key_binding_widget_map: HashMap<InputControlType, KeyBindingWidget<'a>>,
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
    ) -> KeyBindingWidget<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
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
        let attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "attack_key_binding", "Attack");
        let power_attack_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "power_attack_key_binding", "Power Attack");
        let camera_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "camera_key_binding", "Camera");
        let zoom_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "zoom_key_binding", "Zoom");
        let move_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "move_key_binding", "Move");
        let sprint_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "sprint_key_binding", "Sprint");
        let jump_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "jump_key_binding", "Jump");
        let roll_key_binding_widget = KeyBindingWidget::create_key_binding_widget(character_controller_help_widget_mut, "roll_key_binding", "Roll");

        // interaction
        let interaction_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "interaction_key_binding", "Interaction");

        // quick slot
        let select_previous_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "interaction_key_binding", "Previous Item");
        let select_next_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "interaction_key_binding", "Next Item");
        let use_current_item_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "interaction_key_binding", "Use Item");
        let use_item01_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item01_key_binding", "");
        let use_item02_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item02_key_binding", "");
        let use_item03_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item03_key_binding", "");
        let use_item04_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item04_key_binding", "");
        let use_item05_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item05_key_binding", "");
        let use_item06_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item06_key_binding", "");
        let use_item07_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item07_key_binding", "");
        let use_item08_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item08_key_binding", "");
        let use_item09_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item09_key_binding", "");
        let use_item10_key_binding_widget = KeyBindingWidget::create_key_binding_widget(root_widget, "use_item10_key_binding", "");

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
                (InputControlType::SelectPreviousItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_q").clone())),
                (InputControlType::SelectNextItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_e").clone())),
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
                (InputControlType::Attack, Some(engine_resources.get_material_instance_data("ui/controller/mouse_l").clone())),
                (InputControlType::PowerAttack, Some(engine_resources.get_material_instance_data("ui/controller/mouse_r").clone())),
                (InputControlType::Interaction, Some(engine_resources.get_material_instance_data("ui/controller/keycode_f").clone())),
                (InputControlType::CameraRotation, Some(engine_resources.get_material_instance_data("ui/controller/mouse").clone())),
                (InputControlType::Zoom, Some(engine_resources.get_material_instance_data("ui/controller/mouse_m").clone())),
                (InputControlType::Move, Some(engine_resources.get_material_instance_data("ui/controller/keycode_move").clone())),
                (InputControlType::Sprint, Some(engine_resources.get_material_instance_data("ui/controller/keycode_shift").clone())),
                (InputControlType::Jump, Some(engine_resources.get_material_instance_data("ui/controller/keycode_space").clone())),
                (InputControlType::Roll, Some(engine_resources.get_material_instance_data("ui/controller/keycode_alt").clone())),
                (InputControlType::SelectPreviousItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_q").clone())),
                (InputControlType::SelectNextItem, Some(engine_resources.get_material_instance_data("ui/controller/keycode_e").clone())),
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
                (InputControlType::SelectPreviousItem, select_previous_item_key_binding_widget),
                (InputControlType::SelectNextItem, select_next_item_key_binding_widget),
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
        };

        character_controller_help_widget.update_key_binding_widgets();
        character_controller_help_widget
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._character_controller_help_widget).get_ui_component_mut();
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_size_x() - MAIN_LAYOUT_MARGIN);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - MAIN_LAYOUT_MARGIN);
    }

    pub fn update_key_binding_widgets(&mut self) {
        for (input_control_type, key_binding_widget) in self._key_binding_widget_map.iter_mut() {
            let material_instance_data: Option<RcRefCell<MaterialInstanceData<'a>>> = self._keyboard_material_instance_map.get(input_control_type).unwrap().clone();
            key_binding_widget.update_icon_material_instance(material_instance_data);
        }
    }

    pub fn get_interaction_object_position(&self, game_scene_manager: &GameSceneManager, interaction_objects: &IndexSet<InteractionObject>) -> Option<Vector3<f32>> {
        match interaction_objects.last().unwrap() {
            InteractionObject::PropBed(prop_id) => {
                if let Some(prop) = game_scene_manager.get_prop_manager().get_prop(*prop_id) {
                    return Some(prop.borrow().get_bounding_box().get_center().clone());
                }
            }
            InteractionObject::PropPickup(prop_id) => {
                if let Some(prop) = game_scene_manager.get_prop_manager().get_prop(*prop_id) {
                    return Some(prop.borrow().get_bounding_box().get_center().clone());
                }
            }
            _ => ()
        }
        None
    }

    pub fn update_controller_help_widget(&mut self, game_scene_manager: &GameSceneManager) {
        // update interaction icon visibility
        let character_manager = game_scene_manager.get_character_manager();
        let interaction_widget = ptr_as_mut(self._key_binding_widget_map.get(&InputControlType::Interaction).unwrap()._layout_widget);
        let mut is_visible_interaction_widget = false;
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            let interaction_objects: &IndexSet<InteractionObject> = player.get_interaction_objects();
            if interaction_objects.is_empty() == false {
                if let Some(position) = self.get_interaction_object_position(game_scene_manager, interaction_objects) {
                    let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                    let screen_position = main_camera.convert_world_to_screen(&position, false);
                    interaction_widget.get_ui_component_mut().set_pos(screen_position.x, screen_position.y);
                    is_visible_interaction_widget = true;
                }
            }
        }
        interaction_widget._ui_component.set_visible(is_visible_interaction_widget);
    }
}
