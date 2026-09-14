use crate::game_module::game_constants::{
    LANTERN_FADE_SPEED, LANTERN_LIGHT_COLOR, LANTERN_LIGHT_RADIUS, LANTERN_OFFSET_Y,
};
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::get_scene_manager_mut;
use rust_engine_3d::scene::light::{PointLight, PointLightCreateInfo};
use rust_engine_3d::utilities::system::RcRefCell;

pub struct Lantern {
    pub _point_light: Option<RcRefCell<PointLight>>,
    pub _is_on: bool,
    pub _light_color: Vector3<f32>,
    pub _max_radius: f32,
    pub _current_radius: f32,
    pub _offset: Vector3<f32>,
}

impl Lantern {
    pub fn create_lantern(name: &str, position: &Vector3<f32>) -> Lantern {
        let mut lantern = Lantern {
            _point_light: None,
            _is_on: false,
            _light_color: Vector3::new(
                LANTERN_LIGHT_COLOR[0],
                LANTERN_LIGHT_COLOR[1],
                LANTERN_LIGHT_COLOR[2],
            ),
            _max_radius: LANTERN_LIGHT_RADIUS,
            _current_radius: 0.0,
            _offset: Vector3::new(0.0, LANTERN_OFFSET_Y, 0.0),
        };
        lantern.initialize_lantern(name, position);
        lantern
    }

    pub fn initialize_lantern(&mut self, name: &str, position: &Vector3<f32>) {
        let light_position = position + self._offset;
        let light_create_info = PointLightCreateInfo {
            _light_position: light_position,
            _radius: self._current_radius,
            _light_color: self._light_color,
        };
        let point_light = get_scene_manager_mut().add_point_light_object(name, &light_create_info);
        self._point_light = Some(point_light);
    }

    pub fn update_lantern(&mut self, owner_position: &Vector3<f32>, is_night: bool, delta_time: f32) {
        if is_night {
            self.turn_on();
        } else {
            self.turn_off();
        }

        let target_radius = if self._is_on { self._max_radius } else { 0.0 };
        let radius_diff = target_radius - self._current_radius;
        self._current_radius += radius_diff * (delta_time * LANTERN_FADE_SPEED).min(1.0);

        if let Some(point_light) = &self._point_light {
            let mut light_ref = point_light.borrow_mut();
            light_ref._light_data._light_position = owner_position + self._offset;
            light_ref._light_data._radius = self._current_radius;
            light_ref._light_data._light_color = self._light_color;
        }
    }

    pub fn turn_on(&mut self) {
        self._is_on = true;
    }

    pub fn turn_off(&mut self) {
        self._is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self._is_on
    }

    pub fn get_current_radius(&self) -> f32 {
        self._current_radius
    }

    pub fn destroy_lantern(&mut self) {
        if let Some(point_light) = self._point_light.take() {
            let object_id = point_light.borrow()._object_id;
            get_scene_manager_mut().remove_point_light_object(object_id);
        }
    }
}

impl Drop for Lantern {
    fn drop(&mut self) {
        self.destroy_lantern();
    }
}
