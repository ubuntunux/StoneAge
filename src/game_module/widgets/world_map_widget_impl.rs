use crate::game_module::game_constants::{
    AUDIO_PICKUP_ITEM, DEFAULT_GATE_NAME, MATERIAL_PORTRAIT_MONKEY_ARU, MATERIAL_WORLDMAP,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, Stages};
use crate::game_module::widgets::world_map_widget::{
    WorldMapBridge, WorldMapDirection, WorldMapPlayer, WorldMapStage, WorldMapWidget,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, PosHintX, PosHintY, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

impl WorldMapDirection {
    pub fn get_opposite_direction(&self) -> WorldMapDirection {
        if *self == WorldMapDirection::LEFT {
            return WorldMapDirection::RIGHT;
        } else if *self == WorldMapDirection::RIGHT {
            return WorldMapDirection::LEFT;
        } else if *self == WorldMapDirection::UP {
            return WorldMapDirection::DOWN;
        } else if *self == WorldMapDirection::DOWN {
            return WorldMapDirection::UP;
        }
        WorldMapDirection::COUNT
    }
}

impl<'a> WorldMapBridge<'a> {
    pub fn create_world_map_bridge(
        _game_resources: &GameResources<'a>,
        root_layout: &mut WidgetDefault<'a>,
        pos_a: &Vector2<f32>,
        pos_b: &Vector2<f32>,
    ) -> Rc<WidgetDefault<'a>> {
        let world_map_bridge = UIManager::create_widget("world_map_bridge", UIWidgetTypes::Default);
        let diff_x = (pos_a.x - pos_b.x).abs();
        let diff_y = (pos_a.y - pos_b.y).abs();
        let center = (pos_a + pos_b) * 0.5;

        let ui_component = ptr_as_mut(world_map_bridge.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(32, 32, 0, 255));
        ui_component.set_border_color(get_color32(255, 255, 128, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);

        const BRIDGE_THICKNESS: f32 = 24.0;
        if diff_y < diff_x {
            ui_component.set_size(diff_x, BRIDGE_THICKNESS);
        } else {
            ui_component.set_size(BRIDGE_THICKNESS, diff_y);
        }
        ui_component.set_center(center.x, center.y);

        root_layout.add_widget(&world_map_bridge);

        world_map_bridge
    }
}

impl<'a> WorldMapStage<'a> {
    pub fn callback_touch_down(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        if !ui_component.get_user_data().is_null() {
            let world_map_stage = ptr_as_ref(ui_component.get_user_data() as *const WorldMapStage<'a>);
            world_map_stage
                .get_world_map_widget_mut()
                .set_selected_world_map_stage(world_map_stage.get_stage_data_name());
        }
        true
    }

    pub fn create_world_map_stage(
        world_map_stages: &mut HashMap<String, Rc<WorldMapStage<'a>>>,
        world_map_widget: &WorldMapWidget<'a>,
        _game_resources: &GameResources<'a>,
        root_layout: &mut WidgetDefault<'a>,
        stage: Stages,
    ) -> Rc<WorldMapStage<'a>> {
        let world_map_stage = UIManager::create_widget("world_map_stage", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_border_color(get_color32(128, 128, 128, 255));
        ui_component.set_round(10.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text(stage.get_stage_display_name());
        ui_component.set_font_size(32.0);
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_color(get_color32(255, 255, 255, 255));

        const STAGE_SIZE: f32 = 100.0;
        ui_component.set_size(STAGE_SIZE, STAGE_SIZE);
        root_layout.add_widget(&world_map_stage);

        let world_map_stage = Rc::new(WorldMapStage {
            _world_map_widget: world_map_widget,
            _stage_data_name: String::from(stage.get_stage_data_name()),
            _selected: false,
            _world_map_stage: world_map_stage,
            _linked_stages: [None, None, None, None, None],
            _linked_bridges: [None, None, None, None, None],
        });

        // set callback event
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(WorldMapStage::callback_touch_down)));
        ui_component.set_user_data(world_map_stage.as_ref() as *const WorldMapStage<'a> as *const c_void);

        world_map_stages.insert(String::from(stage.get_stage_data_name()), world_map_stage.clone());

        world_map_stage
    }

    pub fn get_world_map_widget(&self) -> &WorldMapWidget<'a> {
        ptr_as_ref(self._world_map_widget)
    }

    pub fn get_world_map_widget_mut(&self) -> &mut WorldMapWidget<'a> {
        ptr_as_mut(self._world_map_widget)
    }

    pub fn get_selected(&self) -> bool {
        self._selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_color(if selected {
            get_color32(255, 255, 0, 255)
        } else {
            get_color32(255, 255, 255, 255)
        });
        self._selected = selected;
    }

    pub fn get_center_pos(&self) -> Vector2<f32> {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.get_center()
    }

    pub fn set_center_pos(&mut self, center_x: f32, center_y: f32) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_center(center_x, center_y);
    }

    pub fn get_stage_data_name(&self) -> &String {
        &self._stage_data_name
    }

    pub fn get_linked_stage(&self, direction: WorldMapDirection) -> &Option<Rc<WorldMapStage<'a>>> {
        &self._linked_stages[direction as usize]
    }

    pub fn set_linked_stage(
        &mut self,
        game_resources: &GameResources<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        direction: WorldMapDirection,
        linked_stage: &Rc<WorldMapStage<'a>>,
        map_size: &Vector2<f32>,
        make_bridge: bool,
    ) {
        let linked_stage_mut = ptr_as_mut(linked_stage.as_ref());
        let pos_step_x = map_size.x * 0.15;
        let pos_step_y = map_size.y * 0.15;
        let pos = self.get_center_pos();
        if direction == WorldMapDirection::LEFT {
            linked_stage_mut.set_center_pos(pos.x - pos_step_x, pos.y);
        } else if direction == WorldMapDirection::RIGHT {
            linked_stage_mut.set_center_pos(pos.x + pos_step_x, pos.y);
        } else if direction == WorldMapDirection::UP {
            linked_stage_mut.set_center_pos(pos.x, pos.y - pos_step_y);
        } else if direction == WorldMapDirection::DOWN {
            linked_stage_mut.set_center_pos(pos.x, pos.y + pos_step_y);
        }
        self._linked_stages[direction as usize] = Some(linked_stage.clone());

        if make_bridge {
            let bridge_widget = WorldMapBridge::create_world_map_bridge(
                game_resources,
                bridge_layer,
                &pos,
                &linked_stage_mut.get_center_pos(),
            );
            self._linked_bridges[direction as usize] = Some(bridge_widget);
        }
    }
}

