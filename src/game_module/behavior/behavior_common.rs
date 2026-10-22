use crate::game_module::actors::character::{ActionAnimationState, Character, CharacterDataType, MoveAnimationState};
use crate::game_module::actors::props::api::PropDataType;
use crate::game_module::behavior::behavior_base::{BehaviorData, BehaviorState};
use crate::game_module::game_constants::{
    ARRIVAL_DISTANCE_THRESHOLD, AUDIO_QUEST_COMPLETE, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, GameViewMode,
    INTIMACY_ARRIVE_RANGE, INTIMACY_FOLLOW_RANGE, INTIMACY_ROAMING_RADIUS, NPC_ATTACK_CHASE_RANGE,
    NPC_ATTACK_HIT_RANGE, NPC_ATTACK_IDLE_RANGE, NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN,
    NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME,
    NPC_TRACKING_RANGE, PROP_NAME_TABLE,
};
use crate::game_module::game_service_locator::{
    get_game_scene_manager, get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut,
};
use crate::game_module::widgets::cooking_widget::COOKING_RECIPES;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;
use rust_engine_3d::utilities::system::State;

pub fn update_hunger_action(owner: &mut Character) {
    if owner.get_stats().is_hungry() {
        if owner.is_move_state(MoveAnimationState::SitDownLoop) {
            owner.set_move_idle();
        }
        if owner.is_idle_action() && !owner.is_action(ActionAnimationState::Hungry) {
            owner.set_action_hungry();
        }
    } else if owner.is_action(ActionAnimationState::Hungry) {
        owner.set_action_none();
    }
}

pub fn begin_idle(data: &mut BehaviorData, owner: &mut Character) {
    owner.set_move_idle();
    update_hunger_action(owner);
    data.set_behavior_time(lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>()));
}

pub fn update_idle_interacting(owner: &mut Character, target: Option<&Character>) {
    if !owner.is_move_state(MoveAnimationState::SitDownLoop) {
        owner.set_move_idle();
    }
    if let Some(target_actor) = target {
        owner.look_at(target_actor.get_position());
    }
}

/// Checks if `target` is a valid hostile enemy within `NPC_TRACKING_RANGE` for `owner`.
/// Civilian, tamed, or intimacy-following characters only treat wild hostile characters as enemies.
pub fn is_enemy_in_range(owner: &Character, target: Option<&Character>) -> bool {
    let Some(target) = target.filter(|t| t.is_alive()) else {
        return false;
    };
    if (owner.is_civilian() || owner.is_tamed() || owner.is_following_intimacy()) && !target.is_wild_hostile() {
        return false;
    }
    owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false)
}

/// Returns the current alive player's position.
fn get_alive_player_position() -> Option<Vector3<f32>> {
    get_game_scene_manager().get_maybe_player().as_ref().and_then(|player| {
        let player_ref = player.borrow();
        if player_ref.is_alive() {
            Some(*player_ref.get_position())
        } else {
            None
        }
    })
}

/// Pick a random roam target around the spawn point, or around the player if intimacy-following or family npc.
pub fn begin_roaming(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    update_hunger_action(owner);

    // Family npcs have high intimacy with the player, so they always roam around the player.
    let family_player_pos = if owner.is_family_npc() {
        get_alive_player_position()
    } else {
        None
    };

    let intimate_target_pos = target
        .filter(|t| {
            owner.is_following_intimacy()
                && t.is_alive()
                && (t.get_position() - owner.get_position()).norm() <= INTIMACY_FOLLOW_RANGE
        })
        .map(|t| *t.get_position());

    let roam_radius = if family_player_pos.is_some() || intimate_target_pos.is_some() {
        INTIMACY_ROAMING_RADIUS
    } else {
        NPC_ROAMING_RADIUS
    };

    let mut move_area = Vector3::new(
        (rand::random::<f32>() - 0.5) * 2.0,
        0.0,
        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
            0.0
        } else {
            (rand::random::<f32>() - 0.5) * 2.0
        },
    ) * roam_radius;

    // If the previous roaming failed (blocked or timeout), roam around the current position
    // and avoid the failed direction so the npc doesn't keep walking into the same obstacle.
    let center_point = if let Some(failed_direction) = data._roaming_failed_direction.take() {
        if move_area.dot(&failed_direction) > 0.0 {
            move_area = -move_area;
        }
        *owner.get_position()
    } else if let Some(player_pos) = family_player_pos {
        player_pos
    } else if let Some(target_pos) = intimate_target_pos {
        target_pos
    } else {
        data._spawn_point
    };

    data._target_point = center_point + move_area;
    data._move_direction = math::safe_normalize(&(data._target_point - owner.get_position()));
    data.set_behavior_time(NPC_ROAMING_TIME);
    owner.set_move(&data._move_direction);
    owner.set_run(false);
}

