use std::rc::Rc;
use rust_engine_3d::scene::ui::{HorizontalAlign, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::QuestItemType;
use crate::game_module::widgets::quest_widgets::quest_widget::{create_quest_item_layout, QuestItemBase, FONT_SIZE, ITEM_SIZE};

pub struct GatherItemData {
    pub _item_data_type: ItemDataType,
    pub _gather_item_count: usize,
}

pub struct QuestItemGatherItem<'a> {
    pub _layout_widget: Rc<WidgetDefault<'a>>,
    pub _icon_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _item_data: GatherItemData,
    pub _item_count: usize,
}

impl<'a> QuestItemGatherItem<'a> {
    pub(crate) fn create_quest_item(game_resources: *const GameResources<'a>, parent_widget: &mut WidgetDefault<'a>, content: GatherItemData) -> QuestItemType<'a> {
        let item = newRcRefCell(QuestItemGatherItem {
            _layout_widget: create_quest_item_layout(parent_widget),
            _icon_widget: UIManager::create_widget("icon_widget", UIWidgetTypes::Default),
            _text_widget: UIManager::create_widget("text_widget", UIWidgetTypes::Default),
            _item_data: content,
            _item_count: 0
        });

        item.borrow_mut().initialize_quest_item(game_resources);
        item
    }
}

impl<'a> QuestItemBase<'a> for QuestItemGatherItem<'a> {
    fn initialize_quest_item(&mut self, game_resources: *const GameResources<'a>) {
        let game_resources = ptr_as_ref(game_resources);
        let engine_resources = game_resources.get_engine_resources();
        let material_instance = engine_resources.get_material_instance_data(
            ItemDataType::get_item_material_instance_name(self._item_data._item_data_type)
        ).clone();

        let ui_component = ptr_as_mut(self._icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(ITEM_SIZE);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_material_instance(Some(material_instance));
        ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._icon_widget);

        let ui_component = ptr_as_mut(self._text_widget.as_ref()).get_ui_component_mut();
        ui_component.set_expandable_x(true);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_text(format!("Collected: 0/{}", self._item_data._gather_item_count).as_str());
        ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._text_widget);
    }

    fn destroy(&mut self) {
        let parent: &*const WidgetDefault = self._layout_widget.as_ref()._parent.as_ref().unwrap();
        ptr_as_mut(*parent).remove_widget(self._layout_widget.as_ref());
    }

    fn is_completed_quest(&self) -> bool {
        self._item_data._gather_item_count <= self._item_count
    }

    fn update_quest_item(&mut self, _game_scene_manager: &GameSceneManager, game_controller: &GameController, _delta_time: f32) {
        let item_bar_widget = game_controller.get_game_ui_manager().get_item_bar_widget();
        let item_count = item_bar_widget.get_item_count(&self._item_data._item_data_type);
        if self._item_count != item_count {
            self._item_count = self._item_data._gather_item_count.min(item_count);
            let ui_component = ptr_as_mut(self._text_widget.as_ref()).get_ui_component_mut();
            ui_component.set_text(format!("Collected: {}/{}", self._item_count, self._item_data._gather_item_count).as_str());
        }
    }
}