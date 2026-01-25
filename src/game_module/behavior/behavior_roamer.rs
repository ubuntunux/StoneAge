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

impl BehaviorBase for BehaviorRoamer {
    fn initialize_behavior(&mut self, _owner: &mut Character, position: &Vector3<f32>) {
        self._spawn_point = position.clone();
        self._behavior_state = BehaviorState::None;
    }

    fn is_enemy_in_range(&self, owner: &Character, player: Option<&Character>) -> bool {
        if let Some(player) = player {
            if player.is_alive() {
                return owner.check_in_range(player.get_collision(), NPC_TRACKING_RANGE, false);
            }
        }
        false
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
            match behavior_state {
                BehaviorState::None => {}
                BehaviorState::Idle => {
                    owner.set_move_stop();
                    self._idle_time =
                        lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>());
                }
                BehaviorState::Move => {
                    let move_area = Vector3::new(
                        rand::random::<f32>() - 0.5,
                        0.0,
                        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D { 0.0 } else { rand::random::<f32>() - 0.5 },
                    )
                    .normalize()
                        * NPC_ROAMING_RADIUS;
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
                    let to_player_direction = (player.as_ref().unwrap().get_position() - owner.get_position()).normalize();
                    owner.set_move_direction(&to_player_direction);
                    if !NPC_AVAILABLE_MOVING_ATTACK {
                        owner.set_move_stop();
                    }
                    owner.set_action_attack();
                    self._attack_time = lerp(
                        NPC_ATTACK_TERM_MIN,
                        NPC_ATTACK_TERM_MAX,
                        rand::random::<f32>(),
                    );

                    // growl
                    owner
                        .get_character_manager()
                        .get_scene_manager()
                        .play_audio(&owner._character_data.borrow()._audio_data._audio_growl);
                }
                _ => (),
            }
        }
    }

    fn end_behavior(&mut self, _owner: &mut Character, _player: Option<&Character>) {
        match self._behavior_state {
            _ => (),
        }
    }

    fn update_behavior(
        &mut self,
        owner: &mut Character,
        player: Option<&Character>,
        delta_time: f32,
    ) {
        match self._behavior_state {
            BehaviorState::None => {
                self.set_behavior(BehaviorState::Idle, owner, player, false);
            }
            BehaviorState::Idle => {
                if self.is_enemy_in_range(owner, player) {
                    self.set_behavior(BehaviorState::Chase, owner, player, false);
                } else if self._idle_time < 0.0 {
                    self.set_behavior(BehaviorState::Move, owner, player, false);
                }
                self._idle_time -= delta_time;
            }
            BehaviorState::Move => {
                if self.is_enemy_in_range(owner, player) {
                    self.set_behavior(BehaviorState::Chase, owner, player, false);
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
                        self.set_behavior(BehaviorState::Idle, owner, player, false);
                    }
                }
                self._move_time -= delta_time;
            }
            BehaviorState::Chase => {
                let mut do_idle: bool = true;
                if let Some(player_ref) = player {
                    if player_ref.is_alive() {
                        if owner.check_in_range(player_ref.get_collision(), NPC_TRACKING_RANGE, false) {
                            if owner.check_in_range(player_ref.get_collision(), NPC_ATTACK_RANGE, false) {
                                self.set_behavior(BehaviorState::Attack, owner, player, false);
                            } else {
                                let to_player: Vector3<f32> = player_ref.get_position() - owner.get_position();
                                owner.set_move(&to_player);
                                owner.set_run(true);
                            }
                            do_idle = false;
                        }
                    }
                }

                if do_idle {
                    self.set_behavior(BehaviorState::Idle, owner, player, false);
                }
            }
            BehaviorState::Attack => {
                let mut do_idle: bool = true;
                if let Some(player_ref) = player {
                    if player_ref.is_alive() && 0.0 < self._attack_time {
                        if owner.is_attack_animation() {
                            if !owner.is_available_move() || (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation()) {
                                owner.set_move_stop();
                            }
                        } else {
                            owner.set_move_stop();
                            self._attack_time -= delta_time;
                        }
                        do_idle = false;
                    }
                }

                if do_idle {
                    self.set_behavior(BehaviorState::Idle, owner, player, false);
                }
            }
            _ => (),
        }
    }
}
