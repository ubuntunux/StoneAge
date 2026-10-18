use crate::game_module::actors::common::{
    calculate_hit_shake_position, play_hit_effect_and_audio, update_actor_hit_blink_color,
};
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::actors::props::api::{
    Prop, PropCreateInfo, PropData, PropDataType, PropID, PropManager, PropSaveData, PropStats,
};
use crate::game_module::game_constants::{GAME_VIEW_MODE, GameViewMode, HIT_BLINK_TIME};
use nalgebra::Vector3;
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::{CollisionData, CollisionType};
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, format_name_with_uuid, ptr_as_ref};

const HARVESTABLE_DROP_SPEED: f32 = 2.0;
const DESTRUCTION_DROP_HORIZONTAL_SPEED: f32 = 5.0;
const DESTRUCTION_DROP_VERTICAL_SPEED: f32 = 10.0;

impl Default for PropData {
    fn default() -> Self {
        PropData {
            _prop_type: PropDataType::None,
            _model_data_name: String::new(),
            _name: String::new(),
            _max_hp: 0,
            _item_data_name: String::new(),
            _item_drop_count_max: 1,
            _item_drop_count_min: 1,
            _item_regenerate_time: 0.0,
        }
    }
}

impl PropData {
    pub fn get_item_drop_count(&self) -> i32 {
        rand::random_range(self._item_drop_count_min..=self._item_drop_count_max)
    }
}

impl<'a> Prop<'a> {
    pub fn create_prop(
        prop_manager: *const PropManager<'a>,
        prop_id: PropID,
        prop_name: &str,
        prop_data: &RcRefCell<PropData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        item_render_objects: Vec<RcRefCell<RenderObjectData<'a>>>,
        prop_create_info: &PropCreateInfo,
    ) -> Prop<'a> {
        let prop_data_ref = prop_data.borrow();
        let item_count = prop_data_ref.get_item_drop_count();
        let mut prop = Prop {
            _prop_id: prop_id,
            _prop_name: String::from(prop_name),
            _prop_data_name: prop_create_info._prop_data_name.clone(),
            _prop_radius: render_object.borrow()._collision._bounding_box.get_mag_xz(),
            _prop_manager: prop_manager,
            _render_object: render_object.clone(),
            _item_render_objects: item_render_objects,
            _prop_data: prop_data.clone(),
            _prop_stats: Box::from(PropStats {
                _is_alive: false,
                _prop_hp: prop_data_ref._max_hp,
                _item_regenerate_time: 0.0,
                _item_count_max: item_count,
                _item_count: item_count,
                _position: prop_create_info._position,
                _rotation: prop_create_info._rotation,
                _scale: prop_create_info._scale,
                _hit_blink_time: 0.0,
            }),
            _instance_parameters: prop_create_info._instance_parameters.clone(),
            _deployed_item_data_name: prop_create_info._deployed_item_data_name.clone(),
        };

        if prop_data_ref._prop_type == PropDataType::Ceiling || prop_data_ref._prop_type == PropDataType::Gate {
            render_object.borrow_mut().set_collision_type(CollisionType::NONE);
        }

