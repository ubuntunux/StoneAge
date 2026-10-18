use super::character::Character;
use crate::game_module::actors::character::data::{
    ActionAnimationState, ActionEvent, CharacterData, MoveAnimationState,
};
pub use crate::game_module::actors::common::ActorWrapper;
use crate::game_module::game_constants::{
    BODY_TEMP_COLD_DRAIN_RATE, BODY_TEMP_DEFAULT_DRAIN_RATE, BODY_TEMP_DEFAULT_RECOVERY_RATE,
    BODY_TEMP_HEAT_RECOVERY_RATE, BODY_TEMP_HEAT_SOURCE_BONUS, BODY_TEMP_HEAT_SOURCE_MAX_OFFSET,
    BODY_TEMP_RAIN_DRAIN_MULTIPLIER, BODY_TEMP_RAIN_MIN_WETNESS, BODY_TEMP_RAIN_PENALTY, BODY_TEMP_WET_DRAIN_FACTOR,
    BODY_TEMP_WETNESS_PENALTY_FACTOR, HEAT_SOURCE_RADIUS, HUNGER_RATE, HUNGER_WARNING_THRESHOLD,
    HYPOTHERMIA_RECOVERY_THRESHOLD, HYPOTHERMIA_THRESHOLD, MAX_BODY_TEMPERATURE, MAX_CORPSE_HIT_COUNT,
    MAX_DRY_COLD_BODY_TEMP_DROP, MAX_DRY_HEAT_BODY_TEMP_RISE, MAX_HUNGER, MAX_STAMINA, MIN_BODY_TEMPERATURE,
    NORMAL_BODY_TEMPERATURE, SHELTER_PROTECTION_RADIUS, STAMINA_RECOVERY, STAMINA_RECOVERY_DELAY_TIME, STAMINA_RUN,
    TEMPERATURE_MAX, TEMPERATURE_MIN, THERMONEUTRAL_MAX_TEMP, THERMONEUTRAL_MIN_TEMP, WETNESS_DRY_DEFAULT_MULTIPLIER,
    WETNESS_DRY_HEAT_SOURCE_MULTIPLIER, WETNESS_DRY_RATE, WETNESS_INCREASE_RATE,
};
use crate::game_module::game_service_locator::get_game_scene_manager;
use crate::game_module::game_weather::WeatherType;
use serde::{Deserialize, Serialize};

fn default_body_temperature() -> f32 {
    NORMAL_BODY_TEMPERATURE
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterStatsSaveData {
    pub _is_alive: bool,
    pub _is_tamed: bool,
    #[serde(alias = "_is_dead_loop")]
    pub _is_down_loop: bool,
    pub _corpse_hit_count: i32,
    pub _hp: i32,
    pub _max_hp: i32,
    pub _max_hp_data: i32,
    pub _stamina_recovery_delay_time: f32,
    pub _prev_stamina: f32,
    pub _stamina: f32,
    pub _max_stamina: f32,
    pub _max_stamina_data: f32,
    pub _hunger: f32,
    pub _tired: f32,
    pub _happiness: f32,
    pub _intimacy: f32,
    pub _invincibility: bool,
    pub _is_stat_displayed: bool,
    #[serde(default = "default_body_temperature")]
    pub _body_temperature: f32,
    pub _wetness: f32,
    pub _is_hypothermia: bool,
}

impl Default for CharacterStatsSaveData {
    fn default() -> Self {
        Self {
            _is_alive: false,
            _is_tamed: false,
            _is_down_loop: false,
            _corpse_hit_count: 0,
            _hp: 0,
            _max_hp: 0,
            _max_hp_data: 0,
            _stamina_recovery_delay_time: 0.0,
            _prev_stamina: 0.0,
            _stamina: 0.0,
            _max_stamina: 0.0,
            _max_stamina_data: 0.0,
            _hunger: 0.0,
            _tired: 0.0,
            _happiness: 0.0,
            _intimacy: 0.0,
            _invincibility: false,
            _is_stat_displayed: false,
            _body_temperature: NORMAL_BODY_TEMPERATURE,
            _wetness: 0.0,
            _is_hypothermia: false,
        }
    }
}

pub struct CharacterStats {
    pub _is_alive: bool,
    pub _is_tamed: bool,
    pub _is_down_loop: bool,
    pub _corpse_hit_count: i32,
    pub _hp: i32,
    pub _max_hp: i32,
    pub _max_hp_data: i32,
    pub _stamina_recovery_delay_time: f32,
    pub _prev_stamina: f32,
    pub _stamina: f32,
    pub _max_stamina: f32,
    pub _max_stamina_data: f32,
    pub _hunger: f32,
    pub _tired: f32,
    pub _happiness: f32,
    pub _intimacy: f32,
    pub _invincibility: bool,
    pub _is_stat_displayed: bool,
    pub _is_interacting: bool,
    pub _hit_blink_time: f32,
    pub _body_temperature: f32,
    pub _wetness: f32,
    pub _is_hypothermia: bool,
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self::create_character_stats()
    }
}

