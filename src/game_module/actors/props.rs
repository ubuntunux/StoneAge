use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PropDataType {
    Rock,
    Tree
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PropCreateInfo {
    pub _prop_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PropData {
    pub _prop_type: PropDataType,
    pub _model_data_name: String,
    pub _max_hp: i32,
}

pub struct PropProperties {
    pub _prop_hp: f32,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Prop<'a> {
    pub _prop_name: String,
    pub _prop_id: u64,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _prop_properties: Box<PropProperties>,
}

// Implementations
impl Default for PropData {
    fn default() -> Self {
        PropData {
            _prop_type: PropDataType::Rock,
            _model_data_name: String::new(),
            _max_hp: 0,
        }
    }
}

impl<'a> Prop<'a> {
    pub fn create_prop(
        prop_id: u64,
        prop_name: &str,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>) -> Prop<'a> {
        let mut prop = Prop {
            _prop_name: String::from(prop_name),
            _prop_id: prop_id,
            _render_object: render_object.clone(),
            _prop_properties: Box::from(PropProperties {
                _prop_hp: 100.0,
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
            }),
        };
        prop.update_transform();
        prop.update_render_object();
        prop
    }

    pub fn get_prop_id(&self) -> u64 {
        self._prop_id
    }

    pub fn update_transform(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&self._prop_properties._position);
        render_object._transform_object.set_rotation(&self._prop_properties._rotation);
        render_object._transform_object.set_scale(&self._prop_properties._scale);
    }

    pub fn update_render_object(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object.update_render_object_data(0.0);
    }
}