use crate::game_module::actors::character::{ActionAnimationState, Character, MoveAnimationState};
use crate::game_module::behavior::behavior_base::BehaviorData;
use crate::game_module::game_constants::{
    ARRIVAL_DISTANCE_THRESHOLD, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, GameViewMode, INTIMACY_ARRIVE_RANGE,
    INTIMACY_FOLLOW_RANGE, INTIMACY_ROAMING_RADIUS, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS,
    NPC_ROAMING_TIME,
};
use nalgebra::Vector3;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;

pub fn begin_idle(data: &mut BehaviorData, owner: &mut Character) {
    owner.set_move_idle();
    data.set_behavior_time(lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>()));
}

/// Pick a random roam target around the spawn point, or around the player if intimacy-following.
pub fn begin_roaming(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    let is_intimate_following = owner.is_following_intimacy()
        && matches!(target, Some(t) if
            t.is_alive() && (t.get_position() - owner.get_position()).norm() <= INTIMACY_FOLLOW_RANGE
        );

    let roam_radius = if is_intimate_following {
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

    let center_point = if is_intimate_following {
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

pub fn begin_eating(data: &mut BehaviorData, owner: &mut Character) {
    if !owner.is_move_stop() {
        owner.set_move_idle();
    }
    owner.set_is_interacting(false);
    owner.set_action_eating();
    data.set_behavior_time(NPC_IDLE_TERM_MIN);
}

pub fn update_eating_should_idle(is_first_update: bool, owner: &Character) -> bool {
    !is_first_update && !owner.is_action(ActionAnimationState::Eating)
}

pub fn begin_interaction(data: &mut BehaviorData, owner: &mut Character) {
    if !owner.is_move_stop() && !owner.is_move_state(MoveAnimationState::SitDownLoop) {
        owner.set_move_idle();
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