        prop.initialize_prop();
        prop
    }

    pub fn is_deployed_prop(&self) -> bool {
        !self._deployed_item_data_name.is_empty()
    }

    pub fn initialize_prop(&mut self) {
        self._prop_stats._is_alive = true;
        self._prop_stats._item_regenerate_time = 0.0;
        self._prop_stats._hit_blink_time = 0.0;
        self.update_item_visible();
        self.update_transform();
    }

    pub fn load_prop_save_data(&mut self, _prop_save_data: &PropSaveData) {}

    pub fn get_prop_save_data(&self) -> (String, PropSaveData) {
        (
            format_name_with_uuid(self._prop_name.as_str(), self.get_prop_id()),
            PropSaveData {
                _prop_create_info: PropCreateInfo {
                    _prop_id: self.get_prop_id(),
                    _prop_data_name: self._prop_data_name.clone(),
                    _position: self._prop_stats._position,
                    _rotation: self._prop_stats._rotation,
                    _scale: self._prop_stats._scale,
                    _instance_parameters: self._instance_parameters.clone(),
                    _deployed_item_data_name: self._deployed_item_data_name.clone(),
                },
            },
        )
    }

    pub fn post_process_after_prop_loading(&mut self) {}

    pub fn get_prop_id(&self) -> PropID {
        self._prop_id
    }

    pub fn get_prop_manager(&self) -> &PropManager<'a> {
        ptr_as_ref(self._prop_manager)
    }

    pub fn get_instance_parameters(&self, key: &str) -> Option<serde_json::Value> {
        self._instance_parameters.get(key).cloned()
    }

    pub fn can_drop_item(&self) -> bool {
        0 < self._prop_stats._item_count
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self._prop_stats._position
    }

    pub fn set_position(&mut self, position: &Vector3<f32>) {
        self._prop_stats._position.clone_from(position);
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &ptr_as_ref(self._render_object.as_ptr())._bounding_box
    }

    pub fn get_collision(&self) -> &CollisionData {
        &ptr_as_ref(self._render_object.as_ptr())._collision
    }

    pub fn is_alive(&self) -> bool {
        self._prop_stats._is_alive
    }

    pub fn is_heat_source(&self) -> bool {
        self._prop_data.borrow()._prop_type.is_heat_source()
    }

    pub fn is_shelter(&self) -> bool {
        self._prop_data.borrow()._prop_type.is_shelter()
    }

    pub fn set_dead(&mut self) {
        self._prop_stats._is_alive = false;
    }

    pub fn set_hit_damage(&mut self, damage: i32) {
        if PropDataType::Harvestable != self._prop_data.borrow()._prop_type {
            self._prop_stats._prop_hp -= damage;
            if self._prop_stats._prop_hp <= 0 {
                self.set_dead();
            }
        }

        self._prop_stats._hit_blink_time = HIT_BLINK_TIME;
        let center = *self.get_bounding_box().get_center();
        play_hit_effect_and_audio(&center);
    }

    pub fn update_generate_item(&mut self, delta_time: f64) {
        if self._prop_data.borrow()._prop_type == PropDataType::Harvestable
            && self._prop_stats._item_count < self._prop_stats._item_count_max
        {
            if self._prop_data.borrow()._item_regenerate_time <= self._prop_stats._item_regenerate_time {
                self._prop_stats._item_count += 1;
                self._prop_stats._item_regenerate_time = 0.0;
                self.update_item_visible();
            } else {
                self._prop_stats._item_regenerate_time += delta_time as f32;
            }
        }
    }

    pub fn drop_items(&mut self, mut drop_count: i32, player_position: &Vector3<f32>) -> Vec<ItemCreateInfo> {
        if self._prop_stats._item_count <= drop_count {
            drop_count = self._prop_stats._item_count;
            self._prop_stats._item_count = 0;
        } else {
            self._prop_stats._item_count -= drop_count;
        }
        self.update_item_visible();

        let mut item_create_infos: Vec<ItemCreateInfo> = vec![];
        let prop_type = self._prop_data.borrow()._prop_type;
        let item_data_name = self._prop_data.borrow()._item_data_name.clone();

        for drop_index in 0..drop_count {
            let mut position = *self.get_bounding_box().get_center();
            let mut velocity = Vector3::zeros();
            if let Some(item_render_object) =
                self._item_render_objects.get((self._prop_stats._item_count + drop_index) as usize)
            {
                match prop_type {
                    PropDataType::Harvestable => {
                        position = item_render_object.borrow().get_transform_object_data()._position;
                        let to_item = math::make_normalize_xz(&(position - self.get_position()));
                        velocity = Vector3::new(to_item.x, 1.0, to_item.z) * HARVESTABLE_DROP_SPEED;
                    }
                    _ => {
                        velocity = Vector3::new(
                            (rand::random::<f32>() * 2.0 - 1.0) * DESTRUCTION_DROP_HORIZONTAL_SPEED,
                            rand::random::<f32>() * DESTRUCTION_DROP_VERTICAL_SPEED,
                            (rand::random::<f32>() * 2.0 - 1.0) * DESTRUCTION_DROP_HORIZONTAL_SPEED,
                        );
                    }
                };
            }

            if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                position.z = player_position.z;
                velocity.z = 0.0;
            }

            item_create_infos.push(ItemCreateInfo {
                _item_data_name: item_data_name.clone(),
                _position: position,
                _velocity: velocity,
                ..Default::default()
            });
        }
        item_create_infos
    }

    pub fn update_item_visible(&mut self) {
        for (i, item_render_object) in self._item_render_objects.iter().enumerate() {
            item_render_object.borrow_mut().set_visible(i < self._prop_stats._item_count as usize);
        }
    }

    pub fn update_transform(&mut self) {
        let position = calculate_hit_shake_position(&self._prop_stats._position, self._prop_stats._hit_blink_time);
        self._render_object.borrow().get_transform_object_data_mut().set_position_rotation_scale(
            &position,
            &self._prop_stats._rotation,
            &self._prop_stats._scale,
        );
    }

    pub fn update_render_object(&mut self) {
        self._render_object.borrow_mut().update_render_object_data(0.0);
    }

    pub fn update_prop(&mut self, delta_time: f64) {
        if 0.0 < self._prop_stats._hit_blink_time {
            update_actor_hit_blink_color(
                &self._render_object,
                &mut self._prop_stats._hit_blink_time,
                delta_time as f32,
            );
        }
        self.update_generate_item(delta_time);
        self.update_transform();
    }
}
