use crate::game_module::actors::character::Character;
use crate::game_module::actors::character::data::ActionAnimationState;
use rust_engine_3d::core::engine_service_locator::get_scene_manager;

// Fishing Cast & Gauge Constants
pub const FISHING_CAST_DISTANCE_MIN: f32 = 2.0;
pub const FISHING_CAST_DISTANCE_RANGE: f32 = 6.0;
pub const FISHING_GAUGE_SPEED: f32 = 2.0;

// Fishing Minigame Tuning Parameters
pub const FISHING_MINIGAME_INITIAL_GAUGE: f32 = 0.5;
pub const FISHING_MINIGAME_RANDOM_ANGLE_INITIAL: f32 = 60.0;
pub const FISHING_MINIGAME_RANDOM_ANGLE_TARGET: f32 = 80.0;
pub const FISHING_MINIGAME_CHANGE_TIMER_MIN: f32 = 2.0;
pub const FISHING_MINIGAME_CHANGE_TIMER_RANGE: f32 = 2.0;
pub const FISHING_PLAYER_ROTATE_SPEED: f32 = 120.0;
pub const FISHING_PLAYER_RETURN_SPEED: f32 = 140.0;
pub const FISHING_PLAYER_RETURN_MIN_FACTOR: f32 = 0.20;
pub const FISHING_FISH_BASE_TURN_SPEED: f32 = 75.0;
pub const FISHING_FISH_TURN_MIN_FACTOR: f32 = 0.25;
pub const FISHING_FISH_TURN_SPEED: f32 = FISHING_FISH_BASE_TURN_SPEED;
pub const FISHING_ALIGNMENT_MATCH_DOT: f32 = 0.85;
pub const FISHING_PULL_DECREASE_MAX: f32 = 0.30;
pub const FISHING_PULL_FAIL_INCREASE_MAX: f32 = 0.38;
pub const FISHING_IDLE_INCREASE_SPEED: f32 = 0.18;
pub const FISHING_PRESS_BONUS_MAX: f32 = 0.05;
pub const FISHING_PRESS_PENALTY_MAX: f32 = 0.07;

// Difficulty Level Escaping Angle Ranges
pub const FISHING_DIFFICULTY_EASY_ANGLE_RANGE: f32 = 45.0;
pub const FISHING_DIFFICULTY_NORMAL_ANGLE_RANGE: f32 = 70.0;
pub const FISHING_DIFFICULTY_HARD_ANGLE_RANGE: f32 = 90.0;

impl<'a> Character<'a> {
    pub fn set_fishing_difficulty_angle_range(&mut self, range_degrees: f32) {
        self._fishing_state._difficulty_angle_range = range_degrees.clamp(15.0, 90.0);
    }

    pub fn set_action_fishing_begin(&mut self) {
        if self.is_available_attack() {
            self._fishing_state._fishing_gauge = 0.0;
            self._fishing_state._fishing_gauge_dir = 1.0;
            self._fishing_state._is_fishing_button_held = true;
            self._fishing_state._fishing_cast_distance = FISHING_CAST_DISTANCE_MIN;
            self.set_next_action_animation(ActionAnimationState::FishingBegin, 1.0);
            self.set_move_idle();
        }
    }

    pub fn get_fishing_gauge(&self) -> f32 {
        self._fishing_state._fishing_gauge
    }

    pub fn is_fishing_gauge_active(&self) -> bool {
        self.is_action(ActionAnimationState::FishingBegin) || self.is_action(ActionAnimationState::FishingLoop)
    }

    pub fn release_fishing_cast(&mut self) {
        self._fishing_state._is_fishing_button_held = false;
        self._fishing_state._fishing_cast_distance = FISHING_CAST_DISTANCE_MIN + self._fishing_state._fishing_gauge * FISHING_CAST_DISTANCE_RANGE;
    }

