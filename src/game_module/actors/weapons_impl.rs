use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use crate::game_module::actors::weapons::{Weapon, WeaponCreateInfo, WeaponData, WeaponDataType, WeaponProperties};

impl WeaponDataType {
    pub fn get_weapon_material_instance_name(weapon_data_type: &WeaponDataType) -> &str {
        match weapon_data_type {
            WeaponDataType::WoodenClub => "ui/weapons/wooden_club",
        }
    }
}

impl Default for WeaponCreateInfo {
    fn default() -> Self {
        WeaponCreateInfo {
            _weapon_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for WeaponData {
    fn default() -> Self {
        WeaponData {
            _weapon_type: WeaponDataType::WoodenClub,
            _model_data_name: String::new(),
        }
    }
}

impl<'a> Weapon<'a> {
    pub fn create_weapon(
        weapon_id: u64,
        weapon_name: &str,
        weapon_data: &RcRefCell<WeaponData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>) -> Weapon<'a> {
        let mut weapon = Weapon {
            _weapon_name: String::from(weapon_name),
            _weapon_id: weapon_id,
            _weapon_data: weapon_data.clone(),
            _render_object: render_object.clone(),
            _weapon_properties: Box::from(WeaponProperties {
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
            }),
        };
        weapon.initialize_weapon();
        weapon
    }

    pub fn initialize_weapon(&mut self) {
        self.update_transform();
    }

    pub fn get_weapon_id(&self) -> u64 {
        self._weapon_id
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._bounding_box.collide_in_radius(pos)
    }

    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_transform(
            &self._weapon_properties._position,
            &self._weapon_properties._rotation,
            &self._weapon_properties._scale,
        );
    }

    pub fn update_weapon(&mut self, _delta_time: f64) {
    }
}
