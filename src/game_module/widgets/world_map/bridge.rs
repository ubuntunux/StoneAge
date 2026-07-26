use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::world_map::api::WorldMapBridge;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

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
        ui_component.set_pivot_vec(rust_engine_3d::scene::ui::PIVOT_CENTER);
        ui_component.set_pos(center.x, center.y);

        root_layout.add_widget(&world_map_bridge);

        world_map_bridge
    }
}
