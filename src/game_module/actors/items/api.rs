use crate::game_module::actors::items::updater::ItemUpdaterBase;
use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::socket::Socket;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

pub type ItemID = Uuid;
pub type ItemMap<'a> = HashMap<ItemID, RcRefCell<Item<'a>>>;
pub type ItemNameMap<'a> = HashMap<String, RcRefCell<Item<'a>>>;

#[derive(Serialize, Deserialize, Hash, Eq, Clone, Copy, Debug, EnumIter, Display, PartialEq, Default)]
pub enum ItemDataType {
    #[default]
    None,
    Hand,
    Bow,
    EnergyBall,
    Food,
    Rock,
    SpiritBall,
    MeleeWeapon,
    Spear,
    Wood,
    FishingRod,
    Meat,
    Coconut,
    StoneAxe,
    LeatherArmor,
    BoneShield,
    Campfire,
    Worktable,
    RoastMeat,
    FishSoup,
    SteamedVegetables,
    EnergyStew,
    GoldenFeast,
    WoodenClub,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ItemCreateInfo {
    pub _item_id: ItemID,
    pub _item_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _pickup_delay: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
pub struct ItemSaveData {
    pub _item_create_info: ItemCreateInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemEffect {
    WeaponDamage(f32),
    WeaponRange(f32),
    Hp(i32),
    Stamina(f32),
    Hunger(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ItemData {
    pub _item_type: ItemDataType,
    pub _model_data_name: String,
    pub _name: String,
    pub _ui_material_instance: String,
    pub _description: String,
    pub _item_effects: Vec<ItemEffect>,
}

impl ItemData {
    pub fn get_weapon_damage(&self) -> f32 {
        for effect in &self._item_effects {
            if let ItemEffect::WeaponDamage(damage) = effect {
                return *damage;
            }
        }
        0.0
    }

    pub fn get_weapon_range(&self) -> f32 {
        for effect in &self._item_effects {
            if let ItemEffect::WeaponRange(range) = effect {
                return *range;
            }
        }
        0.0
    }

    pub fn get_hp(&self) -> i32 {
        for effect in &self._item_effects {
            if let ItemEffect::Hp(hp) = effect {
                return *hp;
            }
        }
        0
    }

    pub fn get_stamina(&self) -> f32 {
        for effect in &self._item_effects {
            if let ItemEffect::Stamina(stamina) = effect {
                return *stamina;
            }
        }
        0.0
    }

    pub fn get_hunger(&self) -> f32 {
        for effect in &self._item_effects {
            if let ItemEffect::Hunger(hunger) = effect {
                return *hunger;
            }
        }
        0.0
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct ItemProperties {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_on_ground: bool,
    pub _pickup_delay: f32,
}

pub struct Item<'a> {
    pub _item_id: ItemID,
    pub _item_name: String,
    pub _item_data_name: String,
    pub _item_data: RcRefCell<ItemData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _attach_socket: Option<RcRefCell<Socket>>,
    pub _item_properties: Box<ItemProperties>,
    pub _item_updater: Box<dyn ItemUpdaterBase>,
}

pub struct ItemManager<'a> {
    pub _items: ItemMap<'a>,
    pub _item_name_map: ItemNameMap<'a>,
    pub _marker: std::marker::PhantomData<&'a ()>,
}
