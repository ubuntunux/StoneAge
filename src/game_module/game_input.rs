use crate::game_module::game_constants::{JOYSTICK_MOVE_DEADZONE, JOYSTICK_SENSITIVITY};
use nalgebra::Vector2;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use winit::keyboard::KeyCode;

// Hold Repeat Navigation Utility
pub const NAV_INITIAL_DELAY: f32 = 0.3;
pub const NAV_REPEAT_INTERVAL: f32 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuickSlotNavDirection {
    Previous,
    Next,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoldRepeatController<T: PartialEq + Copy> {
    pub _initial_delay: f32,
    pub _repeat_interval: f32,
    pub _timer: f32,
    pub _is_repeating: bool,
    pub _last_state: Option<T>,
}

impl<T: PartialEq + Copy> HoldRepeatController<T> {
    pub fn new(initial_delay: f32, repeat_interval: f32) -> Self {
        Self {
            _initial_delay: initial_delay,
            _repeat_interval: repeat_interval,
            _timer: 0.0,
            _is_repeating: false,
            _last_state: None,
        }
    }

    pub fn reset(&mut self) {
        self._timer = 0.0;
        self._is_repeating = false;
        self._last_state = None;
    }

    pub fn update(&mut self, current_state: Option<T>, delta_time: f32) -> (bool, Option<T>) {
        if let Some(state) = current_state {
            if Some(state) != self._last_state {
                self._last_state = Some(state);
                self._timer = self._initial_delay;
                self._is_repeating = false;
                (true, Some(state))
            } else {
                self._timer -= delta_time;
                if self._timer <= 0.0 {
                    if !self._is_repeating {
                        self._is_repeating = true;
                        self._timer = self._repeat_interval;
                    } else {
                        self._timer += self._repeat_interval;
                    }
                    (true, Some(state))
                } else {
                    (false, None)
                }
            }
        } else {
            self.reset();
            (false, None)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WidgetNavRepeatController {
    pub _controller: HoldRepeatController<(i32, i32)>,
}

impl Default for WidgetNavRepeatController {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetNavRepeatController {
    pub fn new() -> Self {
        Self {
            _controller: HoldRepeatController::new(NAV_INITIAL_DELAY, NAV_REPEAT_INTERVAL),
        }
    }

    pub fn with_delay_and_interval(initial_delay: f32, repeat_interval: f32) -> Self {
        Self {
            _controller: HoldRepeatController::new(initial_delay, repeat_interval),
        }
    }

    pub fn reset(&mut self) {
        self._controller.reset();
    }

    pub fn get_direction(
        keyboard_input_data: &KeyboardInputData,
        joystick_input_data: &JoystickInputData,
    ) -> Option<(i32, i32)> {
        let is_left = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || keyboard_input_data.get_key_hold(KeyCode::KeyA)
            || joystick_input_data._btn_left == ButtonState::Pressed
            || joystick_input_data._btn_left == ButtonState::Hold
            || joystick_input_data._stick_left_direction.x < -10000;

        let is_right = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || keyboard_input_data.get_key_hold(KeyCode::KeyD)
            || joystick_input_data._btn_right == ButtonState::Pressed
            || joystick_input_data._btn_right == ButtonState::Hold
            || joystick_input_data._stick_left_direction.x > 10000;

        let is_up = keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_hold(KeyCode::KeyW)
            || joystick_input_data._btn_up == ButtonState::Pressed
            || joystick_input_data._btn_up == ButtonState::Hold
            || joystick_input_data._stick_left_direction.y > 10000;

        let is_down = keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_hold(KeyCode::KeyS)
            || joystick_input_data._btn_down == ButtonState::Pressed
            || joystick_input_data._btn_down == ButtonState::Hold
            || joystick_input_data._stick_left_direction.y < -10000;

        if is_left {
            Some((-1, 0))
        } else if is_right {
            Some((1, 0))
        } else if is_up {
            Some((0, -1))
        } else if is_down {
            Some((0, 1))
        } else {
            None
        }
    }

    pub fn update(
        &mut self,
        keyboard_input_data: &KeyboardInputData,
        joystick_input_data: &JoystickInputData,
        delta_time: f32,
    ) -> (bool, Option<(i32, i32)>) {
        let current_dir = Self::get_direction(keyboard_input_data, joystick_input_data);
        self._controller.update(current_dir, delta_time)
    }
}

pub static DEFAULT_BINDINGS: LazyLock<InputBindings> = LazyLock::new(InputBindings::default);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UiAction {
    Confirm,
    Cancel,
    Up,
    Down,
    Left,
    Right,
    NextTab,
    PrevTab,
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GamePlayAction {
    AttackOrUseItem,
    PowerAttack,
    Cancel,
    Jump,
    Run,
    Roll,
    Interact,
    Request,
    PreviousItem,
    NextItem,
    Help,
    OpenInventory,
    OpenMenu,
    PrevQuickSlotRow,
    NextQuickSlotRow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,
    LeftStick,
    RightStick,
    Back,
    Start,
    Guide,
}

impl GamepadButton {
    pub fn get_state(&self, joystick: &JoystickInputData) -> ButtonState {
        match self {
            GamepadButton::A => joystick._btn_a,
            GamepadButton::B => joystick._btn_b,
            GamepadButton::X => joystick._btn_x,
            GamepadButton::Y => joystick._btn_y,
            GamepadButton::DpadUp => joystick._btn_up,
            GamepadButton::DpadDown => joystick._btn_down,
            GamepadButton::DpadLeft => joystick._btn_left,
            GamepadButton::DpadRight => joystick._btn_right,
            GamepadButton::LeftShoulder => joystick._btn_left_shoulder,
            GamepadButton::RightShoulder => joystick._btn_right_shoulder,
            GamepadButton::LeftTrigger => joystick._btn_left_trigger,
            GamepadButton::RightTrigger => joystick._btn_right_trigger,
            GamepadButton::LeftStick => joystick._btn_left_stick,
            GamepadButton::RightStick => joystick._btn_right_stick,
            GamepadButton::Back => joystick._btn_back,
            GamepadButton::Start => joystick._btn_start,
            GamepadButton::Guide => joystick._btn_guide,
        }
    }

    pub fn is_pressed(&self, joystick: &JoystickInputData) -> bool {
        self.get_state(joystick) == ButtonState::Pressed
    }

    pub fn is_hold(&self, joystick: &JoystickInputData) -> bool {
        let state = self.get_state(joystick);
        state == ButtonState::Hold || state == ButtonState::Pressed
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    pub fn is_pressed(&self, mouse: &MouseInputData) -> bool {
        match self {
            MouseButton::Left => mouse._btn_l_pressed,
            MouseButton::Right => mouse._btn_r_pressed,
            MouseButton::Middle => mouse._btn_m_pressed,
        }
    }

    pub fn is_hold(&self, mouse: &MouseInputData) -> bool {
        match self {
            MouseButton::Left => mouse._btn_l_hold,
            MouseButton::Right => mouse._btn_r_hold,
            MouseButton::Middle => mouse._btn_m_hold,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBinding {
    pub _key_codes: Vec<KeyCode>,
    pub _gamepad_buttons: Vec<GamepadButton>,
    pub _mouse_buttons: Vec<MouseButton>,
}

impl ActionBinding {
    pub fn new(key_codes: Vec<KeyCode>, gamepad_buttons: Vec<GamepadButton>, mouse_buttons: Vec<MouseButton>) -> Self {
        Self {
            _key_codes: key_codes,
            _gamepad_buttons: gamepad_buttons,
            _mouse_buttons: mouse_buttons,
        }
    }

    pub fn is_pressed(
        &self,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: Option<&MouseInputData>,
    ) -> bool {
        let key_pressed = self._key_codes.iter().any(|k| keyboard.get_key_pressed(*k));
        let pad_pressed = self._gamepad_buttons.iter().any(|b| b.is_pressed(joystick));
        let mouse_pressed = mouse.is_some_and(|m| self._mouse_buttons.iter().any(|b| b.is_pressed(m)));
        key_pressed || pad_pressed || mouse_pressed
    }

    pub fn is_hold(
        &self,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: Option<&MouseInputData>,
    ) -> bool {
        let key_hold = self._key_codes.iter().any(|k| keyboard.get_key_hold(*k) || keyboard.get_key_pressed(*k));
        let pad_hold = self._gamepad_buttons.iter().any(|b| b.is_hold(joystick));
        let mouse_hold = mouse.is_some_and(|m| self._mouse_buttons.iter().any(|b| b.is_hold(m)));
        key_hold || pad_hold || mouse_hold
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputBindings {
    pub _ui_bindings: HashMap<UiAction, ActionBinding>,
    pub _gameplay_bindings: HashMap<GamePlayAction, ActionBinding>,
}

impl Default for InputBindings {
    fn default() -> Self {
        let mut ui_bindings = HashMap::new();
        ui_bindings.insert(
            UiAction::Confirm,
            ActionBinding::new(
                vec![KeyCode::Enter, KeyCode::Space, KeyCode::NumpadEnter, KeyCode::KeyE],
                vec![GamepadButton::A],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::Cancel,
            ActionBinding::new(
                vec![KeyCode::Escape, KeyCode::KeyB],
                vec![GamepadButton::B, GamepadButton::Start],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::Up,
            ActionBinding::new(
                vec![KeyCode::ArrowUp, KeyCode::KeyW],
                vec![GamepadButton::DpadUp],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::Down,
            ActionBinding::new(
                vec![KeyCode::ArrowDown, KeyCode::KeyS],
                vec![GamepadButton::DpadDown],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::Left,
            ActionBinding::new(
                vec![KeyCode::ArrowLeft, KeyCode::KeyA],
                vec![GamepadButton::DpadLeft],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::Right,
            ActionBinding::new(
                vec![KeyCode::ArrowRight, KeyCode::KeyD],
                vec![GamepadButton::DpadRight],
                vec![],
            ),
        );
        ui_bindings.insert(
            UiAction::NextTab,
            ActionBinding::new(vec![KeyCode::Tab], vec![GamepadButton::RightShoulder], vec![]),
        );
        ui_bindings.insert(
            UiAction::PrevTab,
            ActionBinding::new(vec![], vec![GamepadButton::LeftShoulder], vec![]),
        );
        ui_bindings.insert(
            UiAction::Delete,
            ActionBinding::new(vec![KeyCode::Delete, KeyCode::KeyF], vec![GamepadButton::X], vec![]),
        );

        let mut gameplay_bindings = HashMap::new();
        gameplay_bindings.insert(
            GamePlayAction::AttackOrUseItem,
            ActionBinding::new(vec![], vec![GamepadButton::RightShoulder], vec![MouseButton::Left]),
        );
        gameplay_bindings.insert(
            GamePlayAction::PowerAttack,
            ActionBinding::new(vec![], vec![GamepadButton::RightTrigger], vec![MouseButton::Right]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Cancel,
            ActionBinding::new(vec![KeyCode::KeyB, KeyCode::Escape], vec![GamepadButton::B], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Jump,
            ActionBinding::new(vec![KeyCode::Space], vec![GamepadButton::A], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Run,
            ActionBinding::new(vec![KeyCode::ShiftLeft], vec![GamepadButton::LeftShoulder], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Roll,
            ActionBinding::new(vec![KeyCode::AltLeft], vec![GamepadButton::B], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Interact,
            ActionBinding::new(vec![KeyCode::KeyF], vec![GamepadButton::X], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::Request,
            ActionBinding::new(vec![KeyCode::KeyC], vec![GamepadButton::Y], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::PreviousItem,
            ActionBinding::new(
                vec![KeyCode::ArrowLeft, KeyCode::KeyQ],
                vec![GamepadButton::DpadLeft],
                vec![],
            ),
        );
        gameplay_bindings.insert(
            GamePlayAction::NextItem,
            ActionBinding::new(
                vec![KeyCode::ArrowRight, KeyCode::KeyE],
                vec![GamepadButton::DpadRight],
                vec![],
            ),
        );
        gameplay_bindings.insert(
            GamePlayAction::Help,
            ActionBinding::new(vec![KeyCode::F1], vec![GamepadButton::Back], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::OpenInventory,
            ActionBinding::new(vec![KeyCode::KeyI], vec![GamepadButton::Start], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::OpenMenu,
            ActionBinding::new(vec![KeyCode::Escape], vec![GamepadButton::Start], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::PrevQuickSlotRow,
            ActionBinding::new(vec![], vec![GamepadButton::DpadUp], vec![]),
        );
        gameplay_bindings.insert(
            GamePlayAction::NextQuickSlotRow,
            ActionBinding::new(vec![KeyCode::Tab], vec![GamepadButton::DpadDown], vec![]),
        );

        Self {
            _ui_bindings: ui_bindings,
            _gameplay_bindings: gameplay_bindings,
        }
    }
}

impl InputBindings {
    pub fn is_ui_action_pressed(
        &self,
        action: UiAction,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
    ) -> bool {
        if action == UiAction::NextTab {
            let is_shift = keyboard.get_key_hold(KeyCode::ShiftLeft) || keyboard.get_key_hold(KeyCode::ShiftRight);
            if is_shift {
                return false;
            }
        } else if action == UiAction::PrevTab {
            let is_shift = keyboard.get_key_hold(KeyCode::ShiftLeft) || keyboard.get_key_hold(KeyCode::ShiftRight);
            if is_shift && keyboard.get_key_pressed(KeyCode::Tab) {
                return true;
            }
        }

        if let Some(binding) = self._ui_bindings.get(&action) {
            binding.is_pressed(keyboard, joystick, None)
        } else {
            false
        }
    }

    pub fn is_gameplay_action_pressed(
        &self,
        action: GamePlayAction,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
    ) -> bool {
        if let Some(binding) = self._gameplay_bindings.get(&action) {
            binding.is_pressed(keyboard, joystick, Some(mouse))
        } else {
            false
        }
    }

    pub fn is_gameplay_action_hold(
        &self,
        action: GamePlayAction,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
    ) -> bool {
        if let Some(binding) = self._gameplay_bindings.get(&action) {
            binding.is_hold(keyboard, joystick, Some(mouse))
        } else {
            false
        }
    }

    pub fn bind_ui_key(&mut self, action: UiAction, key: KeyCode) {
        let binding = self._ui_bindings.entry(action).or_default();
        if !binding._key_codes.contains(&key) {
            binding._key_codes.push(key);
        }
    }

    pub fn bind_gameplay_key(&mut self, action: GamePlayAction, key: KeyCode) {
        let binding = self._gameplay_bindings.entry(action).or_default();
        if !binding._key_codes.contains(&key) {
            binding._key_codes.push(key);
        }
    }

    pub fn bind_gameplay_gamepad(&mut self, action: GamePlayAction, button: GamepadButton) {
        let binding = self._gameplay_bindings.entry(action).or_default();
        if !binding._gamepad_buttons.contains(&button) {
            binding._gamepad_buttons.push(button);
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UiInputState {
    pub _confirm: bool,
    pub _cancel: bool,
    pub _up: bool,
    pub _down: bool,
    pub _left: bool,
    pub _right: bool,
    pub _next_tab: bool,
    pub _prev_tab: bool,
    pub _delete: bool,
}

impl UiInputState {
    pub fn from_input(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> Self {
        Self::from_input_with_bindings(&DEFAULT_BINDINGS, keyboard, joystick)
    }

    pub fn from_input_with_bindings(
        bindings: &InputBindings,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
    ) -> Self {
        Self {
            _confirm: bindings.is_ui_action_pressed(UiAction::Confirm, keyboard, joystick),
            _cancel: bindings.is_ui_action_pressed(UiAction::Cancel, keyboard, joystick),
            _up: bindings.is_ui_action_pressed(UiAction::Up, keyboard, joystick),
            _down: bindings.is_ui_action_pressed(UiAction::Down, keyboard, joystick),
            _left: bindings.is_ui_action_pressed(UiAction::Left, keyboard, joystick),
            _right: bindings.is_ui_action_pressed(UiAction::Right, keyboard, joystick),
            _next_tab: bindings.is_ui_action_pressed(UiAction::NextTab, keyboard, joystick),
            _prev_tab: bindings.is_ui_action_pressed(UiAction::PrevTab, keyboard, joystick),
            _delete: bindings.is_ui_action_pressed(UiAction::Delete, keyboard, joystick),
        }
    }
}

pub struct UiInput;

impl UiInput {
    pub fn is_confirm(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Confirm, keyboard, joystick)
    }

    pub fn is_cancel(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Cancel, keyboard, joystick)
    }

    pub fn is_up(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Up, keyboard, joystick)
    }

    pub fn is_down(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Down, keyboard, joystick)
    }

    pub fn is_left(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Left, keyboard, joystick)
    }

    pub fn is_right(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Right, keyboard, joystick)
    }

    pub fn is_next_tab(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::NextTab, keyboard, joystick)
    }

    pub fn is_prev_tab(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::PrevTab, keyboard, joystick)
    }

    pub fn is_delete(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        DEFAULT_BINDINGS.is_ui_action_pressed(UiAction::Delete, keyboard, joystick)
    }

    pub fn is_execute(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        Self::is_confirm(keyboard, joystick) || Self::is_delete(keyboard, joystick)
    }

    pub fn get_pressed_digit(keyboard: &KeyboardInputData) -> Option<usize> {
        let key_codes = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ];
        for (i, key_code) in key_codes.iter().enumerate() {
            if keyboard.get_key_pressed(*key_code) {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn is_item_drop(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        Self::is_delete(keyboard, joystick)
            || keyboard.get_key_pressed(KeyCode::KeyF)
            || keyboard.get_key_pressed(KeyCode::Delete)
            || joystick._btn_b == ButtonState::Pressed
    }

    pub fn is_mouse_drop(mouse: &MouseInputData) -> bool {
        mouse._btn_r_pressed
    }

    pub fn is_mouse_sub_action(mouse: &MouseInputData) -> bool {
        mouse._btn_r_pressed
    }
}

pub struct CookingMinigameInput;

impl CookingMinigameInput {
    pub fn is_advance(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        UiInput::is_confirm(keyboard, joystick)
            || keyboard.get_key_pressed(KeyCode::KeyE)
            || joystick._btn_a == ButtonState::Pressed
    }
}

pub struct GameInput;

impl GameInput {
    pub fn is_any_key_pressed(keyboard: &KeyboardInputData) -> bool {
        keyboard.is_any_key_pressed()
    }

    pub fn get_pressed_quick_slot_index(keyboard: &KeyboardInputData) -> Option<usize> {
        if !keyboard.is_any_key_pressed() {
            return None;
        }

        const QUICK_SLOT_KEY_CODES: [KeyCode; 10] = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::Digit0,
        ];

        for (item_index, key_code) in QUICK_SLOT_KEY_CODES.iter().enumerate() {
            if keyboard.get_key_pressed(*key_code) {
                return Some(item_index);
            }
        }
        None
    }
    pub fn is_moving(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        let is_key_move = [
            KeyCode::KeyW,
            KeyCode::KeyA,
            KeyCode::KeyS,
            KeyCode::KeyD,
            KeyCode::ArrowUp,
            KeyCode::ArrowLeft,
            KeyCode::ArrowDown,
            KeyCode::ArrowRight,
        ]
        .iter()
        .any(|&key| keyboard.get_key_pressed(key) || keyboard.get_key_hold(key));

        let is_stick_move = joystick._stick_left_direction.x.abs() > JOYSTICK_MOVE_DEADZONE
            || joystick._stick_left_direction.y.abs() > JOYSTICK_MOVE_DEADZONE;

        is_key_move || is_stick_move
    }

    pub fn is_any_input(keyboard: &KeyboardInputData, joystick: &JoystickInputData, mouse: &MouseInputData) -> bool {
        Self::is_any_input_pressed(keyboard, joystick, mouse) || Self::is_moving(keyboard, joystick)
    }

    pub fn is_any_input_pressed(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
    ) -> bool {
        keyboard.is_any_key_pressed() || mouse.is_any_button_pressed() || joystick.is_any_button_pressed()
    }

    pub fn is_any_input_hold(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
    ) -> bool {
        keyboard.is_any_key_hold() || mouse.is_any_button_hold() || joystick.is_any_button_hold()
    }

    pub fn is_toggle_game_mode(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> bool {
        (joystick._btn_left_trigger == ButtonState::Hold
            && joystick._btn_right_trigger == ButtonState::Hold
            && joystick._btn_left_shoulder == ButtonState::Hold
            && joystick._btn_right_shoulder == ButtonState::Hold)
            || keyboard.get_key_pressed(KeyCode::Backquote)
            || keyboard.get_key_pressed(KeyCode::Escape)
    }
}

#[derive(Clone, Debug, Default)]
pub struct GamePlayInputState {
    pub _attack_or_use_item: bool,
    pub _power_attack: bool,
    pub _cancel: bool,
    pub _left: bool,
    pub _right: bool,
    pub _up: bool,
    pub _down: bool,
    pub _jump: bool,
    pub _run: bool,
    pub _roll: bool,
    pub _interaction: bool,
    pub _interaction_hold: bool,
    pub _request: bool,
    pub _previous_item: bool,
    pub _next_item: bool,
    pub _help: bool,
    pub _open_inventory: bool,
    pub _open_menu: bool,
    pub _prev_quick_slot_row: bool,
    pub _next_quick_slot_row: bool,
    pub _stick_left_direction: Vector2<f32>,
}

impl GamePlayInputState {
    pub fn from_input(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse_input: &MouseInputData,
    ) -> Self {
        Self::from_input_with_bindings(&DEFAULT_BINDINGS, keyboard, joystick, mouse_input)
    }

    pub fn from_input_with_bindings(
        bindings: &InputBindings,
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse_input: &MouseInputData,
    ) -> Self {
        let is_left = keyboard.get_key_hold(KeyCode::KeyA) || joystick._stick_left_direction.x < 0;
        let is_right = keyboard.get_key_hold(KeyCode::KeyD) || 0 < joystick._stick_left_direction.x;
        let is_down = keyboard.get_key_hold(KeyCode::KeyS) || joystick._stick_left_direction.y < 0;
        let is_up = keyboard.get_key_hold(KeyCode::KeyW) || 0 < joystick._stick_left_direction.y;

        let stick_left_direction = Vector2::<f32>::new(
            joystick._stick_left_direction.x as f32,
            joystick._stick_left_direction.y as f32,
        ) * JOYSTICK_SENSITIVITY;

        let is_interaction_hold =
            bindings.is_gameplay_action_hold(GamePlayAction::Interact, keyboard, joystick, mouse_input);

        Self {
            _attack_or_use_item: bindings.is_gameplay_action_pressed(
                GamePlayAction::AttackOrUseItem,
                keyboard,
                joystick,
                mouse_input,
            ),
            _power_attack: bindings.is_gameplay_action_pressed(
                GamePlayAction::PowerAttack,
                keyboard,
                joystick,
                mouse_input,
            ),
            _cancel: bindings.is_gameplay_action_pressed(GamePlayAction::Cancel, keyboard, joystick, mouse_input),
            _left: is_left,
            _right: is_right,
            _up: is_up,
            _down: is_down,
            _jump: bindings.is_gameplay_action_pressed(GamePlayAction::Jump, keyboard, joystick, mouse_input),
            _run: bindings.is_gameplay_action_pressed(GamePlayAction::Run, keyboard, joystick, mouse_input),
            _roll: bindings.is_gameplay_action_pressed(GamePlayAction::Roll, keyboard, joystick, mouse_input),
            _interaction: bindings.is_gameplay_action_pressed(
                GamePlayAction::Interact,
                keyboard,
                joystick,
                mouse_input,
            ),
            _interaction_hold: is_interaction_hold,
            _request: bindings.is_gameplay_action_pressed(GamePlayAction::Request, keyboard, joystick, mouse_input),
            _previous_item: bindings.is_gameplay_action_pressed(
                GamePlayAction::PreviousItem,
                keyboard,
                joystick,
                mouse_input,
            ) || bindings.is_gameplay_action_hold(
                GamePlayAction::PreviousItem,
                keyboard,
                joystick,
                mouse_input,
            ),
            _next_item: bindings.is_gameplay_action_pressed(GamePlayAction::NextItem, keyboard, joystick, mouse_input)
                || bindings.is_gameplay_action_hold(GamePlayAction::NextItem, keyboard, joystick, mouse_input),
            _help: bindings.is_gameplay_action_pressed(GamePlayAction::Help, keyboard, joystick, mouse_input),
            _open_inventory: bindings.is_gameplay_action_pressed(
                GamePlayAction::OpenInventory,
                keyboard,
                joystick,
                mouse_input,
            ),
            _open_menu: bindings.is_gameplay_action_pressed(GamePlayAction::OpenMenu, keyboard, joystick, mouse_input),
            _prev_quick_slot_row: bindings.is_gameplay_action_pressed(
                GamePlayAction::PrevQuickSlotRow,
                keyboard,
                joystick,
                mouse_input,
            ),
            _next_quick_slot_row: bindings.is_gameplay_action_pressed(
                GamePlayAction::NextQuickSlotRow,
                keyboard,
                joystick,
                mouse_input,
            ),
            _stick_left_direction: stick_left_direction,
        }
    }

    pub fn is_moving(&self) -> bool {
        self._left
            || self._right
            || self._up
            || self._down
            || self._stick_left_direction.x != 0.0
            || self._stick_left_direction.y != 0.0
    }

    pub fn is_any_input(&self, is_any_key: bool) -> bool {
        self.is_moving()
            || self._run
            || self._jump
            || self._roll
            || self._interaction
            || self._request
            || self._attack_or_use_item
            || self._power_attack
            || is_any_key
    }
}

#[derive(Clone, Debug, Default)]
pub struct FishingInputState {
    pub _attack: bool,
    pub _attack_hold: bool,
    pub _cancel: bool,
    pub _left: bool,
    pub _right: bool,
}

impl FishingInputState {
    pub fn from_input(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse_input: &MouseInputData,
    ) -> Self {
        Self {
            _attack: mouse_input._btn_l_pressed || joystick._btn_right_shoulder == ButtonState::Pressed,
            _attack_hold: mouse_input._btn_l_hold || joystick._btn_right_shoulder == ButtonState::Hold,
            _cancel: mouse_input._btn_r_pressed
                || keyboard.get_key_pressed(KeyCode::Escape)
                || joystick._btn_right_trigger == ButtonState::Pressed
                || joystick._btn_b == ButtonState::Pressed,
            _left: keyboard.get_key_hold(KeyCode::KeyA) || joystick._stick_left_direction.x < 0,
            _right: keyboard.get_key_hold(KeyCode::KeyD) || 0 < joystick._stick_left_direction.x,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeployInputState {
    pub _rotate: f32,
    pub _confirm: bool,
    pub _cancel: bool,
}

impl DeployInputState {
    pub fn from_input(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
        is_keyboard_input_mode: bool,
    ) -> Self {
        Self {
            _rotate: DeployInput::get_rotate_input(keyboard, joystick),
            _confirm: DeployInput::is_confirm(keyboard, joystick, mouse, is_keyboard_input_mode),
            _cancel: DeployInput::is_cancel(keyboard, joystick, mouse, is_keyboard_input_mode),
        }
    }
}

pub struct DeployInput;

impl DeployInput {
    pub fn get_rotate_input(keyboard: &KeyboardInputData, joystick: &JoystickInputData) -> f32 {
        let mut rotate_input: f32 = 0.0;
        if keyboard.get_key_hold(KeyCode::KeyQ) || joystick._btn_left_shoulder == ButtonState::Hold {
            rotate_input -= 1.0;
        }
        if keyboard.get_key_hold(KeyCode::KeyE) || joystick._btn_right_shoulder == ButtonState::Hold {
            rotate_input += 1.0;
        }
        rotate_input
    }

    pub fn is_confirm(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
        is_keyboard_input_mode: bool,
    ) -> bool {
        if is_keyboard_input_mode {
            mouse._btn_l_pressed || keyboard.get_key_pressed(KeyCode::Space)
        } else {
            joystick._btn_a == ButtonState::Pressed
        }
    }

    pub fn is_cancel(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse: &MouseInputData,
        is_keyboard_input_mode: bool,
    ) -> bool {
        if is_keyboard_input_mode {
            mouse._btn_r_pressed || keyboard.get_key_pressed(KeyCode::Escape)
        } else {
            joystick._btn_b == ButtonState::Pressed || keyboard.get_key_pressed(KeyCode::KeyB)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraControlState {
    pub _pitch: f32,
    pub _yaw: f32,
    pub _zoom: f32,
}

pub struct CameraInput;

impl CameraInput {
    pub const MOUSE_SENSITIVITY: f32 = 0.001;

    pub fn is_zoom_in(keyboard: &KeyboardInputData, joystick: &JoystickInputData, mouse_move: &MouseMoveData) -> bool {
        keyboard.get_key_hold(KeyCode::ArrowUp)
            || 0 < mouse_move._scroll_delta.y
            || joystick._btn_up == ButtonState::Hold
    }

    pub fn is_zoom_out(keyboard: &KeyboardInputData, joystick: &JoystickInputData, mouse_move: &MouseMoveData) -> bool {
        keyboard.get_key_hold(KeyCode::ArrowDown)
            || mouse_move._scroll_delta.y < 0
            || joystick._btn_down == ButtonState::Hold
    }

    pub fn get_camera_control_state(
        keyboard: &KeyboardInputData,
        joystick: &JoystickInputData,
        mouse_move: &MouseMoveData,
    ) -> CameraControlState {
        let mouse_pos_delta = Vector2::<f32>::new(
            mouse_move._mouse_pos_delta.x as f32,
            mouse_move._mouse_pos_delta.y as f32,
        ) * Self::MOUSE_SENSITIVITY;
        let mouse_scroll_delta =
            Vector2::<f32>::new(mouse_move._scroll_delta.x as f32, mouse_move._scroll_delta.y as f32);

        let stick_right_direction = Vector2::<f32>::new(
            joystick._stick_right_direction.x as f32,
            joystick._stick_right_direction.y as f32,
        ) * JOYSTICK_SENSITIVITY;

        let is_zoom_in = Self::is_zoom_in(keyboard, joystick, mouse_move);
        let is_zoom_out = Self::is_zoom_out(keyboard, joystick, mouse_move);

        let pitch = if mouse_pos_delta.y != 0.0 {
            mouse_pos_delta.y
        } else {
            stick_right_direction.y
        };

        let yaw = if mouse_pos_delta.x != 0.0 {
            mouse_pos_delta.x
        } else {
            stick_right_direction.x
        };

        let zoom = if is_zoom_in || is_zoom_out {
            if mouse_scroll_delta.y != 0.0 {
                -mouse_scroll_delta.y
            } else if is_zoom_in {
                -0.5
            } else {
                0.5
            }
        } else {
            0.0
        };

        CameraControlState {
            _pitch: pitch,
            _yaw: yaw,
            _zoom: zoom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_engine_3d::core::input::{create_keyboard_input_data, create_mouse_input_data, create_mouse_move_data};

    fn create_test_joystick() -> JoystickInputData {
        JoystickInputData {
            _stick_left_direction: Vector2::zeros(),
            _stick_right_direction: Vector2::zeros(),
            _btn_left_stick: ButtonState::None,
            _btn_right_stick: ButtonState::None,
            _btn_left: ButtonState::None,
            _btn_right: ButtonState::None,
            _btn_up: ButtonState::None,
            _btn_down: ButtonState::None,
            _btn_a: ButtonState::None,
            _btn_b: ButtonState::None,
            _btn_x: ButtonState::None,
            _btn_y: ButtonState::None,
            _btn_left_shoulder: ButtonState::None,
            _btn_left_trigger: ButtonState::None,
            _btn_left_trigger_value: 0,
            _btn_right_shoulder: ButtonState::None,
            _btn_right_trigger: ButtonState::None,
            _btn_right_trigger_value: 0,
            _btn_back: ButtonState::None,
            _btn_start: ButtonState::None,
            _btn_guide: ButtonState::None,
            _is_any_button_pressed: false,
            _is_any_button_hold: false,
            _game_controller: None,
        }
    }

    #[test]
    fn test_default_input_bindings_serialization() {
        let bindings = InputBindings::default();
        let json = bindings.to_json().expect("Failed to serialize InputBindings to JSON");
        let deserialized: InputBindings = InputBindings::from_json(&json).expect("Failed to deserialize InputBindings");
        assert_eq!(bindings, deserialized);
    }

    #[test]
    fn test_rebind_custom_key() {
        let mut bindings = InputBindings::default();
        bindings.bind_gameplay_key(GamePlayAction::Jump, KeyCode::KeyJ);
        let binding = bindings._gameplay_bindings.get(&GamePlayAction::Jump).unwrap();
        assert!(binding._key_codes.contains(&KeyCode::KeyJ));

        let mut keyboard = create_keyboard_input_data();
        let joystick = create_test_joystick();
        let mouse = create_mouse_input_data();

        keyboard.set_key_pressed(KeyCode::KeyJ);
        let state = GamePlayInputState::from_input_with_bindings(&bindings, &keyboard, &joystick, &mouse);
        assert!(state._jump);
    }

    #[test]
    fn test_ui_input_confirm_and_cancel() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();

        // Initial state: nothing pressed
        assert!(!UiInput::is_confirm(&keyboard, &joystick));
        assert!(!UiInput::is_cancel(&keyboard, &joystick));

        // Space confirms
        keyboard.set_key_pressed(KeyCode::Space);
        assert!(UiInput::is_confirm(&keyboard, &joystick));
        keyboard.clear_key_pressed();

        // Gamepad A confirms
        joystick._btn_a = ButtonState::Pressed;
        assert!(UiInput::is_confirm(&keyboard, &joystick));
        joystick._btn_a = ButtonState::None;

        // Escape cancels
        keyboard.set_key_pressed(KeyCode::Escape);
        assert!(UiInput::is_cancel(&keyboard, &joystick));
        keyboard.clear_key_pressed();

        // Gamepad B cancels
        joystick._btn_b = ButtonState::Pressed;
        assert!(UiInput::is_cancel(&keyboard, &joystick));
    }

    #[test]
    fn test_ui_input_tabs() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();

        // Tab without shift -> next tab
        keyboard.set_key_pressed(KeyCode::Tab);
        assert!(UiInput::is_next_tab(&keyboard, &joystick));
        assert!(!UiInput::is_prev_tab(&keyboard, &joystick));

        // Tab with shift -> prev tab
        keyboard.set_key_hold(KeyCode::ShiftLeft, true);
        assert!(!UiInput::is_next_tab(&keyboard, &joystick));
        assert!(UiInput::is_prev_tab(&keyboard, &joystick));
        keyboard.clear_key_pressed();
        keyboard.clear_key_hold();

        // Gamepad shoulders
        joystick._btn_right_shoulder = ButtonState::Pressed;
        assert!(UiInput::is_next_tab(&keyboard, &joystick));
        joystick._btn_right_shoulder = ButtonState::None;

        joystick._btn_left_shoulder = ButtonState::Pressed;
        assert!(UiInput::is_prev_tab(&keyboard, &joystick));
    }

    #[test]
    fn test_gameplay_actions_and_movement() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();
        let mut mouse = create_mouse_input_data();

        let state_idle = GamePlayInputState::from_input(&keyboard, &joystick, &mouse);
        assert!(!state_idle.is_moving());
        assert!(!state_idle.is_any_input(false));

        // Move Left via key A
        keyboard.set_key_hold(KeyCode::KeyA, true);
        let state_left = GamePlayInputState::from_input(&keyboard, &joystick, &mouse);
        assert!(state_left._left);
        assert!(state_left.is_moving());
        assert!(state_left.is_any_input(false));
        keyboard.clear_key_hold();

        // Attack via mouse Left button
        mouse.btn_l_pressed(true);
        let state_attack = GamePlayInputState::from_input(&keyboard, &joystick, &mouse);
        assert!(state_attack._attack_or_use_item);
        mouse.btn_l_pressed(false);

        // Jump via gamepad A
        joystick._btn_a = ButtonState::Pressed;
        let state_jump = GamePlayInputState::from_input(&keyboard, &joystick, &mouse);
        assert!(state_jump._jump);
    }

    #[test]
    fn test_fishing_and_camera_input() {
        let keyboard = create_keyboard_input_data();
        let joystick = create_test_joystick();
        let mut mouse = create_mouse_input_data();
        let mut mouse_move = create_mouse_move_data(100, 100);

        // Fishing attack hold
        mouse._btn_l_hold = true;
        let fishing_state = FishingInputState::from_input(&keyboard, &joystick, &mouse);
        assert!(fishing_state._attack_hold);

        // Camera zoom in via scroll delta
        mouse_move._scroll_delta.y = 5;
        assert!(CameraInput::is_zoom_in(&keyboard, &joystick, &mouse_move));
        assert!(!CameraInput::is_zoom_out(&keyboard, &joystick, &mouse_move));

        // Camera zoom out via scroll delta
        mouse_move._scroll_delta.y = -5;
        assert!(!CameraInput::is_zoom_in(&keyboard, &joystick, &mouse_move));
        assert!(CameraInput::is_zoom_out(&keyboard, &joystick, &mouse_move));
    }

    #[test]
    fn test_ui_input_execute_and_digit() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();

        assert!(!UiInput::is_execute(&keyboard, &joystick));
        assert_eq!(UiInput::get_pressed_digit(&keyboard), None);

        keyboard.set_key_pressed(KeyCode::KeyE);
        assert!(UiInput::is_confirm(&keyboard, &joystick));
        assert!(UiInput::is_execute(&keyboard, &joystick));
        keyboard.clear_key_pressed();

        joystick._btn_start = ButtonState::Pressed;
        assert!(UiInput::is_cancel(&keyboard, &joystick));
        joystick._btn_start = ButtonState::None;

        keyboard.set_key_pressed(KeyCode::Digit3);
        assert_eq!(UiInput::get_pressed_digit(&keyboard), Some(3));
        keyboard.clear_key_pressed();
    }

    #[test]
    fn test_cooking_minigame_input_and_drops() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();
        let mut mouse = create_mouse_input_data();

        assert!(!CookingMinigameInput::is_advance(&keyboard, &joystick));
        keyboard.set_key_pressed(KeyCode::KeyE);
        assert!(CookingMinigameInput::is_advance(&keyboard, &joystick));
        keyboard.clear_key_pressed();

        joystick._btn_a = ButtonState::Pressed;
        assert!(CookingMinigameInput::is_advance(&keyboard, &joystick));
        joystick._btn_a = ButtonState::None;

        assert!(!UiInput::is_item_drop(&keyboard, &joystick));
        keyboard.set_key_pressed(KeyCode::KeyF);
        assert!(UiInput::is_item_drop(&keyboard, &joystick));
        keyboard.clear_key_pressed();

        assert!(!UiInput::is_mouse_drop(&mouse));
        mouse.btn_r_pressed(true);
        assert!(UiInput::is_mouse_drop(&mouse));
        assert!(UiInput::is_mouse_sub_action(&mouse));
    }

    #[test]
    fn test_game_input_movement_and_any_input() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();
        let mut mouse = create_mouse_input_data();

        assert!(!GameInput::is_moving(&keyboard, &joystick));
        assert!(!GameInput::is_any_input(&keyboard, &joystick, &mouse));

        keyboard.set_key_hold(KeyCode::KeyW, true);
        assert!(GameInput::is_moving(&keyboard, &joystick));
        assert!(GameInput::is_any_input(&keyboard, &joystick, &mouse));
        keyboard.clear_key_hold();

        joystick._stick_left_direction.x = 4000;
        assert!(GameInput::is_moving(&keyboard, &joystick));
        assert!(GameInput::is_any_input(&keyboard, &joystick, &mouse));
        joystick._stick_left_direction.x = 0;

        mouse.btn_l_pressed(true);
        assert!(GameInput::is_any_input_pressed(&keyboard, &joystick, &mouse));
        assert!(GameInput::is_any_input(&keyboard, &joystick, &mouse));
        mouse.btn_l_pressed(false);

        // Toggle game mode
        keyboard.set_key_pressed(KeyCode::Backquote);
        assert!(GameInput::is_toggle_game_mode(&keyboard, &joystick));
    }

    #[test]
    fn test_game_input_quick_slots_and_any_key() {
        let mut keyboard = create_keyboard_input_data();

        assert!(!GameInput::is_any_key_pressed(&keyboard));
        assert_eq!(GameInput::get_pressed_quick_slot_index(&keyboard), None);

        keyboard.set_key_pressed(KeyCode::Digit1);
        assert!(GameInput::is_any_key_pressed(&keyboard));
        assert_eq!(GameInput::get_pressed_quick_slot_index(&keyboard), Some(0));
        keyboard.clear_key_pressed();

        keyboard.set_key_pressed(KeyCode::Digit0);
        assert_eq!(GameInput::get_pressed_quick_slot_index(&keyboard), Some(9));
    }

    #[test]
    fn test_deploy_input_state() {
        let mut keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();
        let mut mouse = create_mouse_input_data();

        // Idle
        let idle = DeployInputState::from_input(&keyboard, &joystick, &mouse, true);
        assert_eq!(idle._rotate, 0.0);
        assert!(!idle._confirm);
        assert!(!idle._cancel);

        // Rotation: Q -> -1.0, E -> +1.0
        keyboard.set_key_hold(KeyCode::KeyQ, true);
        assert_eq!(DeployInput::get_rotate_input(&keyboard, &joystick), -1.0);
        keyboard.clear_key_hold();

        joystick._btn_right_shoulder = ButtonState::Hold;
        assert_eq!(DeployInput::get_rotate_input(&keyboard, &joystick), 1.0);
        joystick._btn_right_shoulder = ButtonState::None;

        // Keyboard mode confirm/cancel
        keyboard.set_key_pressed(KeyCode::Space);
        assert!(DeployInput::is_confirm(&keyboard, &joystick, &mouse, true));
        assert!(!DeployInput::is_confirm(&keyboard, &joystick, &mouse, false));
        keyboard.clear_key_pressed();

        mouse.btn_r_pressed(true);
        assert!(DeployInput::is_cancel(&keyboard, &joystick, &mouse, true));
        mouse.btn_r_pressed(false);

        // Gamepad mode confirm/cancel
        joystick._btn_a = ButtonState::Pressed;
        assert!(DeployInput::is_confirm(&keyboard, &joystick, &mouse, false));
        joystick._btn_a = ButtonState::None;

        joystick._btn_b = ButtonState::Pressed;
        assert!(DeployInput::is_cancel(&keyboard, &joystick, &mouse, false));
    }

    #[test]
    fn test_camera_control_state() {
        let keyboard = create_keyboard_input_data();
        let mut joystick = create_test_joystick();
        let mut mouse_move = create_mouse_move_data(100, 100);

        mouse_move._mouse_pos_delta = Vector2::new(10, -20);
        mouse_move._scroll_delta.y = 2;

        let control = CameraInput::get_camera_control_state(&keyboard, &joystick, &mouse_move);
        assert_eq!(control._yaw, 10.0 * CameraInput::MOUSE_SENSITIVITY);
        assert_eq!(control._pitch, -20.0 * CameraInput::MOUSE_SENSITIVITY);
        assert_eq!(control._zoom, -2.0);

        mouse_move._mouse_pos_delta = Vector2::zeros();
        mouse_move._scroll_delta.y = 0;
        joystick._stick_right_direction = Vector2::new(5000, 3000);

        let stick_control = CameraInput::get_camera_control_state(&keyboard, &joystick, &mouse_move);
        assert_eq!(stick_control._yaw, 5000.0 * JOYSTICK_SENSITIVITY);
        assert_eq!(stick_control._pitch, 3000.0 * JOYSTICK_SENSITIVITY);
        assert_eq!(stick_control._zoom, 0.0);
    }
}
