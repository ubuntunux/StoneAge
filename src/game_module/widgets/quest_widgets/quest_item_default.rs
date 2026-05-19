use std::rc::Rc;
use rust_engine_3d::scene::ui::{HorizontalAlign, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_constants::AUDIO_QUEST_COMPLETE;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::quest_widgets::quest_widget::{create_quest_item_layout, QuestItemBase, FONT_SIZE, ITEM_MARGIN, ITEM_SIZE, QUEST_COMPLETE_OPACITY};

pub struct DefaultQuestData {
    pub _quest_icon_name: Option<String>,
    pub _quest_description: Option<String>,
}

pub struct QuestItemDefault<'a> {
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _layout_widget: Rc<WidgetDefault<'a>>,
    pub _is_complete_widget: Rc<WidgetDefault<'a>>,
    pub _icon_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _is_completed_quest: bool,
    pub _quest_data: DefaultQuestData,
}

impl<'a> QuestItemDefault<'a> {
    pub fn create_quest_item(
        game_scene_manager: *const GameSceneManager<'a>,
        game_resources: *const GameResources<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        default_quest_data: DefaultQuestData
    ) -> QuestItem<'a> {
        let item = newRcRefCell(QuestItemDefault {
            _game_scene_manager: game_scene_manager,
            _game_resources: game_resources,
            _layout_widget: create_quest_item_layout(parent_widget),
            _is_complete_widget: UIManager::create_widget("is_complete_widget", UIWidgetTypes::Default),
            _icon_widget: UIManager::create_widget("icon_widget", UIWidgetTypes::Default),
            _text_widget: UIManager::create_widget("text_widget", UIWidgetTypes::Default),
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
        ui_component.set_opacity(if is_completed_quest { QUEST_COMPLETE_OPACITY } else { 1.0 });

        let ui_component = ptr_as_mut(self._is_complete_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text(if is_completed_quest {"[X]"} else {"[ ]"});
    }
}

impl<'a> QuestItemBase<'a> for QuestItemDefault<'a> {
    fn initialize_quest_item(&mut self) {
        let game_resources = ptr_as_ref(self._game_resources);
        let engine_resources = game_resources.get_engine_resources();

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
    }

    fn destroy(&mut self) {
        if let Some(parent) = self._layout_widget.as_ref()._parent {
            ptr_as_mut(parent).remove_widget(self._layout_widget.as_ref());
        }
    }

    fn is_completed_quest(&self) -> bool {
        self._is_completed_quest
    }

    fn set_completed_quest(&mut self) {
        if self._is_completed_quest == false {
            ptr_as_ref(self._game_scene_manager).get_scene_manager().play_audio_bank(AUDIO_QUEST_COMPLETE);
            self._is_completed_quest = true;
        }
    }

    fn update_quest_item(&mut self, _game_controller: &GameController, _delta_time: f32) {
        self.update_ui_widgets();
    }
}