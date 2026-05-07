use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;
use crate::game_module::game_resource::GameResources;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{HorizontalAlign, PosHintX, PosHintY, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_constants::{MATERIAL_PORTRAIT_MONKEY_ARU, MATERIAL_WORLDMAP};
use crate::game_module::game_controller::GameController;
use crate::game_module::widgets::world_map_widget::{WorldMapBridge, WorldMapDirection, WorldMapPlayer, WorldMapStage, WorldMapWidget};

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
        pos_hint_a: &Vector2<f32>,
        pos_hint_b: &Vector2<f32>,
        world_map_aspect: f32
    ) -> Rc<WidgetDefault<'a>> {
        let world_map_bridge = UIManager::create_widget("world_map_bridge", UIWidgetTypes::Default);
        let size_hint_x = (pos_hint_a.x - pos_hint_b.x).abs();
        let size_hint_y = (pos_hint_a.y - pos_hint_b.y).abs();

        let ui_component = ptr_as_mut(world_map_bridge.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(32, 32, 0, 255));
        ui_component.set_border_color(get_color32(255, 255, 128, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Center((pos_hint_a.x + pos_hint_b.x) / 2.0));
        ui_component.set_pos_hint_y(PosHintY::Center((pos_hint_a.y + pos_hint_b.y) / 2.0));

        let size_hint = 0.025;
        if size_hint_y < size_hint_x {
            ui_component.set_size_hint_x(Some(size_hint_x));
            ui_component.set_size_hint_y(Some(world_map_aspect * size_hint));
        } else {
            ui_component.set_size_hint_x(Some(size_hint));
            ui_component.set_size_hint_y(Some(size_hint_y));
        }

        root_layout.add_widget(&world_map_bridge);

        world_map_bridge
    }
}

impl<'a> WorldMapStage<'a> {
    pub fn callback_touch_down(_ui_component: &UIComponentInstance<'a>, _touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        if _ui_component.get_user_data().is_null() == false {
            let world_map_stage = ptr_as_mut(_ui_component.get_user_data() as *const WorldMapStage<'a>);
            world_map_stage.set_selected(!world_map_stage.get_selected());
        }
        true
    }

    pub fn create_world_map_stage(_game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>, stage_name: &str, world_map_aspect: f32) -> Rc<WorldMapStage<'a>> {
        let world_map_stage = UIManager::create_widget("world_map_stage", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_border_color(get_color32(128, 128, 128, 255));
        ui_component.set_round(10.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text(stage_name);
        ui_component.set_font_size(32.0);
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_color(get_color32(255, 255, 255, 255));

        let size_hint = 0.05;
        ui_component.set_size_hint_x(Some(size_hint));
        ui_component.set_size_hint_y(Some(world_map_aspect * size_hint));
        root_layout.add_widget(&world_map_stage);

        let world_map_stage = Rc::new(WorldMapStage {
            _selected: false,
            _stage_name: String::from(stage_name),
            _world_map_stage: world_map_stage,
            _linked_stages: [None, None, None, None],
            _linked_bridges: [None, None, None, None]
        });

        // set callback event
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(WorldMapStage::callback_touch_down)));
        ui_component.set_user_data(world_map_stage.as_ref() as *const WorldMapStage<'a> as *const c_void);

        world_map_stage
    }

    pub fn get_selected(&self) -> bool {
        self._selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_color(if selected { get_color32(255, 255, 0, 255) } else { get_color32(255, 255, 255, 255) });
        self._selected = selected;
    }

    pub fn get_pos_hint(&self) -> Vector2<f32> {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        let pos_hint_x = if let PosHintX::Center(pos_hint_x) = ui_component.get_pos_hint_x() {
            pos_hint_x
        } else {
            0.0
        };

        let pos_hint_y = if let PosHintY::Center(pos_hint_y) = ui_component.get_pos_hint_y() {
            pos_hint_y
        } else {
            0.0
        };
        Vector2::new(pos_hint_x, pos_hint_y)
    }