/// Returns true when roaming should end: arrived, timeout, or blocked.
/// On timeout or blocked, remembers the move direction so the next roaming avoids it.
pub fn should_roaming_go_idle(data: &mut BehaviorData, owner: &Character) -> bool {
    let offset = data._target_point - owner.get_position();
    let dist_sq = offset.x * offset.x + offset.z * offset.z;
    if dist_sq < ARRIVAL_DISTANCE_THRESHOLD {
        data._roaming_failed_direction = None;
        return true;
    }
    let is_blocked = (owner._controller._is_blocked || owner._controller._is_cliff) && !owner.is_falling();
    if data.is_end_behavior_time() || is_blocked {
        data._roaming_failed_direction = Some(data._move_direction);
        return true;
    }
    false
}

/// Returns the position the owner should follow: the player for family npcs,
/// or the intimacy target (player) for intimacy-following characters.
fn get_follow_target_position(owner: &Character, target: Option<&Character>) -> Option<Vector3<f32>> {
    if owner.is_family_npc() {
        if !owner.is_alive() {
            return None;
        }
        return get_alive_player_position();
    }
    if owner.is_following_intimacy() {
        return target.filter(|t| t.is_alive() && t._is_player).map(|t| *t.get_position());
    }
    None
}

/// Returns true when the owner is intimacy-following (or a family npc) and the player has moved beyond INTIMACY_FOLLOW_RANGE.
pub fn is_player_too_far_for_intimacy(owner: &Character, target: Option<&Character>) -> bool {
    get_follow_target_position(owner, target)
        .is_some_and(|target_pos| (target_pos - owner.get_position()).norm() > INTIMACY_FOLLOW_RANGE)
}

pub enum IntimacyFollowResult {
    /// Within INTIMACY_ARRIVE_RANGE -> transition to Roaming.
    Arrived,
    Moving,
    /// Intimacy condition lost -> transition to Idle.
    NotFollowing,
}

pub fn update_intimacy_follow(owner: &mut Character, target: Option<&Character>) -> IntimacyFollowResult {
    let Some(target_pos) = get_follow_target_position(owner, target) else {
        return IntimacyFollowResult::NotFollowing;
    };
    let to_target = target_pos - owner.get_position();
    let dist = (to_target.x * to_target.x + to_target.z * to_target.z).sqrt();
    if dist <= INTIMACY_ARRIVE_RANGE {
        return IntimacyFollowResult::Arrived;
    }
    owner.set_move(&to_target);
    owner.set_run(true);
    IntimacyFollowResult::Moving
}

pub fn begin_eating(data: &mut BehaviorData, owner: &mut Character, target: Option<&Character>) {
    if !owner.is_move_stop() {
        owner.set_move_idle();
    }
    if owner.get_attached_item().is_some()
        && let Some(target_actor) = target
    {
        owner.look_at(target_actor.get_position());
    }
    owner.set_is_interacting(false);
    owner.set_action_eating();
    data.set_behavior_time(NPC_IDLE_TERM_MIN);
}

pub fn update_eating_should_idle(is_first_update: bool, owner: &Character) -> bool {
    !is_first_update
        && !owner.is_action(ActionAnimationState::Eating)
        && owner._animation_state._next_action_animation_state != Some(ActionAnimationState::Eating)
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
        update_idle_interacting(owner, target);
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
        } else if (owner.is_civilian() || owner.is_tamed()) && !target_ref.is_wild_hostile() {
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
    if let Some(target) = target.filter(|target| target.is_alive()) {
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
    } else {
        next_state = BehaviorState::Idle;
    }

    if owner.is_available_attack() {
        *attack_time -= delta_time;
    }

    next_state
}

pub fn step_hunger(data: &mut BehaviorData, _state: State, owner: &mut Character) {
    update_hunger_action(owner);
    data.set_next_behavior_state(BehaviorState::Idle, true);
}

pub fn step_eating(
    data: &mut BehaviorData,
    state: State,
    is_first_update: bool,
    owner: &mut Character,
    target: Option<&Character>,
) {
    match state {
        State::Begin => begin_eating(data, owner, target),
        State::Update => {
            if update_eating_should_idle(is_first_update, owner) {
                update_hunger_action(owner);
                data.set_next_behavior_state(BehaviorState::Idle, false);
            }
        }
        State::End => {}
    }
}

pub fn step_interaction(data: &mut BehaviorData, state: State, owner: &mut Character, target: Option<&Character>) {
    match state {
        State::Begin => begin_interaction(data, owner, target),
        State::Update => {
            if owner.get_attached_item_data_type().is_eatable() {
                data.set_next_behavior_state(BehaviorState::Eating, true);
            } else if update_interaction_should_idle(data, owner, target) {
                update_hunger_action(owner);
                data.set_next_behavior_state(BehaviorState::Idle, false);
            }
        }
        State::End => {}
    }
}

