use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::character::character::{Character, CharacterController, CharacterData, CharacterDataType, CharacterProperty};

impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::AnkyloSaurus,
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
            _transform_object: TransformObjectData::create_transform_object_data(),
        }
    }
}



impl Character {
    pub fn create_character_instance(
        character_name: &String,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Character {
        Character {
            _character_name: character_name.clone(),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
        }
    }

    pub fn get_character_transform(&self) -> &TransformObjectData {
        &self._controller._transform_object
    }

    pub fn get_character_transform_mut(&mut self) -> &mut TransformObjectData {
        &mut self._controller._transform_object
    }

    pub fn update_transform(&mut self) {
        let controller = self.get_character_transform();
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&controller._position);
        render_object._transform_object.set_rotation(&controller._rotation);
        render_object._transform_object.set_scale(&controller._scale);
    }

    pub fn update_character(&mut self, _delta_time: f32) {
        self.update_transform();
    }
}