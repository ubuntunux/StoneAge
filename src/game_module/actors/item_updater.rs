use crate::game_module::actors::items::{Item, ItemDataType};
use crate::game_module::game_constants::GRAVITY;
use nalgebra::Vector3;
use rust_engine_3d::scene::height_map::HeightMapData;

pub fn create_item_updater(item_type: ItemDataType) -> Box<dyn ItemUpdaterBase> {
    match item_type {
        ItemDataType::SpiritBall => Box::new(ItemSpiritBallUpdater::default()),
        _ => Box::new(ItemDefaultUpdater::default()),
    }
}

pub trait ItemUpdaterBase {
    fn update_item_transform(
        &mut self,
        owner: &mut Item,
        height_map_data: &HeightMapData,
        delta_time: f64,
    ) {
        if owner._item_properties._is_on_ground == false {
            let item_height = owner._render_object.borrow_mut()._bounding_box._extents.y;
            owner._item_properties._position += owner._item_properties._velocity * delta_time as f32;
            let ground_height = height_map_data.get_height_bilinear(&owner._item_properties._position, 0);
            if (owner._item_properties._position.y - item_height) <= ground_height && owner._item_properties._velocity.y <= 0.0 {
                owner._item_properties._position.y = ground_height + item_height;
                owner._item_properties._is_on_ground = true;
            }
            owner._item_properties._velocity.y -= GRAVITY * delta_time as f32;

            owner.update_transform();
        }
    }
    fn update_item_updater(
        &mut self,
        owner: &mut Item,
        height_map_data: &HeightMapData,
        delta_time: f64,
    );
}

#[derive(Default)]
pub struct ItemDefaultUpdater {}

impl ItemUpdaterBase for ItemDefaultUpdater {
    fn update_item_updater(
        &mut self,
        owner: &mut Item,
        height_map_data: &HeightMapData,
        delta_time: f64,
    ) {
        self.update_item_transform(owner, height_map_data, delta_time);
    }
}

#[derive(Default)]
pub struct ItemSpiritBallUpdater {
    pub _spawn_point: Vector3<f32>,
    pub _floating: bool,
    pub _floating_timer: f32,
}

impl ItemUpdaterBase for ItemSpiritBallUpdater {
    fn update_item_updater(
        &mut self,
        owner: &mut Item,
        height_map_data: &HeightMapData,
        delta_time: f64,
    ) {
        if self._floating {
            let floating_speed = self._floating_timer * 0.5;
            let floating_radius = 0.5;
            let floating_offset = Vector3::new(
                floating_speed.cos() * floating_radius,
                floating_speed.sin() * floating_radius,
                floating_speed.sin() * floating_radius,
            );
            owner._item_properties._position = self._spawn_point + floating_offset;
            owner.update_transform();
            self._floating_timer += delta_time as f32;
        } else {
            let was_on_ground = owner._item_properties._is_on_ground;
            self.update_item_transform(owner, height_map_data, delta_time);
            if was_on_ground == false && owner._item_properties._is_on_ground {
                self._spawn_point = owner._item_properties._position;
                self._floating_timer = 0.0;
                self._floating = true;
            }
        }
    }
}
