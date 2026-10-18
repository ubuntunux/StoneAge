use crate::game_module::actors::items::api::{
    Item, ItemCreateInfo, ItemData, ItemDataType, ItemID, ItemProperties, ItemSaveData,
};
use crate::game_module::actors::items::updater::create_item_updater;
use crate::game_module::game_constants::{
    DEFAULT_FOOD_HP, FOOD_HP_BASIC_COOKED, FOOD_HP_ENERGY_STEW, FOOD_HP_GOLDEN_FEAST, FOOD_MIN_SCALE_FACTOR,
    ITEM_BONE_SHIELD, ITEM_BOW, ITEM_CAMPFIRE, ITEM_COCONUT, ITEM_ENERGY_BALL, ITEM_ENERGY_STEW, ITEM_FISH_SOUP,
    ITEM_FISHING_ROD, ITEM_GOLDEN_FEAST, ITEM_HAND, ITEM_LEATHER_ARMOR, ITEM_MEAT, ITEM_NONE, ITEM_ROAST_MEAT,
    ITEM_ROCK, ITEM_SPEAR, ITEM_SPIRIT_BALL, ITEM_STEAMED_VEGETABLES, ITEM_STONE_AXE, ITEM_WOOD, ITEM_WOODEN_CLUB,
    ITEM_WORKTABLE, PROP_DATA_CAMPFIRE, PROP_DATA_WORKTABLE,
};
use nalgebra::Vector3;
use rust_engine_3d::scene::height_map::HeightMapData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::socket::Socket;
use rust_engine_3d::utilities::system::{RcRefCell, format_name_with_uuid, ptr_as_mut};
use strum::IntoEnumIterator;

const DEFAULT_ITEM_PICKUP_DELAY: f32 = 0.25;

pub fn get_default_food_hp(item_data_name: &str, item_type: ItemDataType) -> i32 {
    let resolved_type = if item_type == ItemDataType::Food || item_type == ItemDataType::None {
        ItemDataType::from_item_code(item_data_name).unwrap_or(item_type)
    } else {
        item_type
    };

    match resolved_type {
        ItemDataType::GoldenFeast => FOOD_HP_GOLDEN_FEAST,
        ItemDataType::EnergyStew => FOOD_HP_ENERGY_STEW,
        ItemDataType::RoastMeat | ItemDataType::FishSoup | ItemDataType::SteamedVegetables => FOOD_HP_BASIC_COOKED,
        _ => DEFAULT_FOOD_HP,
    }
}

impl Default for ItemCreateInfo {
    fn default() -> Self {
        ItemCreateInfo {
            _item_id: Default::default(),
            _item_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _pickup_delay: DEFAULT_ITEM_PICKUP_DELAY,
            _current_hp: 0,
            _max_hp: 0,
        }
    }
}

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            _item_type: ItemDataType::None,
            _model_data_name: String::new(),
            _name: String::new(),
            _ui_material_instance: String::new(),
            _description: String::new(),
            _item_effects: Vec::new(),
        }
    }
}

impl ItemDataType {
    pub fn from_item_code(item_code: &str) -> Option<ItemDataType> {
        ItemDataType::iter().find(|&t| {
            t != ItemDataType::None
                && t != ItemDataType::Food
                && t != ItemDataType::MeleeWeapon
                && t.item_code() == item_code
        })
    }

