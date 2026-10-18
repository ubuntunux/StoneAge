use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::api::PropCreateInfo;
use crate::game_module::game_constants::{
    AUDIO_PICKUP_ITEM, AUDIO_WRONG, PLACEMENT_DISTANCE, PLACEMENT_ROTATION_SPEED,
};
use crate::game_module::game_service_locator::{
    get_game_resources, get_game_ui_manager_mut, get_item_manager_mut, get_prop_manager_mut,
};
use nalgebra::{Vector2, Vector3, Vector4};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager, get_scene_manager_mut};
use rust_engine_3d::renderer::push_constants::PushConstantParameter;
use rust_engine_3d::scene::collision::{CollisionData, CollisionType};
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_ref};
use uuid::Uuid;

pub const PREVIEW_BLINK_SPEED: f32 = 12.0;
pub const PREVIEW_BLINK_INTENSITY: f32 = 2.0;
pub const PLACEMENT_MIN_DISTANCE: f32 = 0.8;
pub const PLACEMENT_MAX_DISTANCE: f32 = 6.0;

pub struct PlacementPreview<'a> {
    pub _is_active: bool,
    pub _item_data_name: String,
    pub _prop_data_name: String,
    pub _render_object: Option<RcRefCell<RenderObjectData<'a>>>,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _yaw_rotation: f32,
    pub _yaw_offset: f32,
    pub _is_valid_location: bool,
    pub _blink_timer: f32,
    pub _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Default for PlacementPreview<'a> {
    fn default() -> Self {
        Self {
            _is_active: false,
            _item_data_name: String::new(),
            _prop_data_name: String::new(),
            _render_object: None,
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _yaw_rotation: 0.0,
            _yaw_offset: 0.0,
            _is_valid_location: false,
            _blink_timer: 0.0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a> PlacementPreview<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_active(&self) -> bool {
        self._is_active
    }

    pub fn is_valid_location(&self) -> bool {
        self._is_valid_location
    }

    pub fn get_item_data_name(&self) -> &str {
        &self._item_data_name
    }

    pub fn get_prop_data_name(&self) -> &str {
        &self._prop_data_name
    }

    pub fn start_placement(&mut self, item_data_name: &str, prop_data_name: &str, player: &Character<'a>) -> bool {
        self.cancel_placement();

        let game_resources = get_game_resources();
        let has_prop = game_resources.has_prop_data(prop_data_name);
        if !has_prop {
            log::warn!("Prop data not found for placement: {}", prop_data_name);
            return false;
        }

        let prop_data = game_resources.get_prop_data(prop_data_name);
        let model_data_name = prop_data.borrow()._model_data_name.clone();

        self._item_data_name = item_data_name.to_string();
        self._prop_data_name = prop_data_name.to_string();
        self._yaw_offset = 0.0;
        self._yaw_rotation = player.get_rotation().y;
        self._scale = Vector3::new(1.0, 1.0, 1.0);

        let face_dir = player.get_face_direction();
        let target_xz = player.get_position() + face_dir * PLACEMENT_DISTANCE;
        let ground_y = get_scene_manager().get_height_map_data().get_height_bilinear(&target_xz, 0);
        self._position = Vector3::new(target_xz.x, ground_y, target_xz.z);
        self._rotation = Vector3::new(player.get_rotation().x, self._yaw_rotation, player.get_rotation().z);

        let render_create_info = RenderObjectCreateInfo {
            _model_data_name: model_data_name,
            _position: self._position,
            _rotation: self._rotation,
            _scale: self._scale,
            ..Default::default()
        };

        let preview_name = format!("deploy_preview_{}", Uuid::new_v4());
        let render_object = get_scene_manager_mut().add_dynamic_render_object(
            &preview_name,
            &render_create_info,
            Some(CollisionType::NONE),
        );

        let model_collision = render_object.borrow().get_model_data().borrow()._collision.clone();
        if model_collision.is_valid_collision() {
            render_object.borrow_mut()._collision = model_collision;
        } else {
            let mut col = CollisionData::default();
            col._collision_type = CollisionType::BOX;
            col._bounding_box = render_object.borrow().get_bounding_box().clone();
            render_object.borrow_mut()._collision = col;
        }

        self._render_object = Some(render_object);
        self._is_active = true;
        self._is_valid_location = true;
        self._blink_timer = 0.0;

        self.update_placement(player, 0.0, 0.0);

        log::info!(
            "Started placement preview: item={}, prop={}",
            item_data_name,
            prop_data_name
        );
        true
    }

    pub fn update_placement(&mut self, player: &Character<'a>, delta_time: f32, rotate_input: f32) {
        if !self._is_active || self._render_object.is_none() {
            return;
        }

        self._blink_timer += delta_time;

        if rotate_input != 0.0 {
            self._yaw_offset += rotate_input * PLACEMENT_ROTATION_SPEED * delta_time;
            self._yaw_offset = self._yaw_offset % std::f32::consts::TAU;
        }

        self._yaw_rotation = (player.get_rotation().y + self._yaw_offset) % std::f32::consts::TAU;

        let player_pos = player.get_position();
        let face_dir = player.get_face_direction();
        let target_xz = player_pos + face_dir * PLACEMENT_DISTANCE;
        let height_map = get_scene_manager().get_height_map_data();
        let ground_y = height_map.get_height_bilinear(&target_xz, 0);

        self._position = Vector3::new(target_xz.x, ground_y, target_xz.z);
        self._rotation = Vector3::new(player.get_rotation().x, self._yaw_rotation, player.get_rotation().z);

        let dist_from_player =
            (Vector2::new(self._position.x, self._position.z) - Vector2::new(player_pos.x, player_pos.z)).magnitude();

        let is_valid_distance_and_ground =
            dist_from_player >= PLACEMENT_MIN_DISTANCE && dist_from_player <= PLACEMENT_MAX_DISTANCE && ground_y >= 0.0;

        if let Some(render_obj) = &self._render_object {
            let mut render_obj_mut = render_obj.borrow_mut();
            {
                let transform = render_obj_mut.get_transform_object_data_mut();
                transform.set_position(&self._position);
                transform.set_rotation(&self._rotation);
            }
            render_obj_mut.update_render_object_data(0.0);

            let preview_obj_id = render_obj_mut.get_object_id();
            let preview_collision = render_obj_mut.get_collision();

            let bound_min = math::get_min(
                &preview_collision._bounding_box._min,
                &preview_collision._bounding_box._max,
            );
            let bound_max = math::get_max(
                &preview_collision._bounding_box._min,
                &preview_collision._bounding_box._max,
            );

            let scene_manager = get_scene_manager();
            let collision_objects = scene_manager.collect_collision_objects(&bound_min, &bound_max);

            let mut has_collision = false;
            for collision_object in collision_objects.values() {
                let block_render_object = ptr_as_ref(collision_object.as_ptr());
                if block_render_object.get_object_id() == preview_obj_id {
                    continue;
                }
                if preview_collision.collide_collision(&block_render_object._collision) {
                    has_collision = true;
                    break;
                }
            }

            if !has_collision && preview_collision.collide_collision(player.get_collision()) {
                has_collision = true;
            }

            self._is_valid_location = is_valid_distance_and_ground && !has_collision;

            let blink_phase = 0.5 - (self._blink_timer * PREVIEW_BLINK_SPEED).cos() * 0.5;
            let color = if self._is_valid_location {
                // 노란색 깜빡임 (Yellow blink: R & G boost, B reduce)
                Vector4::new(
                    1.0 + blink_phase * PREVIEW_BLINK_INTENSITY,
                    1.0 + blink_phase * PREVIEW_BLINK_INTENSITY,
                    (1.0 - blink_phase * 0.8).max(0.0),
                    1.0,
                )
            } else {
                // 빨간색 깜빡임 (Red blink: R boost, G & B reduce)
                Vector4::new(
                    1.0 + blink_phase * PREVIEW_BLINK_INTENSITY,
                    (1.0 - blink_phase * 0.8).max(0.0),
                    (1.0 - blink_phase * 0.8).max(0.0),
                    1.0,
                )
            };

            render_obj_mut.set_push_constant_parameter("_color", &PushConstantParameter::Float4(color));
            render_obj_mut.set_push_constant_parameter("_Color", &PushConstantParameter::Float4(color));
        }
    }

    pub fn confirm_placement(&mut self) -> bool {
        if !self._is_active {
            return false;
        }

        if !self._is_valid_location {
            get_audio_manager_mut().play_audio_bank(AUDIO_WRONG, AudioLoop::ONCE, None);
            return false;
        }

        let prop_name = format!("{}_{}", self._prop_data_name.replace('/', "_"), Uuid::new_v4().simple());
        let prop_create_info = PropCreateInfo {
            _prop_data_name: self._prop_data_name.clone(),
            _position: self._position,
            _rotation: self._rotation,
            _scale: self._scale,
            ..Default::default()
        };

        get_prop_manager_mut().create_prop(&prop_name, &prop_create_info);
        get_item_manager_mut().remove_inventory_item(&self._item_data_name, 1);

        let game_ui = get_game_ui_manager_mut();
        let item_bar = game_ui.get_item_bar_widget_mut();
        item_bar.update_quick_slot_widgets();
        item_bar.update_item_selection_widget();

        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        log::info!("Confirmed placement of prop {} at {:?}", prop_name, self._position);

        self.cleanup();
        true
    }

    pub fn cancel_placement(&mut self) {
        if !self._is_active {
            return;
        }
        self.cleanup();
    }

    fn cleanup(&mut self) {
        if let Some(render_obj) = self._render_object.take() {
            let obj_id = render_obj.borrow().get_object_id();
            get_scene_manager_mut().remove_static_render_object(obj_id);
        }
        self._is_active = false;
        self._is_valid_location = false;
        self._blink_timer = 0.0;
        self._yaw_offset = 0.0;
        self._yaw_rotation = 0.0;
        self._item_data_name.clear();
        self._prop_data_name.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_module::actors::items::ItemDataType;
    use crate::game_module::game_constants::{PROP_DATA_CAMPFIRE, PROP_DATA_WORKTABLE};

    #[test]
    fn test_deployable_item_types() {
        assert!(ItemDataType::Campfire.is_deployable());
        assert!(ItemDataType::Worktable.is_deployable());
        assert!(!ItemDataType::Wood.is_deployable());
        assert!(!ItemDataType::Rock.is_deployable());
        assert!(!ItemDataType::FishingRod.is_deployable());

        assert_eq!(
            ItemDataType::Campfire.get_deploy_prop_data_name(),
            Some(PROP_DATA_CAMPFIRE)
        );
        assert_eq!(
            ItemDataType::Worktable.get_deploy_prop_data_name(),
            Some(PROP_DATA_WORKTABLE)
        );
        assert_eq!(ItemDataType::Wood.get_deploy_prop_data_name(), None);
    }

    #[test]
    fn test_placement_preview_initial_state() {
        let preview = PlacementPreview::new();
        assert!(!preview.is_active());
        assert!(!preview.is_valid_location());
        assert_eq!(preview.get_item_data_name(), "");
        assert_eq!(preview.get_prop_data_name(), "");
    }
}
