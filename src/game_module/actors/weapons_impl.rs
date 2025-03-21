use nalgebra::Vector3;
use rust_engine_3d::scene::model::ModelData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::RcRefCell;
use crate::game_module::actors::weapons::{Weapon, WeaponCreateInfo, WeaponData, WeaponDataCreateInfo, WeaponDataType};

impl Default for WeaponDataType {
    fn default() -> Self {
        WeaponDataType::None
    }
}

impl WeaponDataType {
    pub fn get_weapon_material_instance_name(weapon_data_type: &WeaponDataType) -> &str {
        match weapon_data_type {
            WeaponDataType::WoodenClub => "ui/weapons/wooden_club",
            _ => ""
        }
    }
}

impl Default for WeaponDataCreateInfo {
    fn default() -> Self {
        WeaponDataCreateInfo {
            _damage: 10.0,
            _model_data_name: String::new(),
            _weapon_data_type: WeaponDataType::None
        }
    }
}

impl<'a> WeaponData<'a> {
    pub fn create_weapon_data(weapon_data_create_info: &WeaponDataCreateInfo, weapon_model_data: &RcRefCell<ModelData<'a>>) -> Self {
        WeaponData {
            _damage: weapon_data_create_info._damage,
            _model_data: weapon_model_data.clone(),
            _weapon_data_type: weapon_data_create_info._weapon_data_type,
        }
    }
}

impl Default for WeaponCreateInfo {
    fn default() -> Self {
        WeaponCreateInfo {
            _weapon_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

impl<'a> Weapon<'a> {
    pub fn create_weapon(
        weapon_create_info: &WeaponCreateInfo,
        weapon_data: &RcRefCell<WeaponData<'a>>,
        render_object: &RcRefCell<RenderObjectData<'a>>
    ) -> Weapon<'a> {
        let mut weapon = Weapon {
            _weapon_data: weapon_data.clone(),
            _render_object: render_object.clone(),
            _transform: math::make_srt_transform(
                &weapon_create_info._position,
                &weapon_create_info._rotation,
                &weapon_create_info._scale
            )
        };
        weapon.initialize_weapon();
        weapon
    }

    pub fn initialize_weapon(&mut self) {
        self.update_transform();
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._bounding_box.collide_in_radius(pos)
    }

    pub fn update_transform(&mut self) {
        // self._render_object.borrow_mut()._transform_object.set_transform(
        //     &self._weapon_properties._position,
        //     &self._weapon_properties._rotation,
        //     &self._weapon_properties._scale,
        // );
    }

    pub fn update_weapon(&mut self, _delta_time: f64) {
    }
}
