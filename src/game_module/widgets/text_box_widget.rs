use std::collections::HashMap;
use nalgebra::{Vector2};
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController};

const FONT_SIZE: f32 = 30.0;
const ITEM_HEIGHT: f32 = 20.0;
const ICON_SIZE: f32 = 100.0;
const ITEM_PADDING: f32 = 10.0;
const TEXT_BOX_ANIMATION_DURATION: f32 = 0.25;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TextBoxAnimationState {
    None,
    Growing,
    Idle,
    Shrinking,
}

pub enum TextBoxContent<'a> {
    Text(String),
    MaterialInstance(RcRefCell<MaterialInstanceData<'a>>),
}

pub struct TextBoxWidget<'a> {
    pub _root_widget: *const WidgetDefault<'a>,
    pub _text_box_items:  HashMap<String, TextBoxItem<'a>>
}

pub struct TextBoxItem<'a> {
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _duration: Option<f32>,
    pub _animation_state: TextBoxAnimationState,
    pub _animation_timer: f32,
}

impl<'a> TextBoxItem<'a> {
    pub fn create_text_box_item(parent_widget: &mut WidgetDefault<'a>, contents: &Vec<TextBoxContent<'a>>, duration: Option<f32>) -> TextBoxItem<'a> {
        let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 255, 255, 128));
        ui_component.set_round(30.0);
        ui_component.set_padding(ITEM_PADDING);
        ui_component.set_expandable_x(true);
        ui_component.set_opacity(0.0);
        parent_widget.add_widget(&layout_widget);

        let mut item = TextBoxItem {
            _layout_widget: layout_widget.as_ref(),
            _duration: duration,
            _animation_state: TextBoxAnimationState::None,
            _animation_timer: 0.0,
        };

        item.update_text_box_item(contents, duration);
        item
    }

    pub fn update_text_box_item(&mut self, contents: &Vec<TextBoxContent<'a>>, duration: Option<f32>) {
        let mut widget_height = 0.0;
        for content in contents.iter() {
            let binding_widget = UIManager::create_widget("binding_widget", UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(binding_widget.as_ref()).get_ui_component_mut();
            ui_component.set_halign(HorizontalAlign::LEFT);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_expandable_x(true);
            match content {
                TextBoxContent::Text(text) => {
                    ui_component.set_size_hint_x(Some(1.0));
                    ui_component.set_size_y(ITEM_HEIGHT);
                    ui_component.set_color(get_color32(255, 255, 255, 0));
                    ui_component.set_font_color(get_color32(0, 0, 0, 255));
                    ui_component.set_font_size(FONT_SIZE);
                    ui_component.set_text(text);
                }
                TextBoxContent::MaterialInstance(material_instance) => {
                    ui_component.set_size_x(ICON_SIZE);
                    ui_component.set_size_y(ICON_SIZE);
                    ui_component.set_color(get_color32(255, 255, 255, 255));
                    ui_component.set_material_instance(Some(material_instance.clone()));
                }
            }
            widget_height += ui_component.get_size_y();
            ptr_as_mut(self._layout_widget).add_widget(&binding_widget);
        }
        ptr_as_mut(self._layout_widget)._ui_component.set_size_y(widget_height + ITEM_PADDING * 2.0);
        self._duration = duration;
    }

    pub fn set_animation_state(&mut self, state: TextBoxAnimationState) {
        self._animation_state = state;
        self._animation_timer = 0.0;
    }
}

impl<'a> TextBoxWidget<'a> {
    pub fn create_text_box_widget(_engine_resources: &EngineResources<'a>, root_widget: &mut WidgetDefault<'a>) -> TextBoxWidget<'a> {
        TextBoxWidget {
            _root_widget: root_widget,
            _text_box_items: HashMap::new(),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
    }

    pub fn has_text_box_item(&self, character_name: &str) -> bool {
        self._text_box_items.contains_key(character_name)
    }