    pub fn set_pos_hint(&mut self, pos_hint_x: f32, pos_hint_y: f32) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_pos_hint_x(PosHintX::Center(pos_hint_x));
        ui_component.set_pos_hint_y(PosHintY::Center(pos_hint_y));
    }

    pub fn get_linked_stage(&self, direction: WorldMapDirection) -> &Option<Rc<WorldMapStage<'a>>> {
        &self._linked_stages[direction as usize]
    }

    pub fn set_linked_stage(&mut self, game_resources: &GameResources<'a>, bridge_layer: &mut WidgetDefault<'a>, direction: WorldMapDirection, maybe_stage: Option<Rc<WorldMapStage<'a>>>, world_map_aspect: f32) {
        if let Some(stage) = maybe_stage.as_ref() {
            let stage = ptr_as_mut(stage.as_ref());
            let pos_hint = self.get_pos_hint();
            let pos_hint_step_x = 0.1;
            let pos_hint_step_y = pos_hint_step_x * world_map_aspect;
            if direction == WorldMapDirection::LEFT {
                stage.set_pos_hint(pos_hint.x - pos_hint_step_x, pos_hint.y);
            } else if direction == WorldMapDirection::RIGHT {
                stage.set_pos_hint(pos_hint.x + pos_hint_step_x, pos_hint.y);
            } else if direction == WorldMapDirection::UP {
                stage.set_pos_hint(pos_hint.x, pos_hint.y + pos_hint_step_y);
            } else if direction == WorldMapDirection::DOWN {
                stage.set_pos_hint(pos_hint.x, pos_hint.y - pos_hint_step_y);
            }

            let bridge_widget = WorldMapBridge::create_world_map_bridge(
                game_resources,
                bridge_layer,
                &pos_hint,
                &stage.get_pos_hint(),
                world_map_aspect
            );

            self._linked_stages[direction as usize] = maybe_stage;
            self._linked_bridges[direction as usize] = Some(bridge_widget);
        } else {
            assert!(false, "do not allow set none stage");
        }
    }
}


impl<'a> WorldMapPlayer<'a> {
    pub fn create_world_map_player(game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>, world_map_aspect: f32) -> Box<WorldMapPlayer<'a>> {
        let material_instance = game_resources.get_engine_resources().get_material_instance_data(MATERIAL_PORTRAIT_MONKEY_ARU);

        let player_icon = UIManager::create_widget("player_icon", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_material_instance(Some(material_instance.clone()));

        let size_hint = 0.05;
        ui_component.set_size_hint_x(Some(size_hint));
        ui_component.set_size_hint_y(Some(world_map_aspect * size_hint));
        ui_component.set_pos_hint_x(PosHintX::Center(0.0));
        ui_component.set_pos_hint_y(PosHintY::Center(0.0));
        root_layout.add_widget(&player_icon);

        Box::new(WorldMapPlayer {
            _player_icon: player_icon,
            _player_icon_pos: Vector2::zeros()
        })
    }
}


