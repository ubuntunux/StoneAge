use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::world_map::api::{WorldMapDirection, WorldMapStage};
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::WidgetDefault;
use std::collections::HashMap;
use std::rc::Rc;

pub trait WorldMapLayout<'a> {
    fn create_world_map_stages(
        world_map_widget: &Self,
        game_resources: &GameResources<'a>,
        stage_layer: &mut WidgetDefault<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        map_size: &Vector2<f32>,
    ) -> HashMap<String, Rc<WorldMapStage<'a>>>;

    fn set_linked_stage(
        game_resources: &GameResources<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        stage: &Rc<WorldMapStage<'a>>,
        linked_stage: &Rc<WorldMapStage<'a>>,
        direction: WorldMapDirection,
        map_size: &Vector2<f32>,
    );
}