impl<'a> WorldMapPlayer<'a> {
    pub fn create_world_map_player(
        world_map_widget: &WorldMapWidget<'a>,
        game_resources: &GameResources<'a>,
        root_layout: &mut WidgetDefault<'a>,
    ) -> Box<WorldMapPlayer<'a>> {
        let material_instance =
            game_resources.get_engine_resources().get_material_instance_data(MATERIAL_PORTRAIT_MONKEY_ARU);

        let player_icon = UIManager::create_widget("player_icon", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_material_instance(Some(material_instance.clone()));

        const PLAYER_SIZE: f32 = 100.0;
        ui_component.set_size(PLAYER_SIZE, PLAYER_SIZE);
        ui_component.set_center(0.0, 0.0);
        root_layout.add_widget(&player_icon);

        Box::new(WorldMapPlayer {
            _world_map_widget: world_map_widget,
            _player_icon: player_icon,
        })
    }

    pub fn set_center_pos(&mut self, center_x: f32, center_y: f32) {
        let ui_component = ptr_as_mut(self._player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_center(center_x, center_y);
    }
}

impl<'a> WorldMapWidget<'a> {
    pub fn create_world_map_widget(
        game_scene_manager: &GameSceneManager<'a>,
        audio_manager: &AudioManager<'a>,
        game_resources: &GameResources<'a>,
        root_widget: &mut WidgetDefault<'a>,
        _window_size: &Vector2<i32>,
    ) -> Box<WorldMapWidget<'a>> {
        let background_layout = UIManager::create_widget("background image layout", UIWidgetTypes::Default);
        let background_layout_mut = ptr_as_mut(background_layout.as_ref());
        let ui_component = background_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(80, 80, 180, 255));
        ui_component.set_enable(false);
        root_widget.add_widget(&background_layout);

        let world_map_material_instance =
            game_resources.get_engine_resources().get_material_instance_data(MATERIAL_WORLDMAP);
        let texture_parameter =
            world_map_material_instance.borrow()._material_parameters.get("texture_color").unwrap().clone();
        let texture_name = texture_parameter.as_str().unwrap();
        let texture = game_resources.get_engine_resources().get_texture_data(texture_name);
        let image_width = texture.borrow()._image_width as f32;
        let image_height = texture.borrow()._image_height as f32;
        let image_aspect = image_width / image_height;

        let max_bounds = Vector2::new(1400.0, 800.0);
        let scale = (max_bounds.x / image_width).min(max_bounds.y / image_height);
        let map_size = Vector2::new(image_width * scale, image_height * scale);