    pub fn item_code(&self) -> &'static str {
        match self {
            ItemDataType::None => ITEM_NONE,
            ItemDataType::Hand => ITEM_HAND,
            ItemDataType::Bow => ITEM_BOW,
            ItemDataType::EnergyBall => ITEM_ENERGY_BALL,
            ItemDataType::Food | ItemDataType::RoastMeat => ITEM_ROAST_MEAT,
            ItemDataType::Meat => ITEM_MEAT,
            ItemDataType::Coconut => ITEM_COCONUT,
            ItemDataType::Rock => ITEM_ROCK,
            ItemDataType::SpiritBall => ITEM_SPIRIT_BALL,
            ItemDataType::MeleeWeapon | ItemDataType::StoneAxe => ITEM_STONE_AXE,
            ItemDataType::Spear => ITEM_SPEAR,
            ItemDataType::Wood => ITEM_WOOD,
            ItemDataType::FishingRod => ITEM_FISHING_ROD,
            ItemDataType::LeatherArmor => ITEM_LEATHER_ARMOR,
            ItemDataType::BoneShield => ITEM_BONE_SHIELD,
            ItemDataType::Campfire => ITEM_CAMPFIRE,
            ItemDataType::Worktable => ITEM_WORKTABLE,
            ItemDataType::FishSoup => ITEM_FISH_SOUP,
            ItemDataType::SteamedVegetables => ITEM_STEAMED_VEGETABLES,
            ItemDataType::EnergyStew => ITEM_ENERGY_STEW,
            ItemDataType::GoldenFeast => ITEM_GOLDEN_FEAST,
            ItemDataType::WoodenClub => ITEM_WOODEN_CLUB,
        }
    }

    pub fn is_eatable(&self) -> bool {
        matches!(
            *self,
            ItemDataType::Food
                | ItemDataType::Meat
                | ItemDataType::Coconut
                | ItemDataType::RoastMeat
                | ItemDataType::FishSoup
                | ItemDataType::SteamedVegetables
                | ItemDataType::EnergyStew
                | ItemDataType::GoldenFeast
        )
    }

    pub fn is_droppable(&self) -> bool {
        *self != ItemDataType::Hand && *self != ItemDataType::None
    }

    pub fn is_weapon_item_type(&self) -> bool {
        matches!(
            *self,
            ItemDataType::Bow
                | ItemDataType::MeleeWeapon
                | ItemDataType::Spear
                | ItemDataType::StoneAxe
                | ItemDataType::WoodenClub
        )
    }

    pub fn is_fishing_item_type(&self) -> bool {
        *self == ItemDataType::FishingRod
    }

    pub fn is_deployable(&self) -> bool {
        matches!(*self, ItemDataType::Campfire | ItemDataType::Worktable)
    }

    pub fn get_deploy_prop_data_name(&self) -> Option<&'static str> {
        match *self {
            ItemDataType::Campfire => Some(PROP_DATA_CAMPFIRE),
            ItemDataType::Worktable => Some(PROP_DATA_WORKTABLE),
            _ => None,
        }
    }

    pub fn is_base_material(&self) -> bool {
        matches!(
            *self,
            ItemDataType::Wood
                | ItemDataType::Rock
                | ItemDataType::Coconut
                | ItemDataType::Meat
                | ItemDataType::EnergyBall
                | ItemDataType::SpiritBall
        )
    }
}

impl<'a> Item<'a> {
    pub fn create_item(
        item_id: ItemID,
        item_name: &str,
        item_data_name: &str,
        item_data: &RcRefCell<ItemData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        attach_socket: Option<RcRefCell<Socket>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
        velocity: &Vector3<f32>,
        pickup_delay: f32,
        current_hp: i32,
        max_hp: i32,
    ) -> Item<'a> {
        let (resolved_max_hp, resolved_current_hp) = if max_hp > 0 {
            (max_hp, if current_hp > 0 { current_hp } else { max_hp })
        } else if item_data.borrow()._item_type.is_eatable() {
            let hp = get_default_food_hp(item_data_name, item_data.borrow()._item_type);
            (hp, hp)
        } else {
            (1, 1)
        };

        let mut item = Item {
            _item_id: item_id,
            _item_name: item_name.to_string(),
            _item_data_name: item_data_name.to_string(),
            _item_data: item_data.clone(),
            _render_object: render_object.clone(),
            _attach_socket: attach_socket,
            _item_properties: Box::new(ItemProperties {
                _position: *position,
                _rotation: *rotation,
                _scale: *scale,
                _base_scale: *scale,
                _velocity: *velocity,
                _is_on_ground: false,
                _pickup_delay: pickup_delay,
                _current_hp: resolved_current_hp,
                _max_hp: resolved_max_hp,
            }),
            _item_updater: create_item_updater(item_data.borrow()._item_type),
        };

