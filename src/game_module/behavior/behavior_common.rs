use crate::game_module::actors::character::{ActionAnimationState, Character, MoveAnimationState};
use crate::game_module::behavior::behavior_base::{BehaviorData, BehaviorState};
use crate::game_module::game_constants::{
    ARRIVAL_DISTANCE_THRESHOLD, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, GameViewMode, INTIMACY_ARRIVE_RANGE,
    INTIMACY_FOLLOW_RANGE, INTIMACY_ROAMING_RADIUS, NPC_ATTACK_CHASE_RANGE, NPC_ATTACK_HIT_RANGE,
    NPC_ATTACK_IDLE_RANGE, NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN, NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX,
    NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME, NPC_TRACKING_RANGE,
};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;

use crate::game_module::game_service_locator::get_game_scene_manager;
use crate::game_module::scenario::scenario::ScenarioType;

pub fn begin_idle(data: &mut BehaviorData, owner: &mut Character) {
    owner.set_move_idle();
    data.set_behavior_time(lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>()));
}

/// Pick a random roam target around the spawn point, or around the player if intimacy-following or for Ewa/Koa during WrapUpTheDay phase.
pub fn begin_roaming(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    let name = owner.get_character_name().as_str();
    let is_ewa_or_koa = name.contains("ewa") || name.contains("koa");

    let is_wrap_up_the_day_for_partner =
        is_ewa_or_koa && get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay);

    let is_intimate_following = owner.is_following_intimacy()
        && matches!(target, Some(t) if
            t.is_alive() && (t.get_position() - owner.get_position()).norm() <= INTIMACY_FOLLOW_RANGE
        );

    let roam_radius = if is_wrap_up_the_day_for_partner || is_intimate_following {
        INTIMACY_ROAMING_RADIUS
    } else {
        NPC_ROAMING_RADIUS
    };

    let move_area = Vector3::new(
        (rand::random::<f32>() - 0.5) * 2.0,
        0.0,
        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
            0.0
        } else {
            (rand::random::<f32>() - 0.5) * 2.0
        },
    ) * roam_radius;

    let player_pos = target
        .map(|t| *t.get_position())
        .or_else(|| get_game_scene_manager().get_maybe_player().as_ref().map(|p| *p.borrow().get_position()));

    let center_point = if is_wrap_up_the_day_for_partner {
        player_pos.unwrap_or(data._spawn_point)
    } else if is_intimate_following {
        *target.unwrap().get_position()
    } else {
        data._spawn_point
    };

    data._target_point = center_point + move_area;
    data._move_direction = math::safe_normalize(&(data._target_point - owner.get_position()));
    data.set_behavior_time(NPC_ROAMING_TIME);
    owner.set_move(&data._move_direction);
    owner.set_run(false);
}

/// Returns true when roaming should end: timeout, arrived, or blocked.
pub fn should_roaming_go_idle(data: &BehaviorData, owner: &Character) -> bool {
    if data.is_end_behavior_time() {
        return true;
    }
    let offset = data._target_point - owner.get_position();
    let dist_sq = offset.x * offset.x + offset.z * offset.z;
    if dist_sq < ARRIVAL_DISTANCE_THRESHOLD {
        return true;
    }
    (owner._controller._is_blocked || owner._controller._is_cliff) && !owner.is_falling()
}

/// Returns true when the owner is intimacy-following and the player has moved beyond INTIMACY_FOLLOW_RANGE.
pub fn is_player_too_far_for_intimacy(owner: &Character, target: Option<&Character>) -> bool {
    owner.is_following_intimacy()
        && matches!(target, Some(t) if
            t.is_alive() && t._is_player
            && (t.get_position() - owner.get_position()).norm() > INTIMACY_FOLLOW_RANGE
        )
}

pub enum IntimacyFollowResult {
    /// Within INTIMACY_ARRIVE_RANGE -> transition to Roaming.
    Arrived,
    Moving,
    /// Intimacy condition lost -> transition to Idle.
    NotFollowing,
}

pub fn update_intimacy_follow(owner: &mut Character, target: Option<&Character>) -> IntimacyFollowResult {
    if !owner.is_following_intimacy() {
        return IntimacyFollowResult::NotFollowing;
    }
    if let Some(target_ref) = target {
        if target_ref.is_alive() {
            let to_target = target_ref.get_position() - owner.get_position();
            let dist = (to_target.x * to_target.x + to_target.z * to_target.z).sqrt();
            if dist <= INTIMACY_ARRIVE_RANGE {
                return IntimacyFollowResult::Arrived;
            }
            owner.set_move(&to_target);
            owner.set_run(true);
            return IntimacyFollowResult::Moving;
        }
    }
    IntimacyFollowResult::NotFollowing
}

