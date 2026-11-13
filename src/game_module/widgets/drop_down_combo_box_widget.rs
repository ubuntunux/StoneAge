use crate::game_module::game_constants::AUDIO_SELECT_ITEM;
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use nalgebra::{Vector2, Vector4};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, PIVOT_TOP_LEFT, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault, get_global_dpi_scale,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

pub const DEFAULT_COMBO_WIDTH: f32 = 250.0;
pub const DEFAULT_COMBO_HEIGHT: f32 = 34.0;
pub const DEFAULT_ITEM_HEIGHT: f32 = 32.0;

const MAX_VISIBLE_DROPDOWN_ITEMS: usize = 8;
const DROPDOWN_HEADER_MARGIN: f32 = 4.0;
const DROPDOWN_HEADER_ROUND: f32 = 4.0;
const DROPDOWN_HEADER_BORDER: f32 = 1.5;
const DROPDOWN_HEADER_FONT_SIZE: f32 = 20.0;
const DROPDOWN_MENU_OFFSET_Y: f32 = 4.0;
const DROPDOWN_MENU_PADDING: f32 = 4.0;
const DROPDOWN_MENU_ROUND: f32 = 6.0;
const DROPDOWN_MENU_BORDER: f32 = 2.0;
const DROPDOWN_ITEM_MARGIN: f32 = 2.0;
const DROPDOWN_ITEM_ROUND: f32 = 3.0;
const DROPDOWN_ITEM_FONT_SIZE: f32 = 18.0;
const DROPDOWN_SCREEN_MARGIN: f32 = 8.0;
const MIN_DROPDOWN_DPI_SCALE: f32 = 0.1;

