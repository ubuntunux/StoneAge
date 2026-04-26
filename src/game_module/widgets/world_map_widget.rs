use crate::game_module::game_resource::GameResources;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;
use crate::game_module::game_constants::{MATERIAL_PORTRAIT_MONKEY_ARU, MATERIAL_WORLDMAP};
use crate::game_module::game_controller::GameController;

pub struct WorldMapBridge<'a> {
    pub _bridge_widget: Rc<WidgetDefault<'a>>,
    pub _stage_widget_a: Rc<WidgetDefault<'a>>,
    pub _stage_widget_b: Rc<WidgetDefault<'a>>
}

pub struct WorldMapStage<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _world_map_icon: Rc<WidgetDefault<'a>>,
    pub _world_map_text: Rc<WidgetDefault<'a>>,
}

pub struct WorldMapPlayer<'a> {
    pub _player_icon: Rc<WidgetDefault<'a>>,
    pub _player_icon_pos: Vector2<f32>
}

pub struct WorldMapWidget<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _world_map_widget: Rc<WidgetDefault<'a>>,
    pub _image_aspect: f32,
    pub _world_map_player: Box<WorldMapPlayer<'a>>,
    pub _world_map_stages: Vec<Box<WorldMapStage<'a>>>,
    pub _world_map_bridges: Vec<Box<WorldMapPlayer<'a>>>,
}

impl<'a> WorldMapPlayer<'a> {
    pub fn create_world_map_player(game_resources: &GameResources<'a>, root_layout: &mut WidgetDefault<'a>) -> Box<WorldMapPlayer<'a>> {
        let material_instance = game_resources.get_engine_resources().get_material_instance_data(MATERIAL_PORTRAIT_MONKEY_ARU);

        let player_icon = UIManager::create_widget("player_icon", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_round(10.0);
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_material_instance(Some(material_instance.clone()));
        root_layout.add_widget(&player_icon);

        Box::new(WorldMapPlayer {
            _player_icon: player_icon,
            _player_icon_pos: Vector2::zeros()
        })
    }

    pub fn changed_world_map_size(&mut self, world_map_size: &Vector2<i32>) {
        let aspect = world_map_size.x as f32 / world_map_size.y as f32;
        let size_hint = 0.05;
        let ui_component = ptr_as_mut(self._player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(size_hint));
        ui_component.set_size_hint_y(Some(aspect * size_hint));
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
    }
}


impl<'a> WorldMapWidget<'a> {
    pub fn create_world_map_widget(
        root_widget: &mut WidgetDefault<'a>,
        game_resources: &GameResources<'a>,
        _window_size: &Vector2<i32>
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

        let world_map_player = WorldMapPlayer::create_world_map_player(game_resources, world_map_widget_mut);

        WorldMapWidget {
            _background_layout: background_layout.clone(),
            _world_map_widget: world_map_widget.clone(),
            _image_aspect: image_aspect,
            _world_map_player: world_map_player,
            _world_map_stages: vec![],
            _world_map_bridges: vec![],
        }
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

        let world_map_size: Vector2<i32> = Vector2::new(
            (window_size.x as f32 * world_map_size_hint.x) as i32,
            (window_size.y as f32 * world_map_size_hint.y) as i32
        );

        self._world_map_player.changed_world_map_size(&world_map_size);
    }

    pub fn update_world_map(&mut self, _game_controller: &GameController, _delta_time: f32) {
    }
}
