use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::CharacterDataType;
use crate::game_module::behavior::behavior_civilian::BehaviorCivilian;
use crate::game_module::behavior::behavior_default::BehaviorDefault;
use crate::game_module::behavior::behavior_roamer::BehaviorRoamer;
use crate::game_module::behavior::behavior_ufo::BehaviorUfo;
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::RcRefCell;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy, Default)]
pub enum BehaviorState {
    #[default]
    None,
    Idle,
    Eating,
    Roaming,
    Interaction,
    Chase,
    Attack,
    Sleep,
    WakeUp,
}

pub struct BehaviorData<'a> {
    pub _behavior_time: f32,
    pub _behavior_state: BehaviorState,
    pub _next_behavior_state: BehaviorState,
    pub _is_force: bool,
    pub _move_direction: Vector3<f32>,
    pub _spawn_point: Vector3<f32>,
    pub _target_point: Vector3<f32>,
    pub _behavior_target: Option<RcRefCell<Character<'a>>>,
}

impl<'a> Default for BehaviorData<'a> {
    fn default() -> Self {
        Self {
            _behavior_time: 0.0,
            _behavior_state: BehaviorState::default(),
            _next_behavior_state: BehaviorState::default(),
            _is_force: false,
            _move_direction: Vector3::zeros(),
            _spawn_point: Vector3::zeros(),
            _target_point: Vector3::zeros(),
            _behavior_target: None,
        }
    }
}

impl<'a> BehaviorData<'a> {
    pub fn initialize_behavior_data(&mut self, spawn_point: &Vector3<f32>) {
        self._spawn_point = *spawn_point;
    }

    pub fn is_end_behavior_time(&self) -> bool {
        self._behavior_time <= 0.0
    }

    pub fn set_behavior_time(&mut self, behavior_time: f32) {
        self._behavior_time = behavior_time
    }

    pub fn update_behavior_time(&mut self, delta_time: f32) {
        self._behavior_time -= delta_time;
        if self._behavior_time < 0.0 {
            self._behavior_time = 0.0;
        }
    }

    pub fn is_force_behavior_state_changed(&self) -> bool {
        self._is_force
    }

    pub fn get_behavior_state(&self) -> BehaviorState {
        self._behavior_state
    }

    pub fn get_next_behavior_state(&self) -> BehaviorState {
        self._next_behavior_state
    }

    pub fn set_next_behavior_state(&mut self, next_behavior_state: BehaviorState, is_force: bool) {
        self._next_behavior_state = next_behavior_state;
        self._is_force = is_force;
    }

    pub fn set_behavior_state(&mut self, behavior_state: BehaviorState) {
        self._next_behavior_state = behavior_state;
        self._behavior_time = 0.0;
    }

    pub fn set_behavior_target(&mut self, behavior_target: Option<RcRefCell<Character<'a>>>) {
        self._behavior_target = behavior_target;
    }
}

pub trait BehaviorBase<'a> {
    fn initialize_behavior(&mut self, position: &Vector3<f32>);
    fn set_next_behavior(&mut self, next_behavior_state: BehaviorState, is_force: bool);
    fn update_behavior(&mut self, owner: &mut Character<'a>, behavior_target: Option<&Character<'a>>, delta_time: f32);
}

pub fn create_character_behavior<'a>(character_type: CharacterDataType) -> Box<dyn BehaviorBase<'a> + 'a> {
    match character_type {
        CharacterDataType::Civilian | CharacterDataType::Player => Box::new(BehaviorCivilian { ..Default::default() }),
        CharacterDataType::Roamer => Box::new(BehaviorRoamer { ..Default::default() }),
        CharacterDataType::Ufo => Box::new(BehaviorUfo { ..Default::default() }),
        _ => Box::new(BehaviorDefault { ..Default::default() }),
    }
}