        item.initialize_item();
        item
    }

    pub fn load_item_save_data(&mut self, _item_create_info: &ItemSaveData) {}

    pub fn post_process_after_item_loading(&mut self) {}

    pub fn get_item_save_data(&self) -> (String, ItemSaveData) {
        (
            format_name_with_uuid(self._item_name.as_str(), self.get_item_id()),
            ItemSaveData {
                _item_create_info: ItemCreateInfo {
                    _item_id: self.get_item_id(),
                    _item_data_name: self._item_data_name.clone(),
                    _position: self._item_properties._position,
                    _rotation: self._item_properties._rotation,
                    _scale: self._item_properties._scale,
                    _velocity: self._item_properties._velocity,
                    _pickup_delay: self._item_properties._pickup_delay,
                    _current_hp: self._item_properties._current_hp,
                    _max_hp: self._item_properties._max_hp,
                },
            },
        )
    }

    pub fn get_current_hp(&self) -> i32 {
        self._item_properties._current_hp
    }

    pub fn get_max_hp(&self) -> i32 {
        self._item_properties._max_hp
    }

    pub fn set_current_hp(&mut self, hp: i32) {
        self._item_properties._current_hp = hp;
    }

    pub fn is_valid_table_food(&self) -> bool {
        self._item_data.borrow()._item_type.is_eatable()
            && (self._item_properties._max_hp == 0 || self._item_properties._current_hp > 0)
    }

    pub fn consume_food_charge(&mut self) -> (String, bool) {
        let item_data_name = self._item_data_name.clone();
        if self._item_properties._max_hp <= 0 {
            let hp = get_default_food_hp(&item_data_name, self._item_data.borrow()._item_type);
            self._item_properties._max_hp = hp;
            self._item_properties._current_hp = hp;
        }
        self._item_properties._current_hp = self._item_properties._current_hp.saturating_sub(1);
        let current_hp = self._item_properties._current_hp;
        let max_hp = self._item_properties._max_hp;

        if max_hp > 0 {
            let ratio = (current_hp as f32 / max_hp as f32).clamp(0.0, 1.0);
            let scale_factor = FOOD_MIN_SCALE_FACTOR + (1.0 - FOOD_MIN_SCALE_FACTOR) * ratio;
            self._item_properties._scale = self._item_properties._base_scale * scale_factor;
            self.update_item_transform();
        }

        (item_data_name, current_hp <= 0)
    }

    pub fn initialize_item(&mut self) {
        if self.is_attachment() {
            self.update_item_attach_transform();
        } else {
            self.update_item_transform();
        }
    }

    pub fn get_item_id(&self) -> ItemID {
        self._item_id
    }

    pub fn get_item_data_type(&self) -> ItemDataType {
        self._item_data.borrow()._item_type
    }

    pub fn get_item_data_name(&self) -> &String {
        &self._item_data_name
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self._item_properties._position
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._collision.collide_point(pos)
    }

    pub fn is_attachment(&self) -> bool {
        self._attach_socket.is_some()
    }

    pub fn pickable_item(&self) -> bool {
        self._item_properties._pickup_delay <= 0.0 && !self.is_attachment()
    }

    pub fn update_pickup_delay_time(&mut self, delta_time: f64) {
        if 0.0 < self._item_properties._pickup_delay {
            self._item_properties._pickup_delay -= delta_time as f32;
        }
    }

    pub fn update_item_attach_transform(&mut self) {
        let Some(attach_socket) = self._attach_socket.as_ref() else {
            return;
        };
        let render_object_ref = self._render_object.borrow();
        let transform = render_object_ref.get_transform_object_data_mut();
        if render_object_ref.has_animation() {
            let skeleton_transform = render_object_ref._mesh_data.borrow()._skeleton_data_list[0]._transform;
            let final_transform = attach_socket.borrow()._transform * skeleton_transform;
            transform.set_transform(&final_transform);
        } else {
            transform.set_transform(&attach_socket.borrow()._transform);
        }
    }

    pub fn update_item_transform(&mut self) {
        self._render_object.borrow().get_transform_object_data_mut().set_position_rotation_scale(
            &self._item_properties._position,
            &self._item_properties._rotation,
            &self._item_properties._scale,
        );
    }

    pub fn update_item(&mut self, height_map_data: &HeightMapData, delta_time: f64) {
        let owner = ptr_as_mut(self);
        self._item_updater.update_item_updater(owner, height_map_data, delta_time);
    }
}
