use std::rc::Rc;
use crate::game_module::game_resource::GameResources;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{HorizontalAlign, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_constants::{MATERIAL_PORTRAIT_MONKEY_ARU, MATERIAL_WORLDMAP};
use crate::game_module::game_controller::GameController;
use crate::game_module::widgets::world_map_widget::{WorldMapBridge, WorldMapPlayer, WorldMapStage, WorldMapWidget};

impl<'a> WorldMapBridge<'a> {
    pub fn create_world_map_bridge(
        game_resources: &GameResources<'a>,
        root_layout: &mut WidgetDefault<'a>,
        stage_widget_a: &Rc<WidgetDefault<'a>>,
        stage_widget_b: &Rc<WidgetDefault<'a>>,
        world_map_aspect: f32
    ) -> Box<WorldMapBridge<'a>> {
        let world_map_bridge = UIManager::create_widget("world_map_bridge", UIWidgetTypes::Default);
        let PosHintX::Center(stage_widget_a_pos_hint_x) = stage_widget_a.as_ref()._ui_component.get_pos_hint_x() else { todo!() };
        let PosHintY::Center(stage_widget_a_pos_hint_y) = stage_widget_a.as_ref()._ui_component.get_pos_hint_y() else { todo!() };
        let PosHintX::Center(stage_widget_b_pos_hint_x) = stage_widget_b.as_ref()._ui_component.get_pos_hint_x() else { todo!() };
        let PosHintY::Center(stage_widget_b_pos_hint_y) = stage_widget_b.as_ref()._ui_component.get_pos_hint_y() else { todo!() };
        let size_hint_x = (stage_widget_a_pos_hint_x - stage_widget_b_pos_hint_x).abs();
        let size_hint_y = (stage_widget_a_pos_hint_y - stage_widget_b_pos_hint_y).abs();

        let ui_component = ptr_as_mut(world_map_bridge.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_color(get_color32(32, 32, 0, 255));
        ui_component.set_border_color(get_color32(255, 255, 128, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Center((stage_widget_a_pos_hint_x + stage_widget_b_pos_hint_x) / 2.0));
        ui_component.set_pos_hint_y(PosHintY::Center((stage_widget_a_pos_hint_y + stage_widget_b_pos_hint_y) / 2.0));

        let size_hint = 0.025;
        if size_hint_y < size_hint_x {
            ui_component.set_size_hint_x(Some(size_hint_x));
            ui_component.set_size_hint_y(Some(world_map_aspect * size_hint));
        } else {
            ui_component.set_size_hint_x(Some(size_hint));
            ui_component.set_size_hint_y(Some(size_hint_y));
        }

        root_layout.add_widget(&world_map_bridge);

        Box::new(WorldMapBridge {
            _bridge_widget: world_map_bridge,
            _stage_widget_a: stage_widget_a.clone(),
            _stage_widget_b: stage_widget_b.clone(),
        })
    }
}

impl<'a> WorldMapStage<'a> {
    pub fn create_world_map_stage(game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>, stage_name: &str, pos_hint: Vector2<f32>, world_map_aspect: f32) -> Box<WorldMapStage<'a>> {
        let world_map_stage = UIManager::create_widget("world_map_stage", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(10.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text(stage_name);
        ui_component.set_font_size(32.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(0,0,0,255));
        ui_component.set_pos_hint_x(PosHintX::Center(pos_hint.x));
        ui_component.set_pos_hint_y(PosHintY::Center(pos_hint.y));

        let size_hint = 0.05;
        ui_component.set_size_hint_x(Some(size_hint));
        ui_component.set_size_hint_y(Some(world_map_aspect * size_hint));
        root_layout.add_widget(&world_map_stage);

        Box::new(WorldMapStage {
            _world_map_stage: world_map_stage,
        })
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
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));

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
        let ui_component = ptr_as_mut(background_layout.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_color(get_color32(0, 0, 0, 255));
        ui_component.set_visible(false);
        root_widget.add_widget(&background_layout);

        let world_map_material_instance = game_resources.get_engine_resources().get_material_instance_data(MATERIAL_WORLDMAP);
        let world_map_widget = UIManager::create_widget("world_map_widget", UIWidgetTypes::Default);
        let world_map_widget_mut = ptr_as_mut(world_map_widget.as_ref());
        let ui_component = ptr_as_mut(world_map_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_material_instance(Some(world_map_material_instance.clone()));
        background_layout_mut.add_widget(&world_map_widget);

        let texture_parameter = world_map_material_instance.borrow()._material_parameters.get("texture_color").unwrap().clone();
        let texture_name = texture_parameter.as_str().unwrap();
        let texture = game_resources.get_engine_resources().get_texture_data(texture_name);
        let image_aspect = texture.borrow()._image_width as f32 / texture.borrow()._image_height as f32;

        let world_map_player = WorldMapPlayer::create_world_map_player(game_resources, world_map_widget_mut, image_aspect);
        let world_map_stages = WorldMapWidget::create_world_map_stages(game_resources, world_map_widget_mut, image_aspect);
        let world_map_bridges = WorldMapWidget::create_world_map_bridges(game_resources, world_map_widget_mut, &world_map_stages, image_aspect);

        WorldMapWidget {
            _background_layout: background_layout.clone(),
            _world_map_widget: world_map_widget.clone(),
            _image_aspect: image_aspect,
            _world_map_player: world_map_player,
            _world_map_stages: world_map_stages,
            _world_map_bridges: world_map_bridges,
        }
    }

    pub fn create_world_map_stages(game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>, image_aspect: f32) -> Vec<Box<WorldMapStage<'a>>> {
        let world_map_stage_a = WorldMapStage::create_world_map_stage(game_resources, root_layout, "HOME", Vector2::new(0.1, 0.3), image_aspect);
        let world_map_stage_b = WorldMapStage::create_world_map_stage(game_resources, root_layout, "FOREST", Vector2::new(0.6, 0.3), image_aspect);
        let world_map_stage_c = WorldMapStage::create_world_map_stage(game_resources, root_layout, "CAVE", Vector2::new(0.1, 0.6), image_aspect);
        vec![world_map_stage_a, world_map_stage_b, world_map_stage_c]
    }

    pub fn create_world_map_bridges(game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>, world_map_stages: &Vec<Box<WorldMapStage<'a>>>, image_aspect: f32) -> Vec<Box<WorldMapBridge<'a>>> {
        let world_map_bridge_a = WorldMapBridge::create_world_map_bridge(game_resources, root_layout, &world_map_stages[0].as_ref()._world_map_stage, &world_map_stages[1].as_ref()._world_map_stage, image_aspect);
        let world_map_bridge_b = WorldMapBridge::create_world_map_bridge(game_resources, root_layout, &world_map_stages[0].as_ref()._world_map_stage, &world_map_stages[2].as_ref()._world_map_stage, image_aspect);
        vec![world_map_bridge_a, world_map_bridge_b]
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

        // let world_map_size: Vector2<i32> = Vector2::new(
        //     (window_size.x as f32 * world_map_size_hint.x) as i32,
        //     (window_size.y as f32 * world_map_size_hint.y) as i32
        // );
    }

    pub fn update_world_map(&mut self, _game_controller: &GameController, _delta_time: f32) {
    }
}
