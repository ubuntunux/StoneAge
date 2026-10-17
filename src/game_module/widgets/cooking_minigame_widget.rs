use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_service_locator::get_game_ui_manager_mut;
use crate::game_module::widgets::cooking_widget::{COOKING_RECIPES, CookingWidget};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub const COOKING_MINIGAME_WIDTH: f32 = 380.0;
pub const COOKING_MINIGAME_HEIGHT: f32 = 250.0;
pub const PROGRESS_AUTO_FILL_SPEED: f32 = 0.5;

pub struct CookingMinigameWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _container: Rc<WidgetDefault<'a>>,
    pub _title_lbl: Rc<WidgetDefault<'a>>,
    pub _step_lbl: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _name_lbl: Rc<WidgetDefault<'a>>,
    pub _progress_bg: Rc<WidgetDefault<'a>>,
    pub _progress_fill: Rc<WidgetDefault<'a>>,
    pub _next_btn: Rc<WidgetDefault<'a>>,
    pub _is_active: bool,
    pub _recipe_index: usize,
    pub _current_step: usize,
    pub _step_progress: f32,
    pub _last_space_pressed: bool,
    pub _last_enter_pressed: bool,
    pub _last_e_pressed: bool,
    pub _last_joy_a_pressed: bool,
}

impl<'a> CookingMinigameWidget<'a> {
    pub fn create_cooking_minigame_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<CookingMinigameWidget<'a>> {
        let container = UIManager::create_widget("cooking_minigame_container", UIWidgetTypes::Default);
        let container_mut = ptr_as_mut(container.as_ref());
        {
            let ui = container_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::VERTICAL);
            ui.set_pivot_preset(PIVOT_CENTER);
            ui.set_pos_hint(Some(0.5), Some(0.5));
            ui.set_size(COOKING_MINIGAME_WIDTH, COOKING_MINIGAME_HEIGHT);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::TOP);
            ui.set_padding(14.0);
            ui.set_round(14.0);
            ui.set_color(get_color32(35, 24, 18, 245));
            ui.set_border_color(get_color32(220, 150, 60, 255));
            ui.set_border(3.0);
            ui.set_visible(false);
            ui.set_enable(true);
        }
        parent_widget.add_widget(&container);

        // Header Title (e.g. Cooking: Roast Meat)
        let title_lbl = UIManager::create_widget("cooking_minigame_title", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(title_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(28.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(20.0);
            ui.set_font_color(get_color32(255, 215, 0, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("Cooking in Progress");
        }
        container_mut.add_widget(&title_lbl);

        // Step Counter Label (e.g. Ingredient 1 of 2)
        let step_lbl = UIManager::create_widget("cooking_minigame_step", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(step_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(22.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(15.0);
            ui.set_font_color(get_color32(210, 190, 170, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("Step 1/1");
        }
        container_mut.add_widget(&step_lbl);

        // Ingredient Row (Horizontal: Icon + Name)
        let ing_row = UIManager::create_widget("cooking_minigame_ing_row", UIWidgetTypes::Default);
        let ing_row_mut = ptr_as_mut(ing_row.as_ref());
        {
            let ui = ing_row_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(54.0);
            ui.set_margin_top(4.0);
            ui.set_margin_bottom(6.0);
            ui.set_color(get_color32(0, 0, 0, 0));
        }
        container_mut.add_widget(&ing_row);

        // Ingredient Icon
        let icon = UIManager::create_widget("cooking_minigame_icon", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
            ui.set_size(48.0, 48.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(12.0);
            ui.set_color(get_color32(255, 255, 255, 255));
        }
        ing_row_mut.add_widget(&icon);

        // Ingredient Name
        let name_lbl = UIManager::create_widget("cooking_minigame_name", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(0.65));
            ui.set_size_y(32.0);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(18.0);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("Ingredient Name");
        }
        ing_row_mut.add_widget(&name_lbl);

        // Progress Bar Background
        let progress_bg = UIManager::create_widget("cooking_minigame_pbar_bg", UIWidgetTypes::Default);
        let progress_bg_mut = ptr_as_mut(progress_bg.as_ref());
        {
            let ui = progress_bg_mut.get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(16.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_color(get_color32(20, 14, 10, 230));
            ui.set_border_color(get_color32(100, 70, 50, 200));
            ui.set_border(1.5);
            ui.set_round(8.0);
            ui.set_margin_top(2.0);
            ui.set_margin_bottom(10.0);
        }
        container_mut.add_widget(&progress_bg);

        // Progress Bar Fill
        let progress_fill = UIManager::create_widget("cooking_minigame_pbar_fill", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(progress_fill.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(0.0));
            ui.set_size_hint_y(Some(1.0));
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_color(get_color32(235, 135, 45, 255));
            ui.set_round(8.0);
        }
        progress_bg_mut.add_widget(&progress_fill);

        // Next Button
        let next_btn = UIManager::create_widget("cooking_minigame_next_btn", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(next_btn.as_ref()).get_ui_component_mut();
            ui.set_size(150.0, 36.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_color(get_color32(118, 168, 68, 240));
            ui.set_border_color(get_color32(72, 115, 38, 255));
            ui.set_border(2.0);
            ui.set_round(8.0);
            ui.set_text("Add [Space]");
            ui.set_font_size(16.0);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_touchable(true);
            ui.set_callback_touch_down(Some(Box::new(Self::callback_next_btn)));
        }
        container_mut.add_widget(&next_btn);

        let widget = Box::new(CookingMinigameWidget {
            _parent_widget: parent_widget as *const WidgetDefault<'a>,
            _container: container,
            _title_lbl: title_lbl,
            _step_lbl: step_lbl,
            _icon: icon,
            _name_lbl: name_lbl,
            _progress_bg: progress_bg,
            _progress_fill: progress_fill,
            _next_btn: next_btn,
            _is_active: false,
            _recipe_index: 0,
            _current_step: 0,
            _step_progress: 0.0,
            _last_space_pressed: false,
            _last_enter_pressed: false,
            _last_e_pressed: false,
            _last_joy_a_pressed: false,
        });

        let self_ptr = widget.as_ref() as *const CookingMinigameWidget<'a> as *const c_void;
        ptr_as_mut(widget._next_btn.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

        widget
    }

    fn callback_next_btn(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let self_ptr = ui.get_user_data() as *mut CookingMinigameWidget<'a>;
        if !self_ptr.is_null() {
            ptr_as_mut(self_ptr).advance_step();
        }
        true
    }

    pub fn is_active(&self) -> bool {
        self._is_active
    }

    pub fn start_minigame(&mut self, recipe_index: usize) {
        if recipe_index >= COOKING_RECIPES.len() {
            return;
        }

        self._recipe_index = recipe_index;
        self._current_step = 0;
        self._step_progress = 0.0;
        self._is_active = true;

        let recipe = &COOKING_RECIPES[recipe_index];
        let recipe_name = CookingWidget::get_item_name_from_resource(recipe.item_code());
        ptr_as_mut(self._title_lbl.as_ref()).get_ui_component_mut().set_text(&format!("Cooking: {}", recipe_name));

        let container_ui = ptr_as_mut(self._container.as_ref()).get_ui_component_mut();
        container_ui.set_visible(true);

        self.update_step_display();
    }

    pub fn close_minigame(&mut self) {
        self._is_active = false;
        let container_ui = ptr_as_mut(self._container.as_ref()).get_ui_component_mut();
        container_ui.set_visible(false);
    }

    pub fn update_step_display(&mut self) {
        if self._recipe_index >= COOKING_RECIPES.len() {
            return;
        }

        let recipe = &COOKING_RECIPES[self._recipe_index];
        let total_steps = recipe._ingredients.len();

        if self._current_step < total_steps {
            let req = &recipe._ingredients[self._current_step];
            let ing_name = CookingWidget::get_item_name_from_resource(req.item_code());

            ptr_as_mut(self._step_lbl.as_ref()).get_ui_component_mut().set_text(&format!(
                "Ingredient {} / {}",
                self._current_step + 1,
                total_steps
            ));

            CookingWidget::setup_item_icon(&self._icon, req.item_code());

            ptr_as_mut(self._name_lbl.as_ref())
                .get_ui_component_mut()
                .set_text(&format!("{} x{}", ing_name, req._count));

            let btn_text = if self._current_step + 1 == total_steps {
                "Finish [Space]"
            } else {
                "Next [Space]"
            };
            ptr_as_mut(self._next_btn.as_ref()).get_ui_component_mut().set_text(btn_text);

            let fill_ratio = self._step_progress.clamp(0.0, 1.0);
            ptr_as_mut(self._progress_fill.as_ref()).get_ui_component_mut().set_size_hint_x(Some(fill_ratio));
        }
    }

    pub fn advance_step(&mut self) {
        if !self._is_active || self._recipe_index >= COOKING_RECIPES.len() {
            return;
        }

        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);

        let recipe = &COOKING_RECIPES[self._recipe_index];
        self._current_step += 1;
        self._step_progress = 0.0;

        if self._current_step >= recipe._ingredients.len() {
            self.complete_cooking();
        } else {
            self.update_step_display();
        }
    }

    fn complete_cooking(&mut self) {
        let recipe_index = self._recipe_index;
        let recipe = &COOKING_RECIPES[recipe_index];
        let item_code = recipe.item_code();

        CookingWidget::try_cook_recipe(recipe_index);
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        get_game_ui_manager_mut().show_cooking_popup(item_code);
        self.close_minigame();
    }

    pub fn update(&mut self, delta_time: f32, keyboard: &KeyboardInputData, joystick: &JoystickInputData) {
        if !self._is_active {
            return;
        }

        // Auto-fill progress over time
        self._step_progress = (self._step_progress + delta_time * PROGRESS_AUTO_FILL_SPEED).min(1.0);
        let fill_ratio = self._step_progress.clamp(0.0, 1.0);
        ptr_as_mut(self._progress_fill.as_ref()).get_ui_component_mut().set_size_hint_x(Some(fill_ratio));

        // Key presses (Trigger on press edge)
        let space_pressed = keyboard.get_key_pressed(KeyCode::Space);
        let enter_pressed = keyboard.get_key_pressed(KeyCode::Enter);
        let e_pressed = keyboard.get_key_pressed(KeyCode::KeyE);
        let joy_a_pressed = joystick._btn_a == ButtonState::Pressed;

        let is_button_advance = (space_pressed && !self._last_space_pressed)
            || (enter_pressed && !self._last_enter_pressed)
            || (e_pressed && !self._last_e_pressed)
            || (joy_a_pressed && !self._last_joy_a_pressed);

        self._last_space_pressed = space_pressed;
        self._last_enter_pressed = enter_pressed;
        self._last_e_pressed = e_pressed;
        self._last_joy_a_pressed = joy_a_pressed;

        let is_progress_full = 1.0 <= self._step_progress;

        if is_button_advance || is_progress_full {
            self.advance_step();
        }
    }
}