    pub fn add_text_box_item(&mut self, character_name: &str, contents: &Vec<TextBoxContent<'a>>, duration: Option<f32>) {
        if let Some(item) = self._text_box_items.get_mut(character_name) {
            item.update_text_box_item(contents, duration);
            item.set_animation_state(TextBoxAnimationState::None);
        } else {
            self._text_box_items.insert(
                String::from(character_name),
                TextBoxItem::create_text_box_item(ptr_as_mut(self._root_widget), contents, duration)
            );
        }
    }

    pub fn remove_text_box_item(&mut self, character_name: &str) {
        if let Some(item) = self._text_box_items.get_mut(character_name) {
            item.set_animation_state(TextBoxAnimationState::Shrinking);
        }
    }

    pub fn update_text_box_widget(&mut self, game_scene_manager: &GameSceneManager, _game_controller: &GameController, delta_time: f32) {
        let mut remove_items: Vec<String> = Vec::new();
        for (character_name, text_box_item) in self._text_box_items.iter_mut() {
            if let Some(character) = game_scene_manager.get_character_manager().get_character(character_name) {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let mut position = character.borrow().get_center().clone();
                position.y = character.borrow().get_bounding_box()._max.y;
                let screen_position = main_camera.convert_world_to_screen(&position, false);
                let ui_size = ptr_as_ref(text_box_item._layout_widget).get_ui_component().get_ui_size();
                ptr_as_mut(text_box_item._layout_widget)._ui_component.set_center_x(screen_position.x);
                ptr_as_mut(text_box_item._layout_widget)._ui_component.set_pos_y(screen_position.y - ui_size.y);

                match text_box_item._animation_state {
                    TextBoxAnimationState::None => {
                        ptr_as_mut(text_box_item._layout_widget)._ui_component.set_opacity(0.0);
                        text_box_item.set_animation_state(TextBoxAnimationState::Growing);
                    }
                    TextBoxAnimationState::Growing => {
                        let opacity = (text_box_item._animation_timer / TEXT_BOX_ANIMATION_DURATION).min(1.0);
                        ptr_as_mut(text_box_item._layout_widget)._ui_component.set_opacity(opacity);
                        log::info!("{:?} {:?} {:?} {:?}", character_name, text_box_item._animation_state, ptr_as_ref(text_box_item._layout_widget).get_ui_component().get_opacity(), opacity);
                        if 1.0 <= opacity {
                            text_box_item.set_animation_state(TextBoxAnimationState::Idle);
                        }
                        text_box_item._animation_timer += delta_time;
                    },
                    TextBoxAnimationState::Idle => {
                        if let Some(mut duration) = text_box_item._duration {
                            duration -= delta_time;
                            if duration <= 0.0f32 {
                                text_box_item.set_animation_state(TextBoxAnimationState::Shrinking);
                            }
                            text_box_item._duration = Some(duration);
                        }
                    }
                    TextBoxAnimationState::Shrinking => {
                        text_box_item._animation_timer += delta_time;
                        let opacity = 1.0 - (text_box_item._animation_timer / TEXT_BOX_ANIMATION_DURATION).min(1.0);
                        ptr_as_mut(text_box_item._layout_widget)._ui_component.set_opacity(opacity);
                        if opacity <= 0.0 {
                            remove_items.push(character_name.clone());
                        }
                        text_box_item._animation_timer += delta_time;
                        log::info!("{:?} {:?} {:?} {:?}", character_name, text_box_item._animation_state, ptr_as_ref(text_box_item._layout_widget).get_ui_component().get_opacity(), opacity);
                    },
                }
            } else {
                remove_items.push(character_name.clone());
            }
        }

        for character_name in remove_items {
             if let Some(item) = self._text_box_items.remove(&character_name) {
                ptr_as_mut(self._root_widget).remove_widget(item._layout_widget);
            }
        }
    }
}