impl CharacterStats {
    pub fn create_character_stats() -> CharacterStats {
        CharacterStats {
            _is_alive: true,
            _is_tamed: false,
            _is_down_loop: false,
            _corpse_hit_count: MAX_CORPSE_HIT_COUNT,
            _hp: 100,
            _max_hp: 100,
            _max_hp_data: 100,
            _stamina_recovery_delay_time: 0.0,
            _prev_stamina: MAX_STAMINA,
            _stamina: MAX_STAMINA,
            _max_stamina: MAX_STAMINA,
            _max_stamina_data: MAX_STAMINA,
            _hunger: 0.0,
            _tired: 0.0,
            _happiness: 1.0,
            _intimacy: 0.0,
            _invincibility: false,
            _is_stat_displayed: false,
            _is_interacting: false,
            _hit_blink_time: 0.0,
            _body_temperature: NORMAL_BODY_TEMPERATURE,
            _wetness: 0.0,
            _is_hypothermia: false,
        }
    }

    pub fn initialize_character_stats(&mut self, character_data: &CharacterData) {
        self._is_alive = true;
        self._hp = character_data._stat_data._max_hp;
        self._max_hp = character_data._stat_data._max_hp;
        self._max_hp_data = character_data._stat_data._max_hp;
        self._stamina_recovery_delay_time = 0.0;
        self._prev_stamina = MAX_STAMINA;
        self._stamina = MAX_STAMINA;
        self._max_stamina = MAX_STAMINA;
        self._max_stamina_data = MAX_STAMINA;
        self._hunger = 0.0;
        self._invincibility = false;
        self._body_temperature = NORMAL_BODY_TEMPERATURE;
        self._wetness = 0.0;
        self._is_hypothermia = false;
    }

    pub fn get_hp(&self) -> i32 {
        self._hp
    }

    pub fn set_hp(&mut self, hp: i32) {
        self._hp = self._max_hp.min(0.max(hp));
    }

    pub fn add_hp(&mut self, hp: i32) {
        self.set_hp(self.get_hp() + hp);
    }

    pub fn get_max_hp(&self) -> i32 {
        self._max_hp
    }

    pub fn set_max_hp(&mut self, hp: i32) {
        self._max_hp = self._max_hp_data.min(0.max(hp));
    }

    pub fn add_max_hp(&mut self, hp: i32) {
        self.set_max_hp(self.get_max_hp() + hp);
    }

    pub fn get_max_hp_data(&self) -> i32 {
        self._max_hp_data
    }

    pub fn is_hungry(&self) -> bool {
        HUNGER_WARNING_THRESHOLD <= self._hunger
    }

    pub fn get_hunger(&self) -> f32 {
        self._hunger
    }

    pub fn set_hunger(&mut self, hunger: f32) {
        self._hunger = MAX_HUNGER.min(0f32.max(hunger));
        let max_stat_ratio = 1.0
            - 1f32.min(
                (((self._hunger - HUNGER_WARNING_THRESHOLD) / (MAX_HUNGER - HUNGER_WARNING_THRESHOLD)) * 10.0).ceil()
                    / 10.0,
            );
        self.set_max_hp((self._max_hp_data as f32 * max_stat_ratio).ceil() as i32);
        if self._max_hp < self._hp {
            self.set_hp(self._max_hp);
        }

        self.set_max_stamina((self._max_stamina_data * max_stat_ratio).ceil());
        if self._max_stamina < self._stamina {
            self.set_stamina(self._max_stamina);
        }
    }

