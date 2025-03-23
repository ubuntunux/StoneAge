use nalgebra::{Matrix4, Vector3};
use rust_engine_3d::scene::model::ModelData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::socket::Socket;
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
            _weapon_socket_name: String::new(),
            _weapon_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

impl<'a> Weapon<'a> {
    pub fn create_weapon(
        weapon_socket: &RcRefCell<Socket>,
        weapon_create_info: &WeaponCreateInfo,
        weapon_data: &RcRefCell<WeaponData<'a>>,
        render_object: &RcRefCell<RenderObjectData<'a>>
    ) -> Weapon<'a> {
        log::info!("create_weapon: {:?}, socket: {:?}", weapon_data.borrow()._model_data.borrow()._model_data_name, weapon_socket.borrow()._socket_data.borrow()._socket_name);
        Weapon {
            _weapon_socket: weapon_socket.clone(),
            _weapon_data: weapon_data.clone(),
            _render_object: render_object.clone(),
            _transform: math::make_srt_transform(
                &weapon_create_info._position,
                &weapon_create_info._rotation,
                &weapon_create_info._scale
            )
        }
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._bounding_box.collide_in_radius(pos)
    }

    pub fn update_weapon(&mut self, parent_transform: &Matrix4<f32>, _delta_time: f32) {
        let final_transform = parent_transform * self._transform;
        self._render_object.borrow_mut()._transform_object.set_transform(&final_transform);
    }
}