    pub fn is_fishing_spot(&self) -> bool {
        let check_pos = self.get_position() + self.get_face_direction() * self._fishing_state._fishing_cast_distance;
        let height_map_data = get_scene_manager().get_height_map_data();
        let height = height_map_data.get_height_bilinear(&check_pos, 0);
        let sea_height = get_scene_manager().get_sea_height();
        height < sea_height
    }

    pub fn update_fishing(&mut self, delta_time: f32) {
        self._fishing_state._fishing_gauge += self._fishing_state._fishing_gauge_dir * FISHING_GAUGE_SPEED * delta_time;
        if self._fishing_state._fishing_gauge >= 1.0 {
            self._fishing_state._fishing_gauge = 1.0;
            self._fishing_state._fishing_gauge_dir = -1.0;
        } else if self._fishing_state._fishing_gauge <= 0.0 {
            self._fishing_state._fishing_gauge = 0.0;
            self._fishing_state._fishing_gauge_dir = 1.0;
        }
    }

    pub fn start_fishing_minigame(&mut self) {
        self._fishing_state._is_minigame_active = true;
        self._fishing_state._fish_gauge = FISHING_MINIGAME_INITIAL_GAUGE;
        self._fishing_state._player_angle = 0.0;
        let angle_range = self._fishing_state._difficulty_angle_range;
        let random_angle = (rand::random::<f32>() * 2.0 - 1.0) * angle_range.min(60.0);
        self._fishing_state._fish_angle = random_angle;
        self._fishing_state._fish_target_angle = (rand::random::<f32>() * 2.0 - 1.0) * angle_range;
        self._fishing_state._fish_change_timer = FISHING_MINIGAME_CHANGE_TIMER_MIN;
        self._fishing_state._direction_dot = 1.0;
        self._fishing_state._is_pulling = false;
        self._fishing_state._is_direction_matched = true;
        self._fishing_state._minigame_success = None;
    }

    pub fn rotate_player_angle(&mut self, dir: f32, delta_time: f32) {
        if dir < 0.0 {
            let new_angle = self._fishing_state._player_angle + dir * FISHING_PLAYER_ROTATE_SPEED * delta_time;
            self._fishing_state._player_angle = new_angle.clamp(-90.0, 90.0);
        } else if dir > 0.0 {
            let new_angle = self._fishing_state._player_angle + dir * FISHING_PLAYER_ROTATE_SPEED * delta_time;
            self._fishing_state._player_angle = new_angle.clamp(-90.0, 90.0);
        } else {
            let current_angle = self._fishing_state._player_angle;
            if current_angle.abs() <= 0.1 {
                self._fishing_state._player_angle = 0.0;
            } else {
                let distance_ratio = (current_angle.abs() / 90.0).clamp(0.0, 1.0);
                let speed_factor = FISHING_PLAYER_RETURN_MIN_FACTOR + (1.0 - FISHING_PLAYER_RETURN_MIN_FACTOR) * distance_ratio;
                let return_speed = FISHING_PLAYER_RETURN_SPEED * speed_factor;
                let step = return_speed * delta_time;

                if current_angle > 0.0 {
                    self._fishing_state._player_angle = (current_angle - step).max(0.0);
                } else {
                    self._fishing_state._player_angle = (current_angle + step).min(0.0);
                }
            }
        }
    }

    pub fn set_pulling(&mut self, is_pulling: bool) {
        self._fishing_state._is_pulling = is_pulling;
    }

    pub fn on_pull_press(&mut self) {
        if !self._fishing_state._is_minigame_active {
            return;
        }

        let dot = self._fishing_state._direction_dot;
        let impulse = if dot >= FISHING_ALIGNMENT_MATCH_DOT {
            let t = (dot - FISHING_ALIGNMENT_MATCH_DOT) / (1.0 - FISHING_ALIGNMENT_MATCH_DOT);
            -FISHING_PRESS_BONUS_MAX * t
        } else {
            let t = (FISHING_ALIGNMENT_MATCH_DOT - dot) / (1.0 + FISHING_ALIGNMENT_MATCH_DOT);
            FISHING_PRESS_PENALTY_MAX * t
        };

        self._fishing_state._fish_gauge += impulse;
        self._fishing_state._fish_gauge = self._fishing_state._fish_gauge.clamp(0.0, 1.0);
    }