    pub fn add_hunger(&mut self, hunger: f32) {
        self.set_hunger(self.get_hunger() + hunger);
    }

    pub fn update_hunger(&mut self, delta_time: f32) {
        self.add_hunger(HUNGER_RATE * delta_time);
    }

    pub fn get_tired(&self) -> f32 {
        self._tired
    }

    pub fn set_tired(&mut self, tired: f32) {
        self._tired = tired;
    }

    pub fn get_happiness(&self) -> f32 {
        self._happiness
    }

    pub fn set_happiness(&mut self, happiness: f32) {
        self._happiness = happiness;
    }

    pub fn get_intimacy(&self) -> f32 {
        self._intimacy
    }

    pub fn set_intimacy(&mut self, intimacy: f32) {
        self._intimacy = intimacy;
    }

    pub fn add_intimacy(&mut self, intimacy: f32) {
        self._intimacy += intimacy;
    }

    pub fn get_stamina(&self) -> f32 {
        self._stamina
    }

    pub fn set_stamina(&mut self, stamina: f32) {
        self._stamina = self._max_stamina.min(0f32.max(stamina));
    }

    pub fn add_stamina(&mut self, stamina: f32) {
        self.set_stamina(self.get_stamina() + stamina);
    }

    pub fn get_max_stamina(&self) -> f32 {
        self._max_stamina
    }

    pub fn set_max_stamina(&mut self, stamina: f32) {
        self._max_stamina = self._max_stamina_data.min(0f32.max(stamina));
    }

    pub fn add_max_stamina(&mut self, stamina: f32) {
        self.set_max_stamina(self.get_max_stamina() + stamina);
    }

    pub fn get_max_stamina_data(&self) -> f32 {
        self._max_stamina_data
    }

    pub fn set_invincibility(&mut self, invincibility: bool) {
        self._invincibility = invincibility;
    }

    pub fn get_is_stat_displayed(&self) -> bool {
        self._is_stat_displayed
    }

    pub fn set_is_stat_displayed(&mut self, is_stat_displayed: bool) {
        self._is_stat_displayed = is_stat_displayed
    }

    pub fn get_body_temperature(&self) -> f32 {
        self._body_temperature
    }

    pub fn set_body_temperature(&mut self, temp: f32) {
        self._body_temperature = temp.clamp(MIN_BODY_TEMPERATURE, MAX_BODY_TEMPERATURE);
    }

    pub fn get_wetness(&self) -> f32 {
        self._wetness
    }

    pub fn set_wetness(&mut self, wetness: f32) {
        self._wetness = wetness.clamp(0.0, 1.0);
    }

    pub fn is_hypothermia(&self) -> bool {
        self._is_hypothermia
    }

    pub fn set_is_hypothermia(&mut self, is_hypothermia: bool) {
        self._is_hypothermia = is_hypothermia;
    }

    pub fn update_body_temperature<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        let is_raining = get_game_scene_manager()._weather.get_weather_type() == WeatherType::Rain;
        let env_temp = get_game_scene_manager().temperature();
        let mut is_near_heat_source = false;
        let mut is_sheltered = false;

        let char_pos = owner.get_position();

        for prop in get_game_scene_manager().get_prop_manager().get_props().values() {
            let prop_ref = prop.borrow();
            let distance = (prop_ref.get_position() - char_pos).norm();

            if distance <= HEAT_SOURCE_RADIUS && prop_ref.is_heat_source() {
                is_near_heat_source = true;
            }

            if distance <= SHELTER_PROTECTION_RADIUS && prop_ref.is_shelter() {
                is_sheltered = true;
            }
        }

