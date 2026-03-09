use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use nalgebra::Vector3;

#[derive(Default)]
pub struct BehaviorUfo {
    pub _idle_time: f32,
    pub _behavior_state: BehaviorState,
}

impl BehaviorBase for BehaviorUfo {
    fn initialize_behavior(&mut self, _owner: &mut Character, _position: &Vector3<f32>) {
        self._behavior_state = BehaviorState::None;
    }

    fn update_behavior(
        &mut self,
        owner: &mut Character,
        player: Option<&Character>,
        _delta_time: f32,
    ) {
        match self._behavior_state {
            BehaviorState::None => {
                self.set_behavior(BehaviorState::Idle, owner, player, false);
            }
            _ => (),
        }
    }

    fn set_behavior(
        &mut self,
        behavior_state: BehaviorState,
        owner: &mut Character,
        player: Option<&Character>,
        is_force: bool,
    ) {
        if self._behavior_state != behavior_state || is_force {
            self.end_behavior(owner, player);
            self._behavior_state = behavior_state;
        }
    }

    fn end_behavior(&mut self, _owner: &mut Character, _player: Option<&Character>) {
        match self._behavior_state {
            _ => (),
        }
    }
}