pub fn step_wake_up(
    data: &mut BehaviorData,
    state: State,
    is_first_update: bool,
    owner: &mut Character,
    revive_on_begin: bool,
) {
    match state {
        State::Begin => {
            begin_wake_up(owner);
            if revive_on_begin {
                owner._character_stats._is_alive = true;
                let max_hp = owner._character_stats.get_max_hp();
                owner._character_stats.set_hp(max_hp);
            }
        }
        State::Update => {
            if update_wake_up_should_idle(is_first_update, owner) {
                data.set_next_behavior_state(BehaviorState::Idle, false);
            }
        }
        State::End => {}
    }
}

pub fn step_follow(data: &mut BehaviorData, state: State, owner: &mut Character, target: Option<&Character>) {
    if state == State::Update {
        match update_intimacy_follow(owner, target) {
            IntimacyFollowResult::Arrived => data.set_next_behavior_state(BehaviorState::Roaming, false),
            IntimacyFollowResult::Moving => {}
            IntimacyFollowResult::NotFollowing => data.set_next_behavior_state(BehaviorState::Idle, false),
        }
    }
}

pub fn step_dance(data: &mut BehaviorData, state: State, owner: &mut Character) {
    match state {
        State::Begin => owner.set_action_dance(),
        State::Update => {
            if !owner.is_action(ActionAnimationState::Dance) {
                data.set_next_behavior_state(BehaviorState::Idle, false);
            }
        }
        State::End => {
            if owner.is_action(ActionAnimationState::Dance) {
                owner.set_action_none();
            }
            owner.set_is_interacting(false);
        }
    }
}

pub fn step_chase(data: &mut BehaviorData, state: State, owner: &mut Character, target: Option<&Character>) {
    if state == State::Update {
        let next_state = update_chase(owner, target);
        if next_state != BehaviorState::Chase {
            data.set_next_behavior_state(next_state, false);
        }
    }
}

pub fn step_attack(
    data: &mut BehaviorData,
    state: State,
    owner: &mut Character,
    target: Option<&Character>,
    delta_time: f32,
) {
    match state {
        State::Begin => {
            let next_state = begin_attack(owner, target, &mut data._attack_time, &mut data._is_in_attack_range);
            if next_state != BehaviorState::Attack {
                data.set_next_behavior_state(next_state, false);
            }
        }
        State::Update => {
            let next_state = update_attack(
                owner,
                target,
                &mut data._attack_time,
                &mut data._is_in_attack_range,
                delta_time,
            );
            if next_state != BehaviorState::Attack {
                data.set_next_behavior_state(next_state, false);
            }
        }
        State::End => {}
    }
}

pub fn step_dead_with_revive(data: &mut BehaviorData, state: State, owner: &mut Character, revive_time: f32) {
    match state {
        State::Begin => {
            owner.set_action_dead();
            data.set_behavior_time(revive_time);
        }
        State::Update => {
            if data.is_end_behavior_time() {
                data.set_next_behavior_state(BehaviorState::WakeUp, true);
            }
        }
        State::End => {}
    }
}

pub const NPC_COOKING_DURATION: f32 = 4.0;
pub const NPC_COOKING_TABLE_APPROACH_DISTANCE: f32 = 1.3;

