use crate::game_module::game_constants::AUDIO_SELECT_ITEM;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, PIVOT_TOP_LEFT, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

pub const DEFAULT_COMBO_WIDTH: f32 = 250.0;
pub const DEFAULT_COMBO_HEIGHT: f32 = 34.0;
pub const DEFAULT_ITEM_HEIGHT: f32 = 32.0;

const COLOR_HEADER_BG: u32 = get_color32(240, 222, 192, 240);
const COLOR_HEADER_BG_HOVER: u32 = get_color32(250, 235, 210, 255);
const COLOR_HEADER_BORDER: u32 = get_color32(185, 130, 75, 255);
const COLOR_HEADER_TEXT: u32 = get_color32(50, 30, 10, 255);

const COLOR_MENU_BG: u32 = get_color32(248, 238, 220, 255);
const COLOR_MENU_BORDER: u32 = get_color32(170, 110, 60, 255);

const COLOR_ITEM_BG_NORMAL: u32 = get_color32(240, 225, 200, 220);
const COLOR_ITEM_BG_HOVER: u32 = get_color32(225, 175, 120, 250);
const COLOR_ITEM_BG_SELECTED: u32 = get_color32(200, 135, 75, 255);
const COLOR_ITEM_TEXT_NORMAL: u32 = get_color32(60, 35, 15, 255);
const COLOR_ITEM_TEXT_SELECTED: u32 = get_color32(255, 255, 255, 255);

const COLOR_OVERLAY_BG: u32 = get_color32(0, 0, 0, 40);

pub type CallbackSelectionChanged<'a> = fn(*const c_void, usize);

pub struct DropDownItemUserData<'a> {
    pub _combo_ptr: *const DropDownComboBoxWidget<'a>,
    pub _index: usize,
}

pub struct DropDownComboBoxWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _overlay_root: *const WidgetDefault<'a>,
    pub _header_widget: Rc<WidgetDefault<'a>>,
    pub _overlay_layer: Rc<WidgetDefault<'a>>,
    pub _menu_container: Rc<WidgetDefault<'a>>,
    pub _item_widgets: Vec<Rc<WidgetDefault<'a>>>,
    pub _item_user_data: Vec<Box<DropDownItemUserData<'a>>>,
    pub _items: Vec<String>,
    pub _selected_index: usize,
    pub _hovered_index: usize,
    pub _is_open: bool,
    pub _combo_width: f32,
    pub _combo_height: f32,
    pub _item_height: f32,
    pub _callback_selection_changed: Option<(CallbackSelectionChanged<'a>, *const c_void)>,
}

