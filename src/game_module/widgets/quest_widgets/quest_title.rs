use std::rc::Rc;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::quest_widgets::quest_widget::{create_quest_item, create_quest_item_layout, QuestCreateInfo, FONT_SIZE, ITEM_SIZE};

pub struct QuestTitle<'a> {
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _layout_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Option<Rc<WidgetDefault<'a>>>,
    pub _quest_title: Option<String>,
    pub _quest_items: Vec<QuestItem<'a>>,
}

impl<'a> QuestTitle<'a> {
    pub fn create_quest_title(game_scene_manager: *const GameSceneManager<'a>, game_resources: *const GameResources<'a>, parent_widget: &mut WidgetDefault<'a>, title: Option<String>) -> RcRefCell<QuestTitle<'a>> {
        let item = newRcRefCell(QuestTitle {
            _game_scene_manager: game_scene_manager.clone(),
            _game_resources: game_resources.clone(),
            _layout_widget: create_quest_item_layout(parent_widget),
            _text_widget: if title.is_some() { Some(UIManager::create_widget("text_widget", UIWidgetTypes::Default)) } else { None },
            _quest_title: title,
            _quest_items: Vec::new()
        });

        item.borrow_mut().initialize_quest_title();
        item
    }

    fn initialize_quest_title(&mut self) {
        let ui_component = ptr_as_mut(self._layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_color(get_color32(255, 255, 255, 0));

        if self._text_widget.is_some() {
            let ui_component = ptr_as_mut(self._text_widget.as_ref().unwrap().as_ref()).get_ui_component_mut();
            ui_component.set_expandable_x(true);
            ui_component.set_halign(HorizontalAlign::LEFT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_size_y(ITEM_SIZE);
            ui_component.set_color(get_color32(255, 255, 255, 0));
            ui_component.set_font_color(get_color32(255, 255, 255, 255));
            ui_component.set_font_size(FONT_SIZE);
            ui_component.set_text(self._quest_title.as_ref().unwrap().as_str());
            ptr_as_mut(self._layout_widget.as_ref()).add_widget(&self._text_widget.as_ref().unwrap());
        }
    }

    pub fn destroy(&mut self) {
        for quest_item in self._quest_items.iter() {
            quest_item.borrow_mut().destroy();
        }

        let parent: &*const WidgetDefault = self._layout_widget.as_ref()._parent.as_ref().unwrap();
        ptr_as_mut(*parent).remove_widget(self._layout_widget.as_ref());
    }

    pub fn add_quest_item(&mut self, content: QuestCreateInfo) -> QuestItem<'a> {
        let quest_item = create_quest_item(self._game_scene_manager, self._game_resources, ptr_as_mut(self._layout_widget.as_ref()), content);
        self._quest_items.push(quest_item.clone());
        quest_item.clone()
    }

    pub fn is_completed_quest(&self) -> bool {
        for quest_item in self._quest_items.iter() {
            if !quest_item.borrow().is_completed_quest() {
                return false;
            }
        }
        true
    }

    pub fn set_completed_quest(&mut self) {
        for quest_item in self._quest_items.iter() {
            quest_item.borrow_mut().set_completed_quest();
        }
    }

    pub fn update_quest_item(&mut self, game_controller: &GameController, delta_time: f32) {
        for quest_item in self._quest_items.iter() {
            quest_item.borrow_mut().update_quest_item(game_controller, delta_time);
        }
    }
}