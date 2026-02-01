use std::collections::HashMap;
use nalgebra::{Vector2};
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController};

const FONT_SIZE: f32 = 30.0;
const ITEM_HEIGHT: f32 = 20.0;
const ICON_SIZE: f32 = 100.0;
const ITEM_PADDING: f32 = 10.0;


pub enum QuestContent {
    Text(String),
}

pub struct QuestWidget<'a> {
    pub _root_widget: *const WidgetDefault<'a>,
    pub _quest_items:  Vec<QuestItem<'a>>
}

pub struct QuestItem<'a> {
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _duration: f32,
}

impl<'a> QuestItem<'a> {
    pub fn create_quest_item(parent_widget: &mut WidgetDefault<'a>, contents: &Vec<QuestContent>, duration: f32) -> QuestItem<'a> {
        let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Left(0.0));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_padding(ITEM_PADDING);
        ui_component.set_expandable_x(true);
        parent_widget.add_widget(&layout_widget);

        let mut item = QuestItem {
            _layout_widget: layout_widget.as_ref(),
            _duration: duration,
        };

        item.update_quest_item(contents, duration);
        item
    }

    pub fn update_quest_item(&mut self, contents: &Vec<QuestContent>, duration: f32) {
        let mut widget_height = 0.0;
        for content in contents.iter() {
            let binding_widget = UIManager::create_widget("binding_widget", UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(binding_widget.as_ref()).get_ui_component_mut();
            ui_component.set_expandable_x(true);
            match content {
                QuestContent::Text(text) => {
                    ui_component.set_size_hint_x(Some(1.0));
                    ui_component.set_size_y(ITEM_HEIGHT);
                    ui_component.set_color(get_color32(255, 255, 255, 0));
                    ui_component.set_font_color(get_color32(255, 255, 255, 255));
                    ui_component.set_font_size(FONT_SIZE);
                    ui_component.set_text(text);
                }
            }
            widget_height += ui_component.get_size_y();
            ptr_as_mut(self._layout_widget).add_widget(&binding_widget);
        }
        ptr_as_mut(self._layout_widget)._ui_component.set_size_y(widget_height + ITEM_PADDING * 2.0);
        self._duration = duration;
    }
}

impl<'a> QuestWidget<'a> {
    pub fn create_quest_widget(_engine_resources: &EngineResources<'a>, root_widget: &mut WidgetDefault<'a>) -> QuestWidget<'a> {
        QuestWidget {
            _root_widget: root_widget,
            _quest_items: Vec::new(),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
    }

    pub fn add_quest_item(&mut self, contents: &Vec<QuestContent>, duration: f32) {
        self._quest_items.push(
            QuestItem::create_quest_item(ptr_as_mut(self._root_widget), contents, duration)
        );
    }

    pub fn update_quest_widget(&mut self, _game_scene_manager: &GameSceneManager, _game_controller: &GameController, delta_time: f32) {
        let item_count = self._quest_items.len();
        let mut index = 0;
        for _ in 0..item_count {
            self._quest_items[index]._duration -= delta_time;
            if self._quest_items[index]._duration <= 0.0 {
                let item = self._quest_items.remove(index);
                ptr_as_mut(self._root_widget).remove_widget(item._layout_widget);
            } else {
                index += 1;
            }
        }
    }
}