        let world_map_widget = UIManager::create_widget("world_map_widget", UIWidgetTypes::Default);
        let world_map_widget_mut = ptr_as_mut(world_map_widget.as_ref());
        let ui_component = world_map_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_material_instance(Some(world_map_material_instance.clone()));
        background_layout_mut.add_widget(&world_map_widget);

        let bridge_layer_widget = UIManager::create_widget("bridge_layer_widget", UIWidgetTypes::Default);
        let bridge_layer_widget_mut = ptr_as_mut(bridge_layer_widget.as_ref());
        let ui_component = bridge_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&bridge_layer_widget);

        let stage_layer_widget = UIManager::create_widget("stage_layer_widget", UIWidgetTypes::Default);
        let stage_layer_widget_mut = ptr_as_mut(stage_layer_widget.as_ref());
        let ui_component = stage_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&stage_layer_widget);

        let player_layer_widget = UIManager::create_widget("player_layer_widget", UIWidgetTypes::Default);
        let player_layer_widget_mut = ptr_as_mut(player_layer_widget.as_ref());
        let ui_component = player_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&player_layer_widget);

        let mut world_map_widget = Box::new(WorldMapWidget {
            _game_scene_manager: game_scene_manager,
            _audio_manager: audio_manager,
            _root_widget: root_widget,
            _background_layout: background_layout.clone(),
            _world_map_widget: world_map_widget.clone(),
            _bridge_layer_widget: bridge_layer_widget.clone(),
            _stage_layer_widget: stage_layer_widget.clone(),
            _player_layer_widget: player_layer_widget.clone(),
            _image_aspect: image_aspect,
            _selected_stage_name: String::new(),
            _world_map_player: None,
            _world_map_stages: HashMap::new(),
            _is_opened_world_map: false,
            _request_close_world_map: false,
        });

        world_map_widget.as_mut()._world_map_player = Some(WorldMapPlayer::create_world_map_player(
            world_map_widget.as_ref(),
            game_resources,
            player_layer_widget_mut,
        ));
        world_map_widget.as_mut()._world_map_stages = WorldMapWidget::create_world_map_stages(
            world_map_widget.as_ref(),
            game_resources,
            stage_layer_widget_mut,
            bridge_layer_widget_mut,
            &map_size,
        );

        world_map_widget
    }

    pub fn create_world_map_stages(
        world_map_widget: &WorldMapWidget<'a>,
        game_resources: &GameResources<'a>,
        stage_layer: &mut WidgetDefault<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        map_size: &Vector2<f32>,
    ) -> HashMap<String, Rc<WorldMapStage<'a>>> {
        // create stages
        let mut world_map_stages = HashMap::new();
        let world_map_stage_home = WorldMapStage::create_world_map_stage(
            &mut world_map_stages,
            world_map_widget,
            game_resources,
            stage_layer,
            Stages::Home,
        );
        let world_map_stage_forest = WorldMapStage::create_world_map_stage(
            &mut world_map_stages,
            world_map_widget,
            game_resources,
            stage_layer,
            Stages::Forest,
        );
        let world_map_stage_cave = WorldMapStage::create_world_map_stage(
            &mut world_map_stages,
            world_map_widget,
            game_resources,
            stage_layer,
            Stages::Cave,
        );
        let world_map_stage_ufo = WorldMapStage::create_world_map_stage(
            &mut world_map_stages,
            world_map_widget,
            game_resources,
            stage_layer,
            Stages::Ufo,
        );

        // link stages
        let map_center = map_size * 0.5;
        ptr_as_mut(world_map_stage_home.as_ref()).set_center_pos(map_center.x, map_center.y);
        WorldMapWidget::set_linked_stage(
            game_resources,
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_forest,
            WorldMapDirection::RIGHT,
            map_size,
        );
        WorldMapWidget::set_linked_stage(
            game_resources,
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_cave,
            WorldMapDirection::DOWN,
            map_size,
        );
        WorldMapWidget::set_linked_stage(
            game_resources,
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_ufo,
            WorldMapDirection::LEFT,
            map_size,
        );

        world_map_stages
    }

    pub fn set_linked_stage(
        game_resources: &GameResources<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        stage: &Rc<WorldMapStage<'a>>,
        linked_stage: &Rc<WorldMapStage<'a>>,
        direction: WorldMapDirection,
        map_size: &Vector2<f32>,
    ) {
        ptr_as_mut(stage.as_ref()).set_linked_stage(
            game_resources,
            bridge_layer,
            direction,
            linked_stage,
            map_size,
            true,
        );
        ptr_as_mut(linked_stage.as_ref()).set_linked_stage(
            game_resources,
            bridge_layer,
            direction.get_opposite_direction(),
            stage,
            map_size,
            false,
        );
    }

    pub fn get_audio_manager(&self) -> &AudioManager<'a> {
        ptr_as_ref(self._audio_manager)
    }

    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
    }

    pub fn is_opened_world_map(&self) -> bool {
        self._is_opened_world_map
    }
    pub fn open_world_map(&mut self) {
        if !self._is_opened_world_map {
            self.get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            ptr_as_mut(self._background_layout.as_ref()).get_ui_component_mut().set_enable(true);
            self._request_close_world_map = false;
            self._is_opened_world_map = true;
        }
    }
    pub fn close_world_map(&mut self) {
        if self._is_opened_world_map {
            ptr_as_mut(self._background_layout.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened_world_map = false;
        }
    }
    pub fn is_requested_close_world_map(&self) -> bool {
        self._request_close_world_map
    }
    pub fn request_close_world_map(&mut self) {
        self.get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self._request_close_world_map = true;
    }
    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn teleport_selected_world_map_stage(&mut self) {
        self.set_selected_world_map_stage(&self._selected_stage_name.clone());
    }

    pub fn get_selected_world_map_stage_data_name(&self) -> &String {
        &self._selected_stage_name
    }

    pub fn set_selected_world_map_stage(&mut self, selected_stage_name: &String) {
        if !self._selected_stage_name.is_empty() && !selected_stage_name.is_empty() {
            self.get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }

        if self._selected_stage_name == *selected_stage_name {
            if let Some(selected_stage) = self._world_map_stages.get_mut(selected_stage_name) {
                let teleport_stage: &String = ptr_as_ref(selected_stage.as_ref()).get_stage_data_name();
                ptr_as_mut(self._game_scene_manager).set_teleport_stage(teleport_stage, DEFAULT_GATE_NAME);
            }
        } else {
            if let Some(prev_selected_stage) = self._world_map_stages.get_mut(&self._selected_stage_name) {
                ptr_as_mut(prev_selected_stage.as_ref()).set_selected(false);
            }

            if let Some(selected_stage) = self._world_map_stages.get_mut(selected_stage_name) {
                ptr_as_mut(selected_stage.as_ref()).set_selected(true);
                let pos: Vector2<f32> = ptr_as_ref(selected_stage.as_ref()).get_center_pos();
                self._world_map_player.as_mut().unwrap().set_center_pos(pos.x, pos.y);
            }

            self._selected_stage_name = selected_stage_name.clone();
        }
    }

    pub fn change_selected_world_map_stage(&mut self, direction: WorldMapDirection) {
        if let Some(selected_stage) = self._world_map_stages.get_mut(&self._selected_stage_name)
            && let Some(linked_stage) = ptr_as_ref(selected_stage.as_ref()).get_linked_stage(direction).as_ref()
        {
            self.set_selected_world_map_stage(linked_stage.get_stage_data_name());
        }
    }

    pub fn update_world_map(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        let is_left = keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || joystick_input_data._btn_left == ButtonState::Pressed;
        let is_right = keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || joystick_input_data._btn_right == ButtonState::Pressed;
        let is_down = keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || joystick_input_data._btn_down == ButtonState::Pressed;
        let is_up = keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || joystick_input_data._btn_up == ButtonState::Pressed;
        let is_interaction = keyboard_input_data.get_key_pressed(KeyCode::KeyF)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || joystick_input_data._btn_x == ButtonState::Pressed
            || joystick_input_data._btn_a == ButtonState::Pressed;

        let joystick_sensitivity: f32 = 0.1 / 32767.0;
        let _stick_left_direction = Vector2::<f32>::new(
            joystick_input_data._stick_left_direction.x as f32,
            joystick_input_data._stick_left_direction.y as f32,
        ) * joystick_sensitivity;
        let _stick_right_direction = Vector2::<f32>::new(
            joystick_input_data._stick_right_direction.x as f32,
            joystick_input_data._stick_right_direction.y as f32,
        ) * joystick_sensitivity;

        if keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_start == ButtonState::Pressed
            || joystick_input_data._btn_b == ButtonState::Pressed
        {
            self.request_close_world_map();
        }

        let world_map_direction = if is_left {
            WorldMapDirection::LEFT
        } else if is_right {
            WorldMapDirection::RIGHT
        } else if is_up {
            WorldMapDirection::UP
        } else if is_down {
            WorldMapDirection::DOWN
        } else {
            WorldMapDirection::COUNT
        };
        self.change_selected_world_map_stage(world_map_direction);

        if is_interaction {
            self.teleport_selected_world_map_stage();
        }
    }
}
