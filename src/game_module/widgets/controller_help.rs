use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::game_controller::KeyBindingType;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::widgets::key_binding_widget::{
    KEY_BINDING_FONT_SIZE, KEY_BINDING_ICON_MARGIN, KEY_BINDING_TEXT_MARGIN,
    KeyBindingWidgetManager, KeyBindingWidgetMap,
};
use crate::game_module::widgets::key_binding_widget::{KEY_BINDING_UI_SIZE, KeyBindingWidget};
use nalgebra::Vector2;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_PADDING: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (280.0, 460.0);

pub fn create_player_control_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_text: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
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
    for _ in 0..key_binding_icons.len() {
        let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_right(KEY_BINDING_ICON_MARGIN);
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
    ui_component.set_margin_left(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_margin_right(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_font_size(KEY_BINDING_FONT_SIZE);
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_color(get_color32(255, 255, 255, 0));
    ui_component.set_text(key_binding_text);
    layout_widget_mut.add_widget(&binding_name_widget);

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: binding_name_widget.as_ref(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons,
    }
}

pub fn create_interaction_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_text: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
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
    ui_component.set_margin_right(KEY_BINDING_ICON_MARGIN);
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
    ui_component.set_margin_left(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_margin_right(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_font_size(KEY_BINDING_FONT_SIZE);
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_color(get_color32(255, 255, 255, 0));
    ui_component.set_text(key_binding_text);
    layout_widget_mut.add_widget(&binding_name_widget);

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: binding_name_widget.as_ref(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons,
    }
}

pub struct ControllerHelpWidget<'a> {
    pub _root_widget: *const WidgetDefault<'a>,
    pub _player_controller_help_widget: *const WidgetDefault<'a>,
    pub _player_control_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _interaction_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _last_interaction_object_key: *const c_void,
    pub _window_size: Vector2<i32>,
}

impl<'a> ControllerHelpWidget<'a> {
    pub fn create_controller_help_widget(
        engine_resources: &EngineResources<'a>,
        key_binding_widget_manager: *const KeyBindingWidgetManager<'a>,
        root_widget: &mut WidgetDefault<'a>,
        window_size: &Vector2<i32>,
    ) -> ControllerHelpWidget<'a> {
        let player_controller_help_widget =
            UIManager::create_widget("player_controller_help_widget", UIWidgetTypes::Default);
        let player_controller_help_widget_mut = ptr_as_mut(player_controller_help_widget.as_ref());
        let ui_component =
            ptr_as_mut(player_controller_help_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_round(15.0);
        ui_component.set_size(MAIN_LAYOUT_SIZE.0, MAIN_LAYOUT_SIZE.1);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&player_controller_help_widget);

        let mut player_controller_help_widget = ControllerHelpWidget {
            _root_widget: root_widget,
            _player_controller_help_widget: player_controller_help_widget_mut,
            _player_control_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _interaction_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _last_interaction_object_key: std::ptr::null(),
            _window_size: *window_size,
        };

        player_controller_help_widget
            .register_key_binding_widgets(engine_resources, key_binding_widget_manager);
        player_controller_help_widget
    }

    pub fn register_key_binding_widgets(
        &mut self,
        engine_resources: &EngineResources<'a>,
        key_binding_widget_manager: *const KeyBindingWidgetManager<'a>,
    ) {
        let key_binding_widget_manager = ptr_as_mut(key_binding_widget_manager);
        key_binding_widget_manager
            .register_key_binding_widget_map(&self._player_control_key_binding_widget_map);
        key_binding_widget_manager
            .register_key_binding_widget_map(&self._interaction_key_binding_widget_map);

        // player control ui
        let player_control_key_binding_widget_map =
            ptr_as_mut(self._player_control_key_binding_widget_map.as_ref());
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::CameraRotation,
                "view_key_binding",
                "View",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/mouse")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_l_stick")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Move,
                "move_key_binding",
                "Move",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_a")
                        .clone(),
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_s")
                        .clone(),
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_d")
                        .clone(),
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_w")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_r_stick")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Zoom,
                "zoom_key_binding",
                "Zoom",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/mouse_m")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_up")
                        .clone(),
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_down")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Attack,
                "attack_key_binding",
                "Attack",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/mouse_l")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_rb")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::PowerAttack,
                "power_attack_key_binding",
                "Power Attack",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/mouse_r")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_rt")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Sprint,
                "sprint_key_binding",
                "Sprint",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_shift")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_lb")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Jump,
                "jump_key_binding",
                "Jump",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_space")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_a")
                        .clone(),
                ],
            ),
        );
        player_control_key_binding_widget_map.register_key_binding_widget(
            create_player_control_key_binding_widget(
                ptr_as_mut(self._player_controller_help_widget),
                KeyBindingType::Roll,
                "roll_key_binding",
                "Roll",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_alt")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_b")
                        .clone(),
                ],
            ),
        );

        // interaction
        let interaction_key_binding_widget_map =
            ptr_as_mut(self._interaction_key_binding_widget_map.as_ref());
        interaction_key_binding_widget_map.register_key_binding_widget(
            create_interaction_key_binding_widget(
                ptr_as_mut(self._root_widget),
                KeyBindingType::Interaction,
                "interaction_key_binding",
                "Interaction",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_f")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_x")
                        .clone(),
                ],
            ),
        );
        interaction_key_binding_widget_map.register_key_binding_widget(
            create_interaction_key_binding_widget(
                ptr_as_mut(self._root_widget),
                KeyBindingType::EnterGate,
                "enter_gate_key_binding",
                "Enter",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/keycode_w")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_r_stick")
                        .clone(),
                ],
            ),
        );
        interaction_key_binding_widget_map.register_key_binding_widget(
            create_interaction_key_binding_widget(
                ptr_as_mut(self._root_widget),
                KeyBindingType::Gathering,
                "gathering_key_binding",
                "Gathering",
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/mouse_l")
                        .clone(),
                ],
                vec![
                    engine_resources
                        .get_material_instance_data("ui/controller/joystick_rb")
                        .clone(),
                ],
            ),
        );
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        self._window_size = *window_size;

        let ui_component = ptr_as_mut(self._player_controller_help_widget).get_ui_component_mut();
        ui_component.set_size_y(
            ui_component.get_num_children() as f32 * KEY_BINDING_UI_SIZE
                + MAIN_LAYOUT_PADDING * 2.0,
        );
        ui_component.set_pos_x(
            window_size.x as f32 - ui_component.get_ui_size().x - MAIN_LAYOUT_MARGIN * 2.0,
        );
        ui_component.set_pos_y(
            window_size.y as f32 - ui_component.get_ui_size().y - MAIN_LAYOUT_MARGIN * 2.0,
        );
    }

    pub fn update_interaction_widget(&mut self, game_scene_manager: &GameSceneManager) {
        let interaction_key_binding_widget_map =
            ptr_as_mut(self._interaction_key_binding_widget_map.as_ref());
        let mut matched_key_binding_type = KeyBindingType::None;
        let mut interaction_name: String = String::new();
        let character_manager = game_scene_manager.get_character_manager();
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            if player.is_in_interaction_range() {
                let interaction_object = player.get_nearest_interaction_object();
                (matched_key_binding_type, interaction_name) = match interaction_object {
                    InteractionObject::PropBed(_) => {
                        (KeyBindingType::Interaction, String::from("Wrap up the day"))
                    }
                    InteractionObject::PropPickup(prop) => (
                        KeyBindingType::Interaction,
                        format!(
                            "Pick up a {}",
                            prop.borrow()._prop_data.borrow()._name.as_str()
                        ),
                    ),
                    InteractionObject::PropMonolith(_) => {
                        (KeyBindingType::Interaction, String::from("Open Toolbox"))
                    }
                    InteractionObject::PropTable(_) => {
                        (KeyBindingType::Interaction, String::from("Sit Down"))
                    }
                    InteractionObject::Npc(npc) => {
                        if player.get_attached_item_data_type().is_eatable() {
                            (
                                KeyBindingType::Interaction,
                                format!(
                                    "Give a {} to {}",
                                    player
                                        .get_attached_item()
                                        .as_ref()
                                        .unwrap()
                                        .borrow()
                                        ._item_data
                                        .borrow()
                                        ._name
                                        .as_str(),
                                    npc.borrow()._character_data.borrow()._name.as_str()
                                ),
                            )
                        } else {
                            (
                                KeyBindingType::Interaction,
                                format!(
                                    "Interaction with {}",
                                    npc.borrow()._character_data.borrow()._name.as_str()
                                ),
                            )
                        }
                    }
                    InteractionObject::PropGate(_) => {
                        (KeyBindingType::None, String::from("Enter Gate"))
                    }
                    InteractionObject::PropGathering(prop) => (
                        KeyBindingType::Gathering,
                        format!(
                            "Hit the {}",
                            prop.borrow()._prop_data.borrow()._name.as_str()
                        ),
                    ),
                    _ => (KeyBindingType::Interaction, String::from("interaction")),
                };
            }
        }

        const INTERACTION_WIDGETS: [KeyBindingType; 3] = [
            KeyBindingType::Interaction,
            KeyBindingType::EnterGate,
            KeyBindingType::Gathering,
        ];
        for key_binding_type in INTERACTION_WIDGETS.iter() {
            let mut enable_interaction = true;
            if character_manager.is_valid_player() {
                let player = character_manager.get_player().borrow();
                if !player.is_alive()
                    || player.is_action(ActionAnimationState::Sleep)
                    || player.is_action(ActionAnimationState::SleepNoSnoring)
                    || player.is_action(ActionAnimationState::LayingDown)
                    || player.is_action(ActionAnimationState::WakeUp)
                {
                    enable_interaction = false;
                }
            }

            let interaction_key_binding_widget =
                interaction_key_binding_widget_map.get_key_binding_widget(*key_binding_type);
            let interaction_widget = ptr_as_mut(interaction_key_binding_widget._layout_widget);
            if enable_interaction && *key_binding_type == matched_key_binding_type {
                let player = character_manager.get_player().borrow();
                let interaction_object = player.get_nearest_interaction_object();
                let position = interaction_object.get_position();
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let screen_position = main_camera.convert_world_to_screen(&position, true);
                interaction_widget
                    ._ui_component
                    .set_pos(screen_position.x, screen_position.y);
                interaction_widget._ui_component.set_visible(true);
                ptr_as_mut(interaction_key_binding_widget._binding_name_widget)
                    ._ui_component
                    .set_text(interaction_name.as_str());
                self._last_interaction_object_key = interaction_object.get_key();
            } else {
                interaction_widget._ui_component.set_visible(false);
            }
        }
    }

    pub fn update_controller_help_widget(&mut self, game_scene_manager: &GameSceneManager) {
        self.update_interaction_widget(game_scene_manager);
    }
}