impl<'a> WorldMapWidget<'a> {
    pub fn create_world_map_widget(
        root_widget: &mut WidgetDefault<'a>,
        game_resources: &GameResources<'a>
    ) -> WorldMapWidget<'a> {
        let background_layout = UIManager::create_widget("background image layout", UIWidgetTypes::Default);
        let background_layout_mut = ptr_as_mut(background_layout.as_ref());
        let ui_component = background_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 255));
        ui_component.set_visible(false);
        root_widget.add_widget(&background_layout);

        let world_map_material_instance = game_resources.get_engine_resources().get_material_instance_data(MATERIAL_WORLDMAP);
        let world_map_widget = UIManager::create_widget("world_map_widget", UIWidgetTypes::Default);
        let world_map_widget_mut = ptr_as_mut(world_map_widget.as_ref());
        let ui_component = world_map_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_material_instance(Some(world_map_material_instance.clone()));
        background_layout_mut.add_widget(&world_map_widget);

        let bridge_layer_widget = UIManager::create_widget("bridge_layer_widget", UIWidgetTypes::Default);
        let bridge_layer_widget_mut = ptr_as_mut(bridge_layer_widget.as_ref());
        let ui_component = bridge_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&bridge_layer_widget);

        let stage_layer_widget = UIManager::create_widget("stage_layer_widget", UIWidgetTypes::Default);
        let stage_layer_widget_mut = ptr_as_mut(stage_layer_widget.as_ref());
        let ui_component = stage_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&stage_layer_widget);

        let player_layer_widget = UIManager::create_widget("player_layer_widget", UIWidgetTypes::Default);
        let player_layer_widget_mut = ptr_as_mut(player_layer_widget.as_ref());
        let ui_component = player_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&player_layer_widget);

        let texture_parameter = world_map_material_instance.borrow()._material_parameters.get("texture_color").unwrap().clone();
        let texture_name = texture_parameter.as_str().unwrap();
        let texture = game_resources.get_engine_resources().get_texture_data(texture_name);
        let image_aspect = texture.borrow()._image_width as f32 / texture.borrow()._image_height as f32;

        let world_map_player = WorldMapPlayer::create_world_map_player(game_resources, player_layer_widget_mut, image_aspect);
        let world_map_stages = WorldMapWidget::create_world_map_stages(game_resources, stage_layer_widget_mut, bridge_layer_widget_mut, image_aspect);

        WorldMapWidget {
            _background_layout: background_layout.clone(),
            _world_map_widget: world_map_widget.clone(),
            _bridge_layer_widget: bridge_layer_widget.clone(),
            _stage_layer_widget: stage_layer_widget.clone(),
            _player_layer_widget: player_layer_widget.clone(),
            _image_aspect: image_aspect,
            _world_map_player: world_map_player,
            _world_map_stages: world_map_stages
        }
    }

    pub fn create_world_map_stages(game_resources: &GameResources<'a>, stage_layer: &mut WidgetDefault<'a>, bridge_layer: &mut WidgetDefault<'a>, image_aspect: f32) -> HashMap<String, Rc<WorldMapStage<'a>>> {
        // create stages
        let world_map_stage_home = WorldMapStage::create_world_map_stage(game_resources, stage_layer, "HOME", image_aspect);
        let world_map_stage_forest = WorldMapStage::create_world_map_stage(game_resources, stage_layer, "FOREST", image_aspect);
        let world_map_stage_cave = WorldMapStage::create_world_map_stage(game_resources, stage_layer, "CAVE", image_aspect);

        // link stages
        ptr_as_mut(world_map_stage_home.as_ref()).set_pos_hint(0.5, 0.5);
        ptr_as_mut(world_map_stage_home.as_ref()).set_linked_stage(game_resources, bridge_layer, WorldMapDirection::RIGHT, Some(world_map_stage_forest.clone()), image_aspect);
        ptr_as_mut(world_map_stage_home.as_ref()).set_linked_stage(game_resources, bridge_layer, WorldMapDirection::UP, Some(world_map_stage_cave.clone()), image_aspect);

        // register
        let mut world_map_stages = HashMap::new();
        world_map_stages.insert(world_map_stage_home._stage_name.clone(), world_map_stage_home);
        world_map_stages.insert(world_map_stage_forest._stage_name.clone(), world_map_stage_forest);
        world_map_stages.insert(world_map_stage_cave._stage_name.clone(), world_map_stage_cave);
        world_map_stages
    }

    pub fn set_visible(&mut self, visible: bool) {
        let ui_component = ptr_as_mut(self._background_layout.as_ref()).get_ui_component_mut();
        ui_component.set_visible(visible);
    }

    pub fn get_visible(&self) -> bool {
        let ui_component = ptr_as_ref(self._background_layout.as_ref()).get_ui_component();
        ui_component.get_visible()
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let world_map_widget = ptr_as_mut(self._world_map_widget.as_ref());
        let ui_component = world_map_widget.get_ui_component_mut();
        let window_aspect: f32 = window_size.x as f32 / window_size.y as f32;
        let world_map_size_hint: Vector2<f32> = if window_aspect < self._image_aspect {
            Vector2::new(1.0, window_aspect / self._image_aspect)
        } else {
            Vector2::new(self._image_aspect / window_aspect, 1.0)
        };

        ui_component.set_size_hint_x(Some(world_map_size_hint.x));
        ui_component.set_size_hint_y(Some(world_map_size_hint.y));
    }

    pub fn update_world_map(&mut self, _game_controller: &GameController, _delta_time: f32) {
    }
}