impl<'a> DropDownComboBoxWidget<'a> {
    pub fn callback_header_click(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let combo = ptr_as_mut(ui_component.get_user_data() as *const DropDownComboBoxWidget<'a>);
        combo.toggle_dropdown();
        true
    }

    pub fn callback_header_over(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let ui = ptr_as_mut(ui_component);
        ui.set_color(COLOR_HEADER_BG_HOVER);
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_header_out(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let ui = ptr_as_mut(ui_component);
        ui.set_color(COLOR_HEADER_BG);
        true
    }

    pub fn callback_overlay_click(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let combo = ptr_as_mut(ui_component.get_user_data() as *const DropDownComboBoxWidget<'a>);
        combo.close_dropdown();
        true
    }

    pub fn callback_item_click(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const DropDownItemUserData<'a>);
        let combo = ptr_as_mut(user_data._combo_ptr);
        combo.set_selected_index(user_data._index);
        combo.close_dropdown();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_item_over(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const DropDownItemUserData<'a>);
        let combo = ptr_as_mut(user_data._combo_ptr);
        combo._hovered_index = user_data._index;
        combo.update_item_visuals();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn create_drop_down_combo_box(
        parent_widget: &mut WidgetDefault<'a>,
        overlay_root: *const WidgetDefault<'a>,
        widget_name: &str,
        items: Vec<String>,
        initial_index: usize,
        width: f32,
        height: f32,
    ) -> Box<DropDownComboBoxWidget<'a>> {
        let header_name = format!("{}_header", widget_name);
        let header_widget = UIManager::create_widget(&header_name, UIWidgetTypes::Default);
        {
            let header_ui = ptr_as_mut(header_widget.as_ref()).get_ui_component_mut();
            header_ui.set_halign(HorizontalAlign::CENTER);
            header_ui.set_valign(VerticalAlign::CENTER);
            header_ui.set_size(width, height);
            header_ui.set_margin(4.0);
            header_ui.set_round(4.0);
            header_ui.set_border(1.5);
            header_ui.set_border_color(COLOR_HEADER_BORDER);
            header_ui.set_color(COLOR_HEADER_BG);
            header_ui.set_font_size(18.0);
            header_ui.set_font_color(COLOR_HEADER_TEXT);
            header_ui.set_touchable(true);
            header_ui.set_callback_touch_down(Some(Box::new(Self::callback_header_click)));
            header_ui.set_callback_touch_over(Some(Box::new(Self::callback_header_over)));
            header_ui.set_callback_touch_out(Some(Box::new(Self::callback_header_out)));
        }
        parent_widget.add_widget(&header_widget);

        // Overlay layer for fullscreen backdrop click detection and menu container
        let overlay_name = format!("{}_overlay", widget_name);
        let overlay_layer = UIManager::create_widget(&overlay_name, UIWidgetTypes::Default);
        {
            let overlay_ui = ptr_as_mut(overlay_layer.as_ref()).get_ui_component_mut();
            overlay_ui.set_layout_type(UILayoutType::FloatLayout);
            overlay_ui.set_pivot_preset(PIVOT_CENTER);
            overlay_ui.set_pos_hint(Some(0.5), Some(0.5));
            overlay_ui.set_size_hint_x(Some(1.0));
            overlay_ui.set_size_hint_y(Some(1.0));
            overlay_ui.set_color(COLOR_OVERLAY_BG);
            overlay_ui.set_touchable(true);
            overlay_ui.set_callback_touch_down(Some(Box::new(Self::callback_overlay_click)));
            overlay_ui.set_ignore_parent_renderable_area(true);
        }

        // Dropdown menu container
        let menu_name = format!("{}_menu", widget_name);
        let menu_container = UIManager::create_widget(&menu_name, UIWidgetTypes::Default);
        {
            let menu_ui = ptr_as_mut(menu_container.as_ref()).get_ui_component_mut();
            menu_ui.set_layout_type(UILayoutType::BoxLayout);
            menu_ui.set_layout_orientation(Orientation::VERTICAL);
            menu_ui.set_halign(HorizontalAlign::CENTER);
            menu_ui.set_valign(VerticalAlign::TOP);
            menu_ui.set_pivot_preset(PIVOT_TOP_LEFT);
            menu_ui.set_padding(4.0);
            menu_ui.set_round(6.0);
            menu_ui.set_border(2.0);
            menu_ui.set_border_color(COLOR_MENU_BORDER);
            menu_ui.set_color(COLOR_MENU_BG);
            menu_ui.set_touchable(true);
            menu_ui.set_ignore_parent_renderable_area(true);
        }
        ptr_as_mut(overlay_layer.as_ref()).add_widget(&menu_container);

        let selected = if initial_index < items.len() { initial_index } else { 0 };

        let mut widget = Box::new(DropDownComboBoxWidget {
            _parent_widget: parent_widget,
            _overlay_root: overlay_root,
            _header_widget: header_widget,
            _overlay_layer: overlay_layer,
            _menu_container: menu_container,
            _item_widgets: Vec::new(),
            _item_user_data: Vec::new(),
            _items: items,
            _selected_index: selected,
            _hovered_index: selected,
            _is_open: false,
            _combo_width: width,
            _combo_height: height,
            _item_height: DEFAULT_ITEM_HEIGHT,
            _callback_selection_changed: None,
        });

        let widget_ptr = widget.as_ref() as *const DropDownComboBoxWidget<'a>;
        ptr_as_mut(widget._header_widget.as_ref()).get_ui_component_mut().set_user_data(widget_ptr as *const c_void);
        ptr_as_mut(widget._overlay_layer.as_ref()).get_ui_component_mut().set_user_data(widget_ptr as *const c_void);

        widget.rebuild_items();
        widget.update_header_text();
        widget
    }

    pub fn create_from_enum<E: Copy + PartialEq>(
        parent_widget: &mut WidgetDefault<'a>,
        overlay_root: *const WidgetDefault<'a>,
        widget_name: &str,
        enum_variants: &[E],
        display_names: &[&str],
        initial_value: E,
        width: f32,
        height: f32,
    ) -> Box<DropDownComboBoxWidget<'a>> {
        let initial_index = enum_variants.iter().position(|v| *v == initial_value).unwrap_or(0);
        Self::create_drop_down_combo_box(
            parent_widget,
            overlay_root,
            widget_name,
            display_names.iter().map(|s| s.to_string()).collect(),
            initial_index,
            width,
            height,
        )
    }

    pub fn set_callback_selection_changed(&mut self, callback: Option<(CallbackSelectionChanged<'a>, *const c_void)>) {
        self._callback_selection_changed = callback;
    }

    pub fn set_items(&mut self, items: Vec<String>, selected_index: usize) {
        self._items = items;
        self._selected_index = if selected_index < self._items.len() {
            selected_index
        } else {
            0
        };
        self._hovered_index = self._selected_index;
        self.rebuild_items();
        self.update_header_text();
    }

    pub fn rebuild_items(&mut self) {
        let menu_mut = ptr_as_mut(self._menu_container.as_ref());
        for item_widget in self._item_widgets.iter() {
            menu_mut.remove_widget(item_widget.as_ref());
        }
        self._item_widgets.clear();
        self._item_user_data.clear();

        let widget_ptr = self as *const DropDownComboBoxWidget<'a>;
        let total_items = self._items.len();
        let menu_width = self._combo_width;
        let menu_height = (total_items as f32 * (self._item_height + 4.0)) + 8.0;

        menu_mut.get_ui_component_mut().set_size(menu_width, menu_height);

        for (idx, item_text) in self._items.iter().enumerate() {
            let item_user_data = Box::new(DropDownItemUserData {
                _combo_ptr: widget_ptr,
                _index: idx,
            });

            let item_name = format!("combo_item_{}", idx);
            let item_widget = UIManager::create_widget(&item_name, UIWidgetTypes::Default);
            let item_ui = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();
            item_ui.set_halign(HorizontalAlign::CENTER);
            item_ui.set_valign(VerticalAlign::CENTER);
            item_ui.set_size(menu_width - 8.0, self._item_height);
            item_ui.set_margin(2.0);
            item_ui.set_round(3.0);
            item_ui.set_font_size(16.0);
            item_ui.set_text(item_text);
            item_ui.set_touchable(true);
            item_ui.set_callback_touch_down(Some(Box::new(Self::callback_item_click)));
            item_ui.set_callback_touch_over(Some(Box::new(Self::callback_item_over)));
            item_ui.set_user_data(item_user_data.as_ref() as *const DropDownItemUserData<'a> as *const c_void);

            menu_mut.add_widget(&item_widget);
            self._item_widgets.push(item_widget);
            self._item_user_data.push(item_user_data);
        }

        self.update_item_visuals();
    }

    pub fn update_header_text(&mut self) {
        let text = if let Some(selected) = self._items.get(self._selected_index) {
            let arrow = if self._is_open { "▲" } else { "▼" };
            format!("{}  {}", selected, arrow)
        } else {
            "None ▼".to_string()
        };
        let header_ui = ptr_as_mut(self._header_widget.as_ref()).get_ui_component_mut();
        header_ui.set_text(&text);
    }

    pub fn update_item_visuals(&mut self) {
        for (i, item_widget) in self._item_widgets.iter_mut().enumerate() {
            let is_selected = i == self._selected_index;
            let is_hovered = i == self._hovered_index;
            let item_ui = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();

            let prefix = if is_selected { "✓ " } else { "   " };
            if let Some(text) = self._items.get(i) {
                item_ui.set_text(&format!("{}{}", prefix, text));
            }

            if is_selected {
                item_ui.set_color(COLOR_ITEM_BG_SELECTED);
                item_ui.set_font_color(COLOR_ITEM_TEXT_SELECTED);
            } else if is_hovered {
                item_ui.set_color(COLOR_ITEM_BG_HOVER);
                item_ui.set_font_color(COLOR_ITEM_TEXT_NORMAL);
            } else {
                item_ui.set_color(COLOR_ITEM_BG_NORMAL);
                item_ui.set_font_color(COLOR_ITEM_TEXT_NORMAL);
            }
        }
    }

    pub fn is_opened(&self) -> bool {
        self._is_open
    }

    pub fn open_dropdown(&mut self) {
        if !self._is_open {
            self._is_open = true;
            self._hovered_index = self._selected_index;
            self.update_header_text();
            self.update_item_visuals();

            // Calculate menu position relative to header button
            let header_render_area = *ptr_as_ref(self._header_widget.as_ref()).get_ui_component().get_render_area();

            let menu_mut = ptr_as_mut(self._menu_container.as_ref());
            let menu_ui = menu_mut.get_ui_component_mut();
            if header_render_area.z > header_render_area.x && header_render_area.w > header_render_area.y {
                menu_ui.set_pos(header_render_area.x, header_render_area.w + 4.0);
                menu_ui.set_pos_hint(None, None);
            } else {
                menu_ui.set_pos_hint(Some(0.5), Some(0.5));
            }

            let overlay_root_mut = ptr_as_mut(self._overlay_root);
            overlay_root_mut.add_widget(&self._overlay_layer);
        }
    }

    pub fn close_dropdown(&mut self) {
        if self._is_open {
            self._is_open = false;
            let overlay_root_mut = ptr_as_mut(self._overlay_root);
            overlay_root_mut.remove_widget(self._overlay_layer.as_ref());
            self.update_header_text();
        }
    }

    pub fn toggle_dropdown(&mut self) {
        if self._is_open {
            self.close_dropdown();
        } else {
            self.open_dropdown();
        }
    }

    pub fn get_selected_index(&self) -> usize {
        self._selected_index
    }

    pub fn set_selected_index(&mut self, index: usize) {
        if index < self._items.len() && self._selected_index != index {
            self._selected_index = index;
            self._hovered_index = index;
            self.update_header_text();
            self.update_item_visuals();

            if let Some((callback, user_data)) = self._callback_selection_changed {
                callback(user_data, index);
            }
        }
    }

    pub fn next_item(&mut self) {
        if !self._items.is_empty() {
            let next = (self._selected_index + 1) % self._items.len();
            self.set_selected_index(next);
        }
    }

    pub fn prev_item(&mut self) {
        if !self._items.is_empty() {
            let prev = if self._selected_index == 0 {
                self._items.len() - 1
            } else {
                self._selected_index - 1
            };
            self.set_selected_index(prev);
        }
    }

    pub fn move_hover_next(&mut self) {
        if !self._items.is_empty() {
            self._hovered_index = (self._hovered_index + 1) % self._items.len();
            self.update_item_visuals();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        }
    }

    pub fn move_hover_prev(&mut self) {
        if !self._items.is_empty() {
            self._hovered_index = if self._hovered_index == 0 {
                self._items.len() - 1
            } else {
                self._hovered_index - 1
            };
            self.update_item_visuals();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        }
    }

    pub fn confirm_hovered(&mut self) {
        if self._hovered_index < self._items.len() {
            self.set_selected_index(self._hovered_index);
            self.close_dropdown();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        }
    }

    pub fn get_selected_text(&self) -> Option<&str> {
        self._items.get(self._selected_index).map(|s| s.as_str())
    }
}
