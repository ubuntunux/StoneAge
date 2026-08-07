use crate::game_module::actors::items::ItemData;
use crate::game_module::game_constants::AUDIO_QUEST_COMPLETE;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_service_locator::get_game_ui_manager;
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::quest_widgets::quest_widget::{
    FONT_SIZE, ITEM_MARGIN, ITEM_SIZE, QUEST_COMPLETE_OPACITY, QuestItemBase, create_quest_item_layout,
};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::scene::ui::{HorizontalAlign, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub struct GatherItemData {
    pub _item_data_name: String,
    pub _item_data: RcRefCell<ItemData>,
    pub _gather_item_count: usize,
}

pub struct QuestItemGatherItem<'a> {
    pub _layout_widget: Rc<WidgetDefault<'a>>,
    pub _is_complete_widget: Rc<WidgetDefault<'a>>,
    pub _icon_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _item_data: GatherItemData,
    pub _item_count: usize,
    pub _is_completed_quest: bool,
}

impl<'a> QuestItemGatherItem<'a> {
    pub fn create_quest_item(parent_widget: &mut WidgetDefault<'a>, content: GatherItemData) -> QuestItem<'a> {
        let item = newRcRefCell(QuestItemGatherItem {
            _layout_widget: create_quest_item_layout(parent_widget),
            _is_complete_widget: UIManager::create_widget("is_complete_widget", UIWidgetTypes::Default),
            _icon_widget: UIManager::create_widget("icon_widget", UIWidgetTypes::Default),
            _text_widget: UIManager::create_widget("text_widget", UIWidgetTypes::Default),
            _item_data: content,
            _item_count: 0,
            _is_completed_quest: false,
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

        let ui_component = ptr_as_mut(self._text_widget.as_ref()).get_ui_component_mut();
        ui_component
            .set_text(format!("Collected: {}/{}", self._item_count, self._item_data._gather_item_count).as_str());
    }
}

impl<'a> QuestItemBase<'a> for QuestItemGatherItem<'a> {
    fn initialize_quest_item(&mut self) {
        let engine_resources = get_engine_resources();
        let item_material_instance = engine_resources
            .get_material_instance_data(self._item_data._item_data.borrow()._ui_material_instance.as_str())
            .clone();

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

        let ui_component = ptr_as_mut(self._icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(ITEM_SIZE);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_material_instance(Some(item_material_instance));
        ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._icon_widget);

        let ui_component = ptr_as_mut(self._text_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable_x(true);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(FONT_SIZE);
        ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._text_widget);
    }

    fn destroy_quest_item(&mut self) {
        if let Some(parent) = self._layout_widget.as_ref()._parent {
            ptr_as_mut(parent).remove_widget(self._layout_widget.as_ref());
        }
    }

    fn is_completed_quest(&self) -> bool {
        self._is_completed_quest || self._item_data._gather_item_count <= self._item_count
    }

    fn set_completed_quest(&mut self) {
        if !self._is_completed_quest {
            self._item_count = self._item_data._gather_item_count;
            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
            self._is_completed_quest = true;
        }
    }

    fn load_completed_quest(&mut self) {
        self._is_completed_quest = true;
        self._item_count = self._item_data._gather_item_count;
        self.update_ui_widgets();
    }

    fn update_quest_item(&mut self, _game_controller: &GameController, _delta_time: f32) {
        let item_bar_widget = get_game_ui_manager().get_item_bar_widget();
        let item_count = item_bar_widget.get_item_count(self._item_data._item_data_name.as_str());
        if self._item_count != item_count {
            self._item_count = self._item_data._gather_item_count.min(item_count);
            self.update_ui_widgets();

            if !self._is_completed_quest && self.is_completed_quest() {
                self.set_completed_quest();
            }
        }
    }
}