        if is_raining && !is_sheltered && !is_near_heat_source {
            self._wetness = (self._wetness + WETNESS_INCREASE_RATE * delta_time).min(1.0);
        } else {
            let dry_multiplier = if is_near_heat_source {
                WETNESS_DRY_HEAT_SOURCE_MULTIPLIER
            } else {
                WETNESS_DRY_DEFAULT_MULTIPLIER
            };
            self._wetness = (self._wetness - WETNESS_DRY_RATE * dry_multiplier * delta_time).max(0.0);
        }

        // Calculate target body temperature based on realistic human thermoregulation (homeostasis)
        let mut target_temp = NORMAL_BODY_TEMPERATURE;

        if env_temp < THERMONEUTRAL_MIN_TEMP {
            let cold_ratio =
                ((THERMONEUTRAL_MIN_TEMP - env_temp) / (THERMONEUTRAL_MIN_TEMP - TEMPERATURE_MIN)).clamp(0.0, 1.0);
            target_temp -= cold_ratio * MAX_DRY_COLD_BODY_TEMP_DROP;
        } else if env_temp > THERMONEUTRAL_MAX_TEMP {
            let heat_ratio =
                ((env_temp - THERMONEUTRAL_MAX_TEMP) / (TEMPERATURE_MAX - THERMONEUTRAL_MAX_TEMP)).clamp(0.0, 1.0);
            target_temp += heat_ratio * MAX_DRY_HEAT_BODY_TEMP_RISE;
        }

        if is_near_heat_source {
            target_temp = (target_temp + BODY_TEMP_HEAT_SOURCE_BONUS).clamp(
                NORMAL_BODY_TEMPERATURE,
                NORMAL_BODY_TEMPERATURE + BODY_TEMP_HEAT_SOURCE_MAX_OFFSET,
            );
        } else {
            if is_raining && !is_sheltered {
                target_temp -= BODY_TEMP_RAIN_PENALTY;
            }
            if self._wetness > 0.0 {
                target_temp -= self._wetness * BODY_TEMP_WETNESS_PENALTY_FACTOR;
            }
        }

        target_temp = target_temp.clamp(MIN_BODY_TEMPERATURE, MAX_BODY_TEMPERATURE);

        let temp_diff = target_temp - self._body_temperature;
        if temp_diff > 0.0 {
            let recovery_rate = if is_near_heat_source {
                BODY_TEMP_HEAT_RECOVERY_RATE
            } else {
                BODY_TEMP_DEFAULT_RECOVERY_RATE
            };
            self._body_temperature = (self._body_temperature + recovery_rate * delta_time).min(target_temp);
        } else if temp_diff < 0.0 {
            let mut drain_rate = BODY_TEMP_DEFAULT_DRAIN_RATE;
            if is_raining && !is_sheltered {
                drain_rate = BODY_TEMP_COLD_DRAIN_RATE
                    * (self._wetness.max(BODY_TEMP_RAIN_MIN_WETNESS) * BODY_TEMP_RAIN_DRAIN_MULTIPLIER);
            } else if self._wetness > 0.0 {
                drain_rate = self._wetness * BODY_TEMP_WET_DRAIN_FACTOR;
            }
            self._body_temperature = (self._body_temperature - drain_rate * delta_time).max(target_temp);
        }

