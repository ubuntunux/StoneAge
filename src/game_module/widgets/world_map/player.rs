use crate::game_module::game_constants::MATERIAL_PORTRAIT_MONKEY_ARU;
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::world_map::api::{WorldMapPlayer, WorldMapWidget};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

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
        ui_component.set_pivot_vec(rust_engine_3d::scene::ui::PIVOT_CENTER);
        ui_component.set_pos(0.0, 0.0);
        root_layout.add_widget(&player_icon);

        Box::new(WorldMapPlayer {
            _world_map_widget: world_map_widget,
            _player_icon: player_icon,
        })
    }

    pub fn set_center_pos(&mut self, center_x: f32, center_y: f32) {
        let ui_component = ptr_as_mut(self._player_icon.as_ref()).get_ui_component_mut();
        ui_component.set_pos(center_x, center_y);
    }
}
