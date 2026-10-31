use crate::game_module::game_constants::AUDIO_QUEST_COMPLETE;
use crate::game_module::game_controller::{GameController, KeyBindingType};
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::key_binding_widget::{KEY_BINDING_FONT_SIZE, KEY_BINDING_UI_SIZE, KeyBindingWidget};
use crate::game_module::widgets::quest_widgets::quest_widget::{
    FONT_SIZE, ITEM_MARGIN, ITEM_SIZE, QUEST_COMPLETE_OPACITY, QuestItemBase, create_quest_item_layout,
};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_core, get_engine_resources};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub const QUEST_KEY_BINDING_MARGIN_LEFT: f32 = 6.0;
pub const QUEST_KEY_BINDING_TEXT_MARGIN_LEFT: f32 = 6.0;

pub fn get_quest_key_binding_icons<'a>(
    engine_resources: &rust_engine_3d::resource::resource::EngineResources<'a>,
    key_binding_type: KeyBindingType,
) -> (
    Vec<RcRefCell<MaterialInstanceData<'a>>>,
    Vec<RcRefCell<MaterialInstanceData<'a>>>,
) {
    match key_binding_type {
        KeyBindingType::Craft => (
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_c").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_y").clone()],
        ),
        KeyBindingType::Interaction => (
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()],
        ),
        KeyBindingType::Jump => (
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_space").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_a").clone()],
        ),
        _ => (Vec::new(), Vec::new()),
    }
}

pub fn create_quest_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    key_binding_text: Option<&str>,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
) -> KeyBindingWidget<'a> {
    let max_icons = key_binding_icons.len().max(joystick_binding_icons.len()).max(1);
    let layout_widget = UIManager::create_widget("quest_key_binding_layout", UIWidgetTypes::Default);
    let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
    let ui_component = layout_widget_mut.get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_margin_left(QUEST_KEY_BINDING_MARGIN_LEFT);
    ui_component.set_expandable_x(true);
    ui_component.set_size_x(0.0);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_round(6.0);
    ui_component.set_color(get_color32(0, 0, 0, 0));
    parent_widget.add_widget(&layout_widget);

    let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
    for _ in 0..max_icons {
        let binding_icon_widget = UIManager::create_widget("quest_key_binding_icon", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(KEY_BINDING_UI_SIZE);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 255));
        layout_widget_mut.add_widget(&binding_icon_widget);
        binding_icon_widgets.push(binding_icon_widget.as_ref());
    }

    let binding_name_widget_ptr = if let Some(text) = key_binding_text {
        let binding_name_widget = UIManager::create_widget("quest_key_binding_text", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_left(QUEST_KEY_BINDING_TEXT_MARGIN_LEFT);
        ui_component.set_expandable_x(true);
        ui_component.set_size_x(0.0);
        ui_component.set_size_y(KEY_BINDING_UI_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(KEY_BINDING_FONT_SIZE);
        ui_component.set_text(text);
        layout_widget_mut.add_widget(&binding_name_widget);
        binding_name_widget.as_ref() as *const WidgetDefault<'a>
    } else {
        std::ptr::null()
    };

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: binding_name_widget_ptr,
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons,
        _progress_bar_bg: None,
        _progress_bar_fill: None,
    }
}

#[derive(Clone, Debug, Default)]
pub struct DefaultQuestData {
    pub _quest_icon_name: Option<String>,
    pub _quest_description: Option<String>,
    pub _key_binding_type: Option<KeyBindingType>,
    pub _key_binding_text: Option<String>,
}

pub struct QuestItemDefault<'a> {
    pub _layout_widget: Rc<WidgetDefault<'a>>,
    pub _is_complete_widget: Rc<WidgetDefault<'a>>,
    pub _icon_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _key_binding_widget: Option<KeyBindingWidget<'a>>,
    pub _last_is_keyboard_input_mode: Option<bool>,
    pub _is_completed_quest: bool,
    pub _quest_data: DefaultQuestData,
}

impl<'a> QuestItemDefault<'a> {
    pub fn create_quest_item(
        parent_widget: &mut WidgetDefault<'a>,
        default_quest_data: DefaultQuestData,
    ) -> QuestItem<'a> {
        let item = newRcRefCell(QuestItemDefault {
            _layout_widget: create_quest_item_layout(parent_widget),
            _is_complete_widget: UIManager::create_widget("is_complete_widget", UIWidgetTypes::Default),
            _icon_widget: UIManager::create_widget("icon_widget", UIWidgetTypes::Default),
            _text_widget: UIManager::create_widget("text_widget", UIWidgetTypes::Default),
            _key_binding_widget: None,
            _last_is_keyboard_input_mode: None,
            _is_completed_quest: false,
            _quest_data: default_quest_data,
        });