        if self._body_temperature <= HYPOTHERMIA_THRESHOLD {
            self._is_hypothermia = true;
        } else if self._body_temperature >= HYPOTHERMIA_RECOVERY_THRESHOLD {
            self._is_hypothermia = false;
        }
    }

    pub fn update_hp<'a>(&mut self, _owner: &Character<'a>, _delta_time: f32) {
        if self._max_hp < self._hp {
            self._hp = self._max_hp;
        }
    }

    pub fn update_stamina<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        if self._max_stamina < self._stamina {
            self._stamina = self._max_stamina;
        }

        if self._prev_stamina != self._stamina {
            if self._stamina < self._prev_stamina {
                self._stamina_recovery_delay_time = STAMINA_RECOVERY_DELAY_TIME;
            }
            self._prev_stamina = self._stamina;
        }

        if owner.is_move_state(MoveAnimationState::Run) {
            self._stamina = (self._stamina - STAMINA_RUN * delta_time).max(0.0);
        } else if owner.is_idle_action()
            && (owner.is_move_stop()
                || owner.is_move_state(MoveAnimationState::SitDownLoop)
                || owner.is_move_state(MoveAnimationState::Walk))
        {
            self._stamina = self._stamina.max(0.0);

            if self._stamina_recovery_delay_time <= 0.0 {
                self._stamina = (self._stamina + STAMINA_RECOVERY * delta_time).min(self._max_stamina);
            } else {
                self._stamina_recovery_delay_time -= delta_time;
            }
        }
    }

    pub fn update_stat<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        if self._is_alive {
            if owner._is_player || owner.get_character_type().is_family() {
                self.update_hunger(delta_time);
            }

            if owner._is_player {
                self.update_hp(owner, delta_time);
                self.update_stamina(owner, delta_time);
                self.update_body_temperature(owner, delta_time);
            }
        }
    }

    pub fn get_character_stats_save_data(&self) -> CharacterStatsSaveData {
        CharacterStatsSaveData {
            _is_alive: self._is_alive,
            _is_tamed: self._is_tamed,
            _is_down_loop: self._is_down_loop,
            _corpse_hit_count: self._corpse_hit_count,
            _hp: self._hp,
            _max_hp: self._max_hp,
            _max_hp_data: self._max_hp_data,
            _stamina_recovery_delay_time: self._stamina_recovery_delay_time,
            _prev_stamina: self._prev_stamina,
            _stamina: self._stamina,
            _max_stamina: self._max_stamina,
            _max_stamina_data: self._max_stamina_data,
            _hunger: self._hunger,
            _tired: self._tired,
            _happiness: self._happiness,
            _intimacy: self._intimacy,
            _invincibility: self._invincibility,
            _is_stat_displayed: self._is_stat_displayed,
            _body_temperature: self._body_temperature,
            _wetness: self._wetness,
            _is_hypothermia: self._is_hypothermia,
        }
    }

    pub fn load_character_stats_save_data(&mut self, save_data: &CharacterStatsSaveData) {
        self._is_alive = save_data._is_alive;
        self._is_tamed = save_data._is_tamed;
        self._is_down_loop = save_data._is_down_loop;
        self._corpse_hit_count = save_data._corpse_hit_count;
        self._hp = save_data._hp;
        self._max_hp = save_data._max_hp;
        self._max_hp_data = save_data._max_hp_data;
        self._stamina_recovery_delay_time = save_data._stamina_recovery_delay_time;
        self._prev_stamina = save_data._prev_stamina;
        self._stamina = save_data._stamina;
        self._max_stamina = save_data._max_stamina;
        self._max_stamina_data = save_data._max_stamina_data;
        self._hunger = save_data._hunger;
        self._tired = save_data._tired;
        self._happiness = save_data._happiness;
        self._intimacy = save_data._intimacy;
        self._invincibility = save_data._invincibility;
        self._body_temperature = if save_data._body_temperature <= 0.0 {
            NORMAL_BODY_TEMPERATURE
        } else {
            save_data._body_temperature
        };
        self._wetness = save_data._wetness;
        self._is_hypothermia = save_data._is_hypothermia;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct CharacterAnimationState {
    pub _move_animation_state: MoveAnimationState,
    pub _prev_move_animation_state: MoveAnimationState,
    pub _next_move_animation_state: MoveAnimationState,
    pub _next_move_animation_speed: f32,
    pub _action_event: ActionEvent,
    pub _action_animation_state: ActionAnimationState,
    pub _next_action_animation_state: Option<ActionAnimationState>,
    pub _next_action_animation_speed: f32,
}

impl CharacterAnimationState {
    pub fn is_attack_event(&self) -> bool {
        matches!(
            self._action_event,
            ActionEvent::Attack | ActionEvent::PowerAttack | ActionEvent::Kick
        )
    }

    pub fn is_action_event(&self, action_event: ActionEvent) -> bool {
        self._action_event == action_event
    }

    pub fn get_action_event(&self) -> ActionEvent {
        self._action_event
    }

    pub fn set_action_event(&mut self, action_event: ActionEvent) {
        self._action_event = action_event;
    }
}
