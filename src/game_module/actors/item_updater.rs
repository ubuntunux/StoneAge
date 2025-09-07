use rust_engine_3d::scene::height_map::HeightMapData;
use crate::game_module::actors::items::{Item, ItemDataType};
use crate::game_module::game_constants::GRAVITY;

pub fn create_item_updater(item_type: ItemDataType) -> Box<dyn ItemUpdaterBase> {
    Box::new(ItemDefaultUpdater { })
}

pub trait ItemUpdaterBase {
    fn update_item_updater(&mut self, owner: &mut Item, height_map_data: &HeightMapData, delta_time: f64) {
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
}

pub struct ItemDefaultUpdater {
}

impl ItemUpdaterBase for ItemDefaultUpdater {
}

pub struct ItemSpiritBallUpdater {
}