pub fn begin_eating(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    if !owner.is_move_stop() {
        owner.set_move_idle();
    }
    if let Some(target_actor) = target {
        owner.look_at(target_actor.get_position());
    }
    owner.set_is_interacting(false);
    owner.set_action_eating();
    data.set_behavior_time(NPC_IDLE_TERM_MIN);
}

pub fn update_eating_should_idle(is_first_update: bool, owner: &Character) -> bool {
    !is_first_update && !owner.is_action(ActionAnimationState::Eating)
}

pub fn begin_interaction(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    if !owner.is_move_stop() && !owner.is_move_state(MoveAnimationState::SitDownLoop) {
        owner.set_move_idle();
    }
    if let Some(target_actor) = target {
        owner.look_at(target_actor.get_position());
    }
    data.set_behavior_time(CHARACTER_INTERACTION_TIME);
}

pub fn update_interaction_should_idle(data: &BehaviorData, owner: &mut Character, target: Option<&Character>) -> bool {
    if data.is_end_behavior_time() {
        owner.set_is_interacting(false);
        return true;
    }
    if owner.is_interacting() {
        if !owner.is_move_state(MoveAnimationState::SitDownLoop) {
            owner.set_move_idle();
        }
        if let Some(target_actor) = target {
            owner.look_at(target_actor.get_position());
        }
        return false;
    }
    true
}

pub fn begin_wake_up(owner: &mut Character) {
    owner.set_action_wake_up();
}

pub fn update_wake_up_should_idle(is_first_update: bool, owner: &Character) -> bool {
    !is_first_update && !owner.is_action(ActionAnimationState::WakeUp)
}

pub fn update_chase(owner: &mut Character, target: Option<&Character>) -> BehaviorState {
    let mut do_idle = true;
    if let Some(target_ref) = target
        && target_ref.is_alive()
    {
        if owner.is_following_intimacy() && target_ref._is_player {
            match update_intimacy_follow(owner, target) {
                IntimacyFollowResult::Arrived => {
                    return BehaviorState::Roaming;
                }
                IntimacyFollowResult::Moving | IntimacyFollowResult::NotFollowing => {}
            }
            do_idle = false;
        } else if owner.is_tamed() && (target_ref._is_player || target_ref.is_civilian() || target_ref.is_tamed()) {
            return BehaviorState::Idle;
        } else if owner.check_in_range(target_ref.get_collision(), NPC_TRACKING_RANGE, false) {
            if owner.check_in_range(target_ref.get_collision(), NPC_ATTACK_HIT_RANGE, false) {
                return BehaviorState::Attack;
            } else {
                let to_target = target_ref.get_position() - owner.get_position();
                owner.set_move(&to_target);
                owner.set_run(true);
            }
            do_idle = false;
        }
    }
    if do_idle {
        BehaviorState::Idle
    } else {
        BehaviorState::Chase
    }
}

pub fn begin_attack(
    owner: &mut Character,
    target: Option<&Character>,
    attack_time: &mut f32,
    is_in_attack_range: &mut bool,
) -> BehaviorState {
    if let Some(target_ref) = target
        && owner.is_available_attack()
    {
        let to_dir = math::make_normalize_xz(&(target_ref.get_position() - owner.get_position()));
        owner.set_move_direction(&to_dir, false);
        owner.set_action_attack();
        get_audio_manager_mut().play_audio_resource_data(
            &owner._character_data.borrow()._audio_data._audio_growl,
            AudioLoop::ONCE,
            None,
        );

        *attack_time = lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>());
        *is_in_attack_range =
            target_ref.is_alive() && owner.check_in_range(target_ref.get_collision(), NPC_ATTACK_IDLE_RANGE, false);
        BehaviorState::Attack
    } else {
        BehaviorState::Chase
    }
}

pub fn update_attack(
    owner: &mut Character,
    target: Option<&Character>,
    attack_time: &mut f32,
    is_in_attack_range: &mut bool,
    delta_time: f32,
) -> BehaviorState {
    let mut next_state = BehaviorState::Attack;
    if target.is_none_or(|target| !target.is_alive()) {
        next_state = BehaviorState::Idle;
    } else {
        let target = target.unwrap();
        if *is_in_attack_range {
            if !owner.check_in_range(target.get_collision(), NPC_ATTACK_CHASE_RANGE, false) {
                *is_in_attack_range = false;
            }
        } else if owner.check_in_range(target.get_collision(), NPC_ATTACK_IDLE_RANGE, false) {
            *is_in_attack_range = true;
        }

        if NPC_AVAILABLE_MOVING_ATTACK && !*is_in_attack_range {
            let to_target = target.get_position() - owner.get_position();
            owner.set_move(&to_target);
            owner.set_run(true);
        } else {
            owner.set_move_idle();
        }

        if *attack_time <= 0.0 {
            if owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false) {
                next_state = BehaviorState::Chase;
            } else {
                next_state = BehaviorState::Idle;
            }
        }
    }

    if !owner.is_attack_animation() {
        *attack_time -= delta_time;
    }

    next_state
}