    pub fn update_fishing_minigame(&mut self, delta_time: f32) {
        if !self._fishing_state._is_minigame_active {
            return;
        }

        let angle_diff_target = self._fishing_state._fish_target_angle - self._fishing_state._fish_angle;
        let distance_to_target = angle_diff_target.abs();
        if distance_to_target > 0.001 {
            let max_expected_diff = (self._fishing_state._difficulty_angle_range * 1.5).max(45.0);
            let dist_ratio = (distance_to_target / max_expected_diff).clamp(0.0, 1.0);
            let speed_factor = FISHING_FISH_TURN_MIN_FACTOR + (1.0 - FISHING_FISH_TURN_MIN_FACTOR) * dist_ratio;
            let turn_speed = FISHING_FISH_BASE_TURN_SPEED * speed_factor;

            let step = angle_diff_target.signum() * (turn_speed * delta_time).min(distance_to_target);
            self._fishing_state._fish_angle += step;
        }

        self._fishing_state._fish_change_timer -= delta_time;
        let reached_target = (self._fishing_state._fish_target_angle - self._fishing_state._fish_angle).abs() <= 0.5;
        if reached_target || self._fishing_state._fish_change_timer <= 0.0 {
            self._fishing_state._fish_change_timer = FISHING_MINIGAME_CHANGE_TIMER_MIN + rand::random::<f32>() * FISHING_MINIGAME_CHANGE_TIMER_RANGE;
            let angle_range = self._fishing_state._difficulty_angle_range;
            self._fishing_state._fish_target_angle = (rand::random::<f32>() * 2.0 - 1.0) * angle_range;
        }

        let player_rad = self._fishing_state._player_angle.to_radians();
        let fish_rad = self._fishing_state._fish_angle.to_radians();
        let v_player = (player_rad.sin(), player_rad.cos());
        let v_fish = (fish_rad.sin(), fish_rad.cos());
        let dot = (v_player.0 * v_fish.0 + v_player.1 * v_fish.1).clamp(-1.0, 1.0);

        self._fishing_state._direction_dot = dot;
        self._fishing_state._is_direction_matched = dot >= FISHING_ALIGNMENT_MATCH_DOT;

        if self._fishing_state._is_pulling {
            let rate = if dot >= FISHING_ALIGNMENT_MATCH_DOT {
                let t = (dot - FISHING_ALIGNMENT_MATCH_DOT) / (1.0 - FISHING_ALIGNMENT_MATCH_DOT);
                -FISHING_PULL_DECREASE_MAX * t
            } else {
                let t = (FISHING_ALIGNMENT_MATCH_DOT - dot) / (1.0 + FISHING_ALIGNMENT_MATCH_DOT);
                FISHING_PULL_FAIL_INCREASE_MAX * t
            };
            self._fishing_state._fish_gauge += rate * delta_time;
        } else {
            self._fishing_state._fish_gauge += FISHING_IDLE_INCREASE_SPEED * delta_time;
        }

        self._fishing_state._fish_gauge = self._fishing_state._fish_gauge.clamp(0.0, 1.0);

        if self._fishing_state._fish_gauge <= 0.0 {
            self._fishing_state._is_minigame_active = false;
            self._fishing_state._minigame_success = Some(true);
            self.set_action_fishing_end();
        } else if self._fishing_state._fish_gauge >= 1.0 {
            self._fishing_state._is_minigame_active = false;
            self._fishing_state._minigame_success = Some(false);
            self.set_action_fishing_end();
        }
    }

    pub fn set_action_fishing_end(&mut self) {
        self.set_next_action_animation(ActionAnimationState::FishingEnd, 1.0);
        self.set_move_idle();
    }
}