pub fn step_cooking(
    data: &mut BehaviorData,
    state: State,
    owner: &mut Character,
    delta_time: f32,
) {
    match state {
        State::Begin => {
            owner._cooking_progress = 0.0;
        }
        State::Update => {
            let Some(recipe_idx) = owner._cooking_recipe_index else {
                data.set_next_behavior_state(BehaviorState::Idle, false);
                return;
            };

            let table = get_game_scene_manager()
                .get_prop_manager()
                .get_prop_by_name(PROP_NAME_TABLE)
                .cloned()
                .or_else(|| {
                    get_game_scene_manager()
                        .get_prop_manager()
                        .get_props()
                        .values()
                        .find(|p| p.borrow()._prop_data.borrow()._prop_type == PropDataType::Table)
                        .cloned()
                });

            let Some(table) = table else {
                owner._cooking_recipe_index = None;
                data.set_next_behavior_state(BehaviorState::Idle, false);
                return;
            };

            let table_center = table.borrow().get_bounding_box()._center;
            let current_pos = *owner.get_position();
            let dist_to_table = (Vector2::new(current_pos.x, current_pos.z) - Vector2::new(table_center.x, table_center.z)).norm();

            if dist_to_table > NPC_COOKING_TABLE_APPROACH_DISTANCE {
                owner.move_to_target(&table_center, NPC_COOKING_TABLE_APPROACH_DISTANCE, delta_time);
            } else {
                owner.set_move_idle();
                owner.look_at(&table_center);

                if !owner.is_action(ActionAnimationState::Pickup) {
                    owner.set_action_interaction();
                }

                let ui_mgr = get_game_ui_manager_mut();
                if !ui_mgr.is_npc_cooking_active() {
                    ui_mgr.start_npc_cooking(recipe_idx);
                }

                owner._cooking_progress += delta_time / NPC_COOKING_DURATION;
                ui_mgr.set_npc_cooking_progress(owner._cooking_progress);

                if owner._cooking_progress >= 1.0 {
                    owner._cooking_progress = 0.0;
                    owner.set_action_none();

                    if recipe_idx < COOKING_RECIPES.len() {
                        let recipe = &COOKING_RECIPES[recipe_idx];
                        let item_name = recipe.item_code();

                        get_item_manager_mut().place_item_name_on_table(Some(table), item_name);
                        get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                        ui_mgr.complete_npc_cooking(item_name);
                    } else {
                        ui_mgr.close_cooking_minigame();
                    }

                    owner._cooking_recipe_index = None;
                    data.set_next_behavior_state(BehaviorState::Idle, false);
                }
            }
        }
        State::End => {
            if get_game_ui_manager().is_npc_cooking_active() {
                get_game_ui_manager_mut().close_cooking_minigame();
            }
            if owner.is_action(ActionAnimationState::Pickup) {
                owner.set_action_none();
            }
            owner.set_move_idle();
            owner.set_is_interacting(false);
        }
    }
}

pub const TABLE_SEAT_EATING_DIST: f32 = 1.0;
pub const TABLE_SEAT_ARRIVAL_DIST: f32 = 0.35;
pub const TABLE_EATING_SEARCH_RADIUS: f32 = 30.0;

pub fn try_table_eating(owner: &mut Character, _delta_time: f32) -> bool {
    if !owner.is_family_npc() && !owner.is_civilian() {
        return false;
    }

    if owner.is_interacting()
        || owner.is_action(ActionAnimationState::Dance)
        || owner.is_action(ActionAnimationState::Dead)
        || owner.is_action(ActionAnimationState::PassedOut)
        || owner.is_action(ActionAnimationState::Eating)
    {
        return false;
    }

    let table = get_game_scene_manager()
        .get_prop_manager()
        .get_prop_by_name(PROP_NAME_TABLE)
        .cloned()
        .or_else(|| {
            get_game_scene_manager()
                .get_prop_manager()
                .get_props()
                .values()
                .find(|p| p.borrow()._prop_data.borrow()._prop_type == PropDataType::Table)
                .cloned()
        });

    let Some(table) = table else {
        return false;
    };

    let has_food = get_game_scene_manager().get_item_manager().has_item_on_table(&table);
    if !has_food {
        return false;
    }

    let table_pos = *table.borrow().get_position();
    let current_pos = *owner.get_position();
    let dist_to_table = (Vector2::new(current_pos.x, current_pos.z) - Vector2::new(table_pos.x, table_pos.z)).norm();

    if dist_to_table > TABLE_EATING_SEARCH_RADIUS {
        return false;
    }

    let seat_direction = match owner.get_character_type() {
        CharacterDataType::Wife => Vector3::new(0.0, 0.0, 1.0),
        CharacterDataType::Daughter => Vector3::new(-1.0, 0.0, 0.0),
        _ => Vector3::new(1.0, 0.0, 0.0),
    };

    let scene_manager = get_scene_manager();
    let mut seat_pos = table_pos - math::safe_normalize(&seat_direction) * TABLE_SEAT_EATING_DIST;
    seat_pos.y = scene_manager.get_height_bilinear(&seat_pos, 0);

    let (direction, dist) = math::make_normalize_xz_with_norm(&(seat_pos - current_pos));

    if dist > TABLE_SEAT_ARRIVAL_DIST {
        owner.set_move(&direction);
        true
    } else {
        if !owner.is_move_stop() {
            owner.set_move_idle();
        }
        owner.look_at(&table_pos);

        let is_ready_to_eat = owner.get_attached_item().is_none()
            && !owner.is_action(ActionAnimationState::Eating)
            && owner._animation_state._next_action_animation_state != Some(ActionAnimationState::Eating)
            && owner._behavior.get_behavior_data().get_next_behavior_state() != BehaviorState::Eating;

        if is_ready_to_eat {
            let item_manager = get_game_scene_manager().get_item_manager_mut();
            if let Some(item_data_name) = item_manager.consume_item_on_table(&table) {
                owner.apply_food_item_stats(item_data_name.as_str());
                owner.set_next_behavior(BehaviorState::Eating, true);
            }
        }
        true
    }
}
