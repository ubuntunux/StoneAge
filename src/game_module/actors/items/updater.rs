use crate::game_module::actors::items::api::{Item, ItemDataType};
use crate::game_module::game_constants::GRAVITY;
use nalgebra::Vector3;
use rust_engine_3d::scene::height_map::HeightMapData;

const SPIRIT_BALL_FLOATING_SPEED: f32 = 0.5;
const SPIRIT_BALL_FLOATING_RADIUS: f32 = 0.5;

pub fn create_item_updater(item_type: ItemDataType) -> Box<dyn ItemUpdaterBase> {
    match item_type {
        ItemDataType::EnergyBall | ItemDataType::SpiritBall => Box::new(ItemSpiritBallUpdater::default()),
        _ => Box::new(ItemDefaultUpdater::default()),
    }
}

pub trait ItemUpdaterBase {
    fn update_item_transform(&mut self, owner: &mut Item, height_map_data: &HeightMapData, delta_time: f64) {
        if owner.is_attachment() {
            owner.update_item_attach_transform();
        } else if !owner._item_properties._is_on_ground {
            let item_height = owner._render_object.borrow_mut()._bounding_box._extents.y;
            owner._item_properties._position += owner._item_properties._velocity * delta_time as f32;
            let ground_height = height_map_data.get_height_bilinear(&owner._item_properties._position, 0);
            if (owner._item_properties._position.y - item_height) <= ground_height
                && owner._item_properties._velocity.y <= 0.0
            {
                owner._item_properties._position.y = ground_height + item_height;
                owner._item_properties._is_on_ground = true;
            }
            owner._item_properties._velocity.y -= GRAVITY * delta_time as f32;
            owner.update_item_transform();
        }
    }

    fn update_item_updater(&mut self, owner: &mut Item, height_map_data: &HeightMapData, delta_time: f64);
}

#[derive(Default)]
pub struct ItemDefaultUpdater {}

impl ItemUpdaterBase for ItemDefaultUpdater {
    fn update_item_updater(&mut self, owner: &mut Item, height_map_data: &HeightMapData, delta_time: f64) {
        owner.update_pickup_delay_time(delta_time);
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
    fn update_item_updater(&mut self, owner: &mut Item, height_map_data: &HeightMapData, delta_time: f64) {
        if owner.is_attachment() {
            owner.update_item_attach_transform();
        } else {
            owner.update_pickup_delay_time(delta_time);

            if self._floating {
                let item_height = owner._render_object.borrow()._bounding_box._extents.y;
                let floating_speed = self._floating_timer * SPIRIT_BALL_FLOATING_SPEED;
                let floating_offset = Vector3::new(
                    floating_speed.cos() * SPIRIT_BALL_FLOATING_RADIUS,
                    floating_speed.sin() * SPIRIT_BALL_FLOATING_RADIUS + SPIRIT_BALL_FLOATING_RADIUS,
                    floating_speed.sin() * SPIRIT_BALL_FLOATING_RADIUS,
                );
                let mut next_position = self._spawn_point + floating_offset;
                let ground_height = height_map_data.get_height_bilinear(&next_position, 0);
                next_position.y = self._spawn_point.y.max(ground_height) + item_height + floating_offset.y;
                owner._item_properties._position = next_position;
                owner.update_item_transform();
                self._floating_timer += delta_time as f32;
            } else {
                let was_on_ground = owner._item_properties._is_on_ground;
                self.update_item_transform(owner, height_map_data, delta_time);
                if !was_on_ground && owner._item_properties._is_on_ground {
                    self._spawn_point = owner._item_properties._position;
                    self._floating_timer = 0.0;
                    self._floating = true;
                }
            }
        }
    }
}
