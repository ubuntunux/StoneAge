use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorState};
use rust_engine_3d::utilities::system::State;

#[derive(Default)]
pub struct BehaviorUfo<'a> {
    pub _behavior_data: BehaviorData<'a>,
}

impl<'a> BehaviorBase<'a> for BehaviorUfo<'a> {
    fn get_behavior_data(&self) -> &BehaviorData<'a> {
        &self._behavior_data
    }

    fn get_behavior_data_mut(&mut self) -> &mut BehaviorData<'a> {
        &mut self._behavior_data
    }

    fn step_behavior_state(
        &mut self,
        _state: State,
        _behavior_state: BehaviorState,
        _is_first_update: bool,
        _owner: &mut Character<'a>,
        _target: Option<&Character<'a>>,
        _delta_time: f32,
    ) {
    }
}