        item.borrow_mut().initialize_quest_item();
        item.borrow_mut().update_ui_widgets();
        item
    }

    pub fn update_ui_widgets(&mut self) {
        let is_completed_quest = self.is_completed_quest();

        let ui_component = ptr_as_mut(self._layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_opacity(if is_completed_quest {
            QUEST_COMPLETE_OPACITY
        } else {
            1.0
        });

        let ui_component = ptr_as_mut(self._is_complete_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text(if is_completed_quest { "[X]" } else { "[ ]" });

        if let Some(key_binding_widget) = self._key_binding_widget.as_mut() {
            ptr_as_mut(key_binding_widget._layout_widget)._ui_component.set_visible(!is_completed_quest);
        }
    }
}

impl<'a> QuestItemBase<'a> for QuestItemDefault<'a> {
    fn initialize_quest_item(&mut self) {
        let engine_resources = get_engine_resources();

        let ui_component = ptr_as_mut(self._layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_expandable_x(true);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_left(ITEM_MARGIN);

        let ui_component = ptr_as_mut(self._is_complete_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(FONT_SIZE);
        ui_component.set_size_y(FONT_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(FONT_SIZE);
        ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._is_complete_widget);

        if let Some(icon_name) = self._quest_data._quest_icon_name.as_ref() {
            let icon_material_instance = engine_resources.get_material_instance_data(icon_name.as_str()).clone();
            let ui_component = ptr_as_mut(self._icon_widget.as_ref()).get_ui_component_mut();
            ui_component.set_halign(HorizontalAlign::CENTER);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_size_x(ITEM_SIZE);
            ui_component.set_size_y(ITEM_SIZE);
            ui_component.set_material_instance(Some(icon_material_instance));
            ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._icon_widget);
        }

        if let Some(quest_description) = self._quest_data._quest_description.as_ref() {
            let ui_component = ptr_as_mut(self._text_widget.as_ref()).get_ui_component_mut();
            ui_component.set_halign(HorizontalAlign::LEFT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_expandable_x(true);
            ui_component.set_size_y(ITEM_SIZE);
            ui_component.set_color(get_color32(255, 255, 255, 0));
            ui_component.set_font_color(get_color32(255, 255, 255, 255));
            ui_component.set_font_size(FONT_SIZE);
            ui_component.set_text(quest_description);
            ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._text_widget);
        }

        if let Some(key_binding_type) = self._quest_data._key_binding_type {
            let (key_icons, joystick_icons) = get_quest_key_binding_icons(engine_resources, key_binding_type);
            let mut key_binding_widget = create_quest_key_binding_widget(
                ptr_as_mut(self._layout_widget.as_ref()),
                key_binding_type,
                self._quest_data._key_binding_text.as_deref(),
                key_icons,
                joystick_icons,
            );
            let is_keyboard_input_mode = get_engine_core().is_keyboard_input_mode();
            key_binding_widget.update_icon_material_instance(is_keyboard_input_mode);
            self._last_is_keyboard_input_mode = Some(is_keyboard_input_mode);
            self._key_binding_widget = Some(key_binding_widget);
        }
    }

    fn destroy_quest_item(&mut self) {
        if let Some(parent) = self._layout_widget.as_ref()._parent {
            ptr_as_mut(parent).remove_widget(self._layout_widget.as_ref());
        }
    }

    fn is_completed_quest(&self) -> bool {
        self._is_completed_quest
    }

    fn set_completed_quest(&mut self) {
        if !self._is_completed_quest {
            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
            self._is_completed_quest = true;
        }
    }

    fn load_completed_quest(&mut self) {
        self._is_completed_quest = true;
        self.update_ui_widgets();
    }

    fn update_quest_item(&mut self, _game_controller: &GameController, _delta_time: f32) {
        let is_keyboard_input_mode = get_engine_core().is_keyboard_input_mode();
        if self._last_is_keyboard_input_mode != Some(is_keyboard_input_mode) {
            self._last_is_keyboard_input_mode = Some(is_keyboard_input_mode);
            if let Some(key_binding_widget) = self._key_binding_widget.as_mut() {
                key_binding_widget.update_icon_material_instance(is_keyboard_input_mode);
            }
        }
        self.update_ui_widgets();
    }
}
