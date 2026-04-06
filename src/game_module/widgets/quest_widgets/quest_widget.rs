use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::begin_block;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::{GatherItemData, QuestItemGatherItem};
use crate::game_module::widgets::quest_widgets::quest_item_default::{DefaultQuestData, QuestItemDefault};
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;

pub const FONT_SIZE: f32 = 30.0;
pub const ITEM_SIZE: f32 = 50.0;
pub const ITEM_MARGIN: f32 = 20.0;
pub const ITEM_PADDING: f32 = 8.0;
pub const QUEST_COMPLETE_OPACITY: f32 = 0.3;

pub enum QuestCreateInfo {
    DefaultQuest(DefaultQuestData),
    GatherItem(GatherItemData),
}

pub trait QuestItemBase<'a> {
    fn initialize_quest_item(&mut self);
    fn destroy(&mut self);
    fn is_completed_quest(&self) -> bool;
    fn set_completed_quest(&mut self);
    fn update_quest_item(&mut self, game_controller: &GameController, delta_time: f32);
}

pub struct QuestWidget<'a> {
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _root_widget: Rc<WidgetDefault<'a>>,
    pub _quests: Vec<RcRefCell<QuestTitle<'a>>>
}

pub fn create_quest_item<'a>(game_scene_manager: *const GameSceneManager<'a>, game_resources: *const GameResources<'a>, parent_widget: &mut WidgetDefault<'a>, quest_type: QuestCreateInfo) -> QuestItem<'a> {
    match quest_type {
        QuestCreateInfo::DefaultQuest(default_quest_data) => {
            QuestItemDefault::create_quest_item(game_scene_manager, game_resources, parent_widget, default_quest_data)
        }
        QuestCreateInfo::GatherItem(gather_item_data) => {
            QuestItemGatherItem::create_quest_item(game_scene_manager, game_resources, parent_widget, gather_item_data)
        }
    }
}

pub fn create_quest_item_layout<'a>(parent_widget: &mut WidgetDefault<'a>) -> Rc<WidgetDefault<'a>> {
    let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_size_y(ITEM_SIZE);
    ui_component.set_color(get_color32(0, 0, 0, 0));
    ui_component.set_expandable_x(true);
    parent_widget.add_widget(&layout_widget);
    layout_widget
}

impl<'a> QuestWidget<'a> {
    pub fn create_quest_widget(game_scene_manager: *const GameSceneManager<'a>, game_resources: *const GameResources<'a>, root_widget: &mut WidgetDefault<'a>) -> QuestWidget<'a> {
        let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pos_hint_x(PosHintX::Left(0.0));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_margin_left(ITEM_MARGIN);
        ui_component.set_margin_right(ITEM_MARGIN);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_size_y(ITEM_SIZE * 5.0);
        ui_component.set_round(10.0);
        ui_component.set_padding(ITEM_PADDING);
        ui_component.set_expandable_x(true);
        ui_component.set_expandable_y(true);
        ui_component.set_visible(false);
        root_widget.add_widget(&layout_widget);

        QuestWidget {
            _game_scene_manager: game_scene_manager,
            _game_resources: game_resources,
            _root_widget: layout_widget,
            _quests: Vec::new(),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
    }

    pub fn add_quest(&mut self, title: Option<String>) -> RcRefCell<QuestTitle<'a>> {
        ptr_as_mut(self._root_widget.as_ref()).get_ui_component_mut().set_visible(true);
        let quest = QuestTitle::create_quest_title(self._game_scene_manager, self._game_resources, ptr_as_mut(self._root_widget.as_ref()), title);
        self._quests.push(quest.clone());
        quest.clone()
    }

    pub fn update_quest_widget(&mut self, game_controller: &GameController, delta_time: f32) {
        let item_count = self._quests.len();
        let mut index = 0;
        for _ in 0..item_count {
            let mut remove = false;
            begin_block!("Update Quest Item"); {
                let mut quest_item = self._quests[index].borrow_mut();
                quest_item.update_quest_item(game_controller, delta_time);
                if quest_item.is_completed_quest() {
                    quest_item.destroy();
                    remove = true;
                }
            }

            if remove {
                self._quests.remove(index);
                continue;
            }

            index += 1;
        }

        if 0 < item_count && self._quests.is_empty() {
            ptr_as_mut(self._root_widget.as_ref()).get_ui_component_mut().set_visible(false);
        }
    }
}
