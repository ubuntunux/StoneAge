use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::begin_block;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::QuestItemType;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::{GatherItemData, QuestItemGatherItem};

pub const FONT_SIZE: f32 = 30.0;
pub const ITEM_SIZE: f32 = 50.0;
pub const ITEM_PADDING: f32 = 8.0;

pub enum QuestContent {
    GatherItem(GatherItemData),
}

pub trait QuestItemBase<'a> {
    fn initialize_quest_item(&mut self, game_resources: *const GameResources<'a>);
    fn destroy(&mut self);
    fn is_completed_quest(&self) -> bool;
    fn update_quest_item(&mut self, game_scene_manager: &GameSceneManager, game_controller: &GameController, delta_time: f32);
}

pub struct QuestWidget<'a> {
    pub _game_resources: *const GameResources<'a>,
    pub _root_widget: Rc<WidgetDefault<'a>>,
    pub _quest_items:  Vec<QuestItemType<'a>>
}

pub fn create_quest_item<'a>(game_resources: *const GameResources<'a>, parent_widget: &mut WidgetDefault<'a>, content: QuestContent) -> QuestItemType<'a> {
    match content {
        QuestContent::GatherItem(gather_item_data) => QuestItemGatherItem::create_quest_item(game_resources, parent_widget, gather_item_data),
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
    pub fn create_quest_widget(game_resources: *const GameResources<'a>, root_widget: &mut WidgetDefault<'a>) -> QuestWidget<'a> {
        let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Left(0.0));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_round(10.0);
        ui_component.set_padding(ITEM_PADDING);
        ui_component.set_expandable_x(true);
        ui_component.set_expandable_y(true);
        ui_component.set_visible(false);
        root_widget.add_widget(&layout_widget);

        QuestWidget {
            _game_resources: game_resources,
            _root_widget: layout_widget,
            _quest_items: Vec::new(),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
    }

    pub fn add_quest_item(&mut self, content: QuestContent) -> QuestItemType<'a> {
        ptr_as_mut(self._root_widget.as_ref()).get_ui_component_mut().set_visible(false);
        let quest_item = create_quest_item(self._game_resources, ptr_as_mut(self._root_widget.as_ref()), content);
        self._quest_items.push(quest_item.clone());
        quest_item.clone()
    }

    pub fn update_quest_widget(&mut self, game_scene_manager: &GameSceneManager, game_controller: &GameController, delta_time: f32) {
        let item_count = self._quest_items.len();
        let mut index = 0;
        for _ in 0..item_count {
            let mut remove = false;
            begin_block!("Update Quest Item"); {
                let mut quest_item = self._quest_items[index].borrow_mut();
                quest_item.update_quest_item(game_scene_manager, game_controller, delta_time);
                if quest_item.is_completed_quest() {
                    quest_item.destroy();
                    remove = true;
                }
            }

            if remove {
                self._quest_items.remove(index);
                continue;
            }

            index += 1;
        }

        if 0 < item_count && self._quest_items.is_empty() {
            ptr_as_mut(self._root_widget.as_ref()).get_ui_component_mut().set_visible(false);
        }
    }
}
