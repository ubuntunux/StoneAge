use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::character::character::*;

impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::UrsusArctos,
            _model_data_name: "".to_string(),
            _max_hp: 100,
        }
    }
}

impl CharacterProperty {
    pub fn create_character_property() -> CharacterProperty {
        CharacterProperty {
            _hp: 0.0,
        }
    }
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

impl Character {
    pub fn create_character_instance(
        character_id: u64,
        character_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Character {
        Character {
            _character_id: character_id,
            _character_name: String::from(character_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
        }
    }
    pub fn get_character_id(&self) -> u64 { self._character_id }
    pub fn get_character_controller(&self) -> &CharacterController {
        &self._controller
    }
    pub fn get_character_controller_mut(&mut self) -> &mut CharacterController { &mut self._controller }
    pub fn update_transform(&mut self) {
        let controller = self.get_character_controller();
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&controller._position);
        render_object._transform_object.set_rotation(&controller._rotation);
        render_object._transform_object.set_scale(&controller._scale);
    }

    pub fn update_character(&mut self, _delta_time: f32) {
        self.update_transform();
    }
}