use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum BlockDataType {
    Ground,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BlockCreateInfo {
    pub _block_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BlockData {
    pub _block_type: BlockDataType,
    pub _model_data_name: String,
    pub _max_hp: i32,
}

pub struct BlockProperties {
    pub _block_hp: f32,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Block<'a> {
    pub _block_name: String,
    pub _block_id: u64,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _block_properties: Box<BlockProperties>,
}

// Implementations
impl Default for BlockData {
    fn default() -> Self {
        BlockData {
            _block_type: BlockDataType::Ground,
            _model_data_name: String::new(),
            _max_hp: 0,
        }
    }
}

impl<'a> Block<'a> {
    pub fn create_block(
        block_id: u64,
        block_name: &str,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>) -> Block<'a> {
        let mut block = Block {
            _block_name: String::from(block_name),
            _block_id: block_id,
            _render_object: render_object.clone(),
            _block_properties: Box::from(BlockProperties {
                _block_hp: 100.0,
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
            }),
        };
        block.initialize_block();
        block
    }

    pub fn initialize_block(&mut self) {
        self.update_transform();

        // update for bounding box
        let mut render_object = self._render_object.borrow_mut();
        render_object.update_render_object_data(0.0);
    }

    pub fn get_block_id(&self) -> u64 {
        self._block_id
    }

    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_position_rotation_scale(
            &self._block_properties._position,
            &self._block_properties._rotation,
            &self._block_properties._scale,
        );
    }
}