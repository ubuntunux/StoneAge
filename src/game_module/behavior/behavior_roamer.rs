use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use crate::game_module::game_constants::{GameViewMode, ARRIVAL_DISTANCE_THRESHOLD, GAME_VIEW_MODE, NPC_ATTACK_RANGE, NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN, NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME, NPC_TRACKING_RANGE};
use nalgebra::Vector3;
use rust_engine_3d::utilities::math::lerp;

#[derive(Default)]
pub struct BehaviorRoamer {
    pub _idle_time: f32,
    pub _move_direction: Vector3<f32>,
    pub _spawn_point: Vector3<f32>,
    pub _target_point: Vector3<f32>,
    pub _move_time: f32,
    pub _attack_time: f32,
    pub _behavior_state: BehaviorState,
}

impl BehaviorRoamer {
    fn is_enemy_in_range(&self, owner: &Character, target: Option<&Character>) -> bool {
        if let Some(target) = target {
            if target.is_alive() {
                return owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false);
            }
        }
        false
    }
}

impl BehaviorBase for BehaviorRoamer {
    fn initialize_behavior(&mut self, _owner: &mut Character, position: &Vector3<f32>) {
        self._spawn_point = position.clone();
        self._behavior_state = BehaviorState::Idle;
    }

    fn update_behavior(
        &mut self,
        owner: &mut Character,
        target: Option<&Character>,
        delta_time: f32,
    ) {
        match self._behavior_state {
            BehaviorState::Idle => {
                if self.is_enemy_in_range(owner, target) {
                    self.set_behavior(BehaviorState::Chase, owner, target, false);
                } else if self._idle_time < 0.0 {
                    self.set_behavior(BehaviorState::Roaming, owner, target, false);
                }
                self._idle_time -= delta_time;
            }
            BehaviorState::Roaming => {
                if self.is_enemy_in_range(owner, target) {
                    self.set_behavior(BehaviorState::Chase, owner, target, false);
                } else {
                    let mut do_idle: bool = false;
                    if 0.0 < self._move_time {
                        let offset = self._target_point - owner.get_position();
                        let dist = offset.x * offset.x + offset.z * offset.z;
                        if dist < ARRIVAL_DISTANCE_THRESHOLD {
                            do_idle = true;
                        } else if (owner._controller._is_blocked || owner._controller._is_cliff) && !owner.is_falling() {
                            do_idle = true;
                        }
                    } else {
                        do_idle = true;
                    }

                    if do_idle {
                        self.set_behavior(BehaviorState::Idle, owner, target, false);
                    }
                }
                self._move_time -= delta_time;
            }
            BehaviorState::Chase => {
                let mut do_idle: bool = true;
                if let Some(target_ref) = target {
                    if target_ref.is_alive() {
                        if owner.check_in_range(target_ref.get_collision(), NPC_TRACKING_RANGE, false) {
                            if owner.check_in_range(target_ref.get_collision(), NPC_ATTACK_RANGE, false) {
                                self.set_behavior(BehaviorState::Attack, owner, target, false);
                            } else {
                                let to_target: Vector3<f32> = target_ref.get_position() - owner.get_position();
                                owner.set_move(&to_target);
                                owner.set_run(true);
                            }
                            do_idle = false;
                        }
                    }
                }

                if do_idle {
                    self.set_behavior(BehaviorState::Idle, owner, target, false);
                }
            }
            BehaviorState::Attack => {
                let mut do_idle: bool = true;
                if let Some(target_ref) = target {
                    if target_ref.is_alive() && 0.0 < self._attack_time {
                        if owner.is_attack_animation() {
                            if !owner.is_available_move() || (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation()) {
                                owner.set_move_idle();
                            }
                        } else {
                            owner.set_move_idle();
                            self._attack_time -= delta_time;
                        }
                        do_idle = false;
                    }
                }

                if do_idle {
                    self.set_behavior(BehaviorState::Idle, owner, target, false);
                }
            }
            _ => (),
        }
    }

    fn set_behavior(
        &mut self,
        behavior_state: BehaviorState,
        owner: &mut Character,
        target: Option<&Character>,
        is_force: bool,
    ) {
        if self._behavior_state != behavior_state || is_force {
            self.end_behavior(owner, target);

            self._behavior_state = behavior_state;
            match behavior_state {
                BehaviorState::Idle => {
                    owner.set_move_idle();
                    self._idle_time =
                        lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>());
                }
                BehaviorState::Roaming => {
                    let move_area = Vector3::new(
                        rand::random::<f32>() - 0.5,
                        0.0,
                        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D { 0.0 } else { rand::random::<f32>() - 0.5 },
                    ).normalize() * NPC_ROAMING_RADIUS;
                    self._target_point = self._spawn_point + move_area;
                    self._move_direction = (self._target_point - owner.get_position()).normalize();
                    self._move_time = NPC_ROAMING_TIME;
                    owner.set_move(&self._move_direction);
                    owner.set_run(false);
                }
                BehaviorState::Chase => {
                    // growl
                    //owner.get_character_manager().get_scene_manager().play_audio(&owner._audio_growl);
                }
                BehaviorState::Attack => {
                    let to_target_direction = (target.as_ref().unwrap().get_position() - owner.get_position()).normalize();
                    owner.set_move_direction(&to_target_direction, false);
                    if !NPC_AVAILABLE_MOVING_ATTACK {
                        owner.set_move_idle();
                    }
                    owner.set_action_attack();
                    self._attack_time = lerp(
                        NPC_ATTACK_TERM_MIN,
                        NPC_ATTACK_TERM_MAX,
                        rand::random::<f32>(),
                    );

                    // growl
                    owner.get_character_manager().get_scene_manager().play_audio(&owner._character_data.borrow()._audio_data._audio_growl);
                }
                _ => (),
            }
        }
    }

    fn end_behavior(&mut self, _owner: &mut Character, _target: Option<&Character>) {
        match self._behavior_state {
            _ => (),
        }
    }
}