const COLOR_HEADER_BG: u32 = get_color32(240, 222, 192, 240);
const COLOR_HEADER_BG_HOVER: u32 = get_color32(250, 235, 210, 255);
const COLOR_HEADER_BORDER: u32 = get_color32(185, 130, 75, 255);
const COLOR_HEADER_TEXT: u32 = get_color32(50, 30, 10, 255);
const COLOR_HEADER_BG_DISABLED: u32 = get_color32(215, 205, 190, 180);
const COLOR_HEADER_BORDER_DISABLED: u32 = get_color32(170, 155, 135, 180);
const COLOR_HEADER_TEXT_DISABLED: u32 = get_color32(130, 115, 100, 180);

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
    pub _is_enabled: bool,
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
        if !combo._is_enabled {
            return false;
        }
        combo.toggle_dropdown();
        true
    }

    pub fn callback_header_over(
        ui_component: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let combo = ptr_as_ref(ui_component.get_user_data() as *const DropDownComboBoxWidget<'a>);
        if !combo._is_enabled {
            return false;
        }
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
        let combo = ptr_as_ref(ui_component.get_user_data() as *const DropDownComboBoxWidget<'a>);
        if !combo._is_enabled {
            return false;
        }
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
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const DropDownItemUserData<'a>);
        let combo = ptr_as_mut(user_data._combo_ptr);
        if combo._hovered_index != user_data._index {
            combo._hovered_index = user_data._index;
            combo.update_item_visuals();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        }
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
            header_ui.set_expandable(false);
            header_ui.set_size(width, height);
            header_ui.set_margin(DROPDOWN_HEADER_MARGIN);
            header_ui.set_round(DROPDOWN_HEADER_ROUND);
            header_ui.set_border(DROPDOWN_HEADER_BORDER);
            header_ui.set_border_color(COLOR_HEADER_BORDER);
            header_ui.set_color(COLOR_HEADER_BG);
            header_ui.set_font_size(DROPDOWN_HEADER_FONT_SIZE);
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
            overlay_ui.set_renderable(false);
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
            menu_ui.set_expandable(false);
            menu_ui.set_enable_renderable_area(true);
            menu_ui.set_padding(DROPDOWN_MENU_PADDING);
            menu_ui.set_round(DROPDOWN_MENU_ROUND);
            menu_ui.set_border(DROPDOWN_MENU_BORDER);
            menu_ui.set_border_color(COLOR_MENU_BORDER);
            menu_ui.set_color(COLOR_MENU_BG);
            menu_ui.set_touchable(false);
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
            _is_enabled: true,
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

    fn visible_item_count(&self) -> usize {
        self._items.len().min(MAX_VISIBLE_DROPDOWN_ITEMS).max(1)
    }

    fn menu_logical_total_size(&self) -> Vector2<f32> {
        let row_logical_height = self._item_height + DROPDOWN_ITEM_MARGIN * 2.0;
        let visible_contents_height = self.visible_item_count() as f32 * row_logical_height;
        Vector2::new(self._combo_width, visible_contents_height + DROPDOWN_MENU_PADDING * 2.0)
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
        let visible_items = self.visible_item_count();
        let row_logical_height = self._item_height + DROPDOWN_ITEM_MARGIN * 2.0;
        let menu_contents_width = (self._combo_width - DROPDOWN_MENU_PADDING * 2.0).max(0.0);
        let menu_contents_height = visible_items as f32 * row_logical_height;

        {
            let menu_ui = menu_mut.get_ui_component_mut();
            menu_ui.set_size(menu_contents_width, menu_contents_height);
            menu_ui.set_scroll_y(total_items > MAX_VISIBLE_DROPDOWN_ITEMS);
            menu_ui.set_scroll_pos_y(0.0);
        }

        let item_width = (menu_contents_width - DROPDOWN_ITEM_MARGIN * 2.0).max(0.0);
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
            item_ui.set_expandable(false);
            item_ui.set_size(item_width, self._item_height);
            item_ui.set_margin(DROPDOWN_ITEM_MARGIN);
            item_ui.set_round(DROPDOWN_ITEM_ROUND);
            item_ui.set_font_size(DROPDOWN_ITEM_FONT_SIZE);
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
            selected.clone()
        } else {
            "None".to_string()
        };
        let header_ui = ptr_as_mut(self._header_widget.as_ref()).get_ui_component_mut();
        header_ui.set_text(&text);
    }

    pub fn is_enabled(&self) -> bool {
        self._is_enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        if !enabled {
            self.close_dropdown();
        }
        self._is_enabled = enabled;
        let header_ui = ptr_as_mut(self._header_widget.as_ref()).get_ui_component_mut();
        header_ui.set_touchable(enabled);
        header_ui.set_color(if enabled {
            COLOR_HEADER_BG
        } else {
            COLOR_HEADER_BG_DISABLED
        });
        header_ui.set_border_color(if enabled {
            COLOR_HEADER_BORDER
        } else {
            COLOR_HEADER_BORDER_DISABLED
        });
        header_ui.set_font_color(if enabled {
            COLOR_HEADER_TEXT
        } else {
            COLOR_HEADER_TEXT_DISABLED
        });
        self.update_header_text();
    }

    pub fn update_item_visuals(&mut self) {
        for (i, item_widget) in self._item_widgets.iter_mut().enumerate() {
            let is_selected = i == self._selected_index;
            let is_hovered = i == self._hovered_index;
            let item_ui = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();

            if let Some(text) = self._items.get(i) {
                item_ui.set_text(&text);
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

    fn update_dropdown_position(&mut self) {
        let dpi_scale = get_global_dpi_scale().max(MIN_DROPDOWN_DPI_SCALE);
        let header_render_area = *ptr_as_ref(self._header_widget.as_ref()).get_ui_component().get_render_area();
        let overlay_contents_area = *ptr_as_ref(self._overlay_root).get_ui_component().get_contents_area();
        let menu_logical_size = self.menu_logical_total_size();

        let menu_mut = ptr_as_mut(self._menu_container.as_ref());
        let menu_ui = menu_mut.get_ui_component_mut();

        if header_render_area.z > header_render_area.x && header_render_area.w > header_render_area.y {
            let logical_pos = compute_dropdown_menu_logical_pos(
                &header_render_area,
                &overlay_contents_area,
                &menu_logical_size,
                dpi_scale,
            );
            menu_ui.set_pos(logical_pos.x, logical_pos.y);
            menu_ui.set_pos_hint(None, None);
        } else {
            menu_ui.set_pos(0.0, 0.0);
            menu_ui.set_pos_hint(Some(0.5), Some(0.5));
        }

        let initial_scroll_y = if self._selected_index >= MAX_VISIBLE_DROPDOWN_ITEMS {
            let first_visible_idx = self._selected_index + 1 - MAX_VISIBLE_DROPDOWN_ITEMS;
            let row_phys_height = (self._item_height + DROPDOWN_ITEM_MARGIN * 2.0) * dpi_scale;
            first_visible_idx as f32 * row_phys_height
        } else {
            0.0
        };
        menu_ui.set_scroll_pos_y(initial_scroll_y);
    }

    fn scroll_hovered_item_into_view(&mut self) {
        if let Some(target_item) = self._item_widgets.get(self._hovered_index) {
            let menu_ui = ptr_as_mut(self._menu_container.as_ref()).get_ui_component_mut();
            let target_ui = ptr_as_ref(target_item.as_ref()).get_ui_component();
            menu_ui.scroll_into_view(target_ui);
        }
    }

    pub fn open_dropdown(&mut self) {
        if !self._is_enabled {
            return;
        }
        if !self._is_open {
            self._is_open = true;
            self._hovered_index = self._selected_index;
            self.update_header_text();
            self.update_item_visuals();
            self.update_dropdown_position();

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
        if !self._is_enabled {
            return;
        }
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
            self.scroll_hovered_item_into_view();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            if let Some(target_item) = self._item_widgets.get(self._hovered_index) {
                move_mouse_to_ui_component(target_item.get_ui_component());
            }
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
            self.scroll_hovered_item_into_view();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            if let Some(target_item) = self._item_widgets.get(self._hovered_index) {
                move_mouse_to_ui_component(target_item.get_ui_component());
            }
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

pub fn compute_dropdown_menu_logical_pos(
    header_render_area: &Vector4<f32>,
    overlay_contents_area: &Vector4<f32>,
    menu_logical_size: &Vector2<f32>,
    dpi_scale: f32,
) -> Vector2<f32> {
    let safe_dpi = dpi_scale.max(MIN_DROPDOWN_DPI_SCALE);
    let offset_phys_y = DROPDOWN_MENU_OFFSET_Y * safe_dpi;
    let margin_phys = DROPDOWN_SCREEN_MARGIN * safe_dpi;
    let menu_phys_w = menu_logical_size.x * safe_dpi;
    let menu_phys_h = menu_logical_size.y * safe_dpi;

    let mut rel_phys_x = header_render_area.x - overlay_contents_area.x;
    let mut rel_phys_y = (header_render_area.w - overlay_contents_area.y) + offset_phys_y;

    let overlay_phys_w = overlay_contents_area.z - overlay_contents_area.x;
    let overlay_phys_h = overlay_contents_area.w - overlay_contents_area.y;

    if overlay_phys_w > 0.0 && overlay_phys_h > 0.0 {
        if rel_phys_y + menu_phys_h > overlay_phys_h - margin_phys {
            let above_phys_y = (header_render_area.y - overlay_contents_area.y) - menu_phys_h - offset_phys_y;
            let below_space = overlay_phys_h - (header_render_area.w - overlay_contents_area.y);
            let above_space = header_render_area.y - overlay_contents_area.y;
            if above_phys_y >= margin_phys || above_space > below_space {
                rel_phys_y = above_phys_y;
            }
        }

        let max_x = (overlay_phys_w - menu_phys_w - margin_phys).max(margin_phys);
        let max_y = (overlay_phys_h - menu_phys_h - margin_phys).max(margin_phys);
        rel_phys_x = rel_phys_x.clamp(margin_phys.min(max_x), max_x);
        rel_phys_y = rel_phys_y.clamp(margin_phys.min(max_y), max_y);
    }

    Vector2::new(rel_phys_x / safe_dpi, rel_phys_y / safe_dpi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_logical_pos_dpi_scaling() {
        // 2560x1440 monitor (dpi_scale = 2560 / 1920 = 4/3)
        let dpi_scale = 2560.0 / 1920.0;
        let overlay = Vector4::new(0.0, 0.0, 2560.0, 1440.0);
        let header = Vector4::new(1200.0, 460.0, 1520.0, 505.0);
        let menu_logical_size = Vector2::new(240.0, 120.0);

        let logical_pos = compute_dropdown_menu_logical_pos(&header, &overlay, &menu_logical_size, dpi_scale);

        // FloatLayout multiplies logical_pos by dpi_scale and adds overlay origin:
        let resulting_phys_x = overlay.x + logical_pos.x * dpi_scale;
        let resulting_phys_y = overlay.y + logical_pos.y * dpi_scale;
        assert!((resulting_phys_x - header.x).abs() < 1e-3);
        assert!((resulting_phys_y - (header.w + DROPDOWN_MENU_OFFSET_Y * dpi_scale)).abs() < 1e-3);
    }

    #[test]
    fn test_dropdown_logical_pos_flips_above_when_overflowing_bottom() {
        let dpi_scale = 1.0;
        let overlay = Vector4::new(0.0, 0.0, 1920.0, 1080.0);
        let header = Vector4::new(800.0, 950.0, 1040.0, 984.0);
        let menu_logical_size = Vector2::new(240.0, 200.0);

        let logical_pos = compute_dropdown_menu_logical_pos(&header, &overlay, &menu_logical_size, dpi_scale);

        // Should flip above header.y: 950 - 200 - 4 = 746
        assert!((logical_pos.x - 800.0).abs() < 1e-3);
        assert!((logical_pos.y - 746.0).abs() < 1e-3);
    }
}
