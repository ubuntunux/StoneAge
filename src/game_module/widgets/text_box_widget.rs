use crate::game_module::actors::character::ActorWrapper;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::collections::HashMap;
use std::ffi::c_void;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr};

const FONT_SIZE: f32 = 30.0;
const ITEM_HEIGHT: f32 = 20.0;
const ICON_SIZE: f32 = 100.0;
const ITEM_PADDING: f32 = 10.0;
const TEXT_BOX_ANIMATION_DURATION: f32 = 0.25;
const MAX_TEXT_BOX_HEIGHT: f32 = 2.0;

#[repr(u8)]
#[derive(PartialEq, Debug, Display, EnumCount, EnumIter, FromRepr)]
pub enum TextBoxLayerType {
    InteractionLayer,
    GamePlayLayer,
}

#[derive(PartialEq, Debug)]
pub enum TextBoxAnimationState {
    None,
    Growing,
    Idle,
    Shrinking,
}

pub enum TextBoxContent {
    MaterialInstance(String),
    Text(String),
    StatWidget((String, f32)),
    Audio(String),
}

pub struct TextBoxWidget<'a> {
    pub _audio_manager: *const AudioManager<'a>,
    pub _engine_resources: *const EngineResources<'a>,
    pub _root_widget: *const WidgetDefault<'a>,
    pub _layers: Vec<*const WidgetDefault<'a>>,
    pub _text_box_items: HashMap<*const c_void, TextBoxItem<'a>>,
}

pub struct TextBoxItem<'a> {
    pub _actor: ActorWrapper<'a>,
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _duration: Option<f32>,
    pub _animation_state: TextBoxAnimationState,
    pub _animation_timer: f32,
}

impl<'a> TextBoxItem<'a> {
    pub fn create_text_box_item(
        audio_manager: *const AudioManager<'a>,
        engine_resources: *const EngineResources<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        actor: ActorWrapper<'a>,
        contents: &Vec<TextBoxContent>,
        duration: Option<f32>,
    ) -> TextBoxItem<'a> {
        let layout_widget = UIManager::create_widget("layout_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 255, 255, 128));
        ui_component.set_round(20.0);
        ui_component.set_border(2.0);
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_padding(ITEM_PADDING);
        ui_component.set_expandable_x(true);
        ui_component.set_opacity(0.0);
        parent_widget.add_widget(&layout_widget);

        let mut item = TextBoxItem {
            _actor: actor,
            _layout_widget: layout_widget.as_ref(),
            _duration: duration,
            _animation_state: TextBoxAnimationState::None,
            _animation_timer: 0.0,
        };

        item.update_text_box_item(audio_manager, engine_resources, contents, duration, true);
        item
    }

    pub fn update_text_box_item(
        &mut self,
        audio_manager: *const AudioManager<'a>,
        engine_resources: *const EngineResources<'a>,
        contents: &Vec<TextBoxContent>,
        duration: Option<f32>,
        clear_widgets: bool,
    ) {
        if clear_widgets {
            ptr_as_mut(self._layout_widget).clear_widgets();
        }

        let mut widget_height = 0.0;
        for content in contents.iter() {
            if let TextBoxContent::Audio(audio_name) = content {
                ptr_as_mut(audio_manager).play_audio_bank(audio_name, AudioLoop::ONCE, None);
            } else {
                let binding_widget =
                    UIManager::create_widget("binding_widget", UIWidgetTypes::Default);
                let ui_component = ptr_as_mut(binding_widget.as_ref()).get_ui_component_mut();
                ui_component.set_halign(HorizontalAlign::LEFT);
                ui_component.set_valign(VerticalAlign::CENTER);
                ui_component.set_expandable_x(true);
                match content {
                    TextBoxContent::MaterialInstance(material_name) => {
                        let material_instance =
                            ptr_as_mut(engine_resources).get_material_instance_data(material_name);
                        ui_component.set_size_x(ICON_SIZE);
                        ui_component.set_size_y(ICON_SIZE);
                        ui_component.set_color(get_color32(255, 255, 255, 255));
                        ui_component.set_material_instance(Some(material_instance.clone()));
                    }
                    TextBoxContent::StatWidget((text, ratio)) => {
                        ui_component.set_size_hint_x(Some(1.0));
                        ui_component.set_size_y(ITEM_HEIGHT);
                        ui_component.set_color(get_color32(255, 255, 255, 0));
                        ui_component.set_font_color(get_color32(0, 0, 0, 255));
                        ui_component.set_font_size(FONT_SIZE);
                        ui_component.set_text(&format!("{}: {:.1}%", text, ratio));
                    }
                    TextBoxContent::Text(text) => {
                        ui_component.set_size_hint_x(Some(1.0));
                        ui_component.set_size_y(ITEM_HEIGHT);
                        ui_component.set_color(get_color32(255, 255, 255, 0));
                        ui_component.set_font_color(get_color32(0, 0, 0, 255));
                        ui_component.set_font_size(FONT_SIZE);
                        ui_component.set_text(text);
                    }
                    TextBoxContent::Audio(_) => (),
                }
                widget_height += ui_component.get_size_y();
                ptr_as_mut(self._layout_widget).add_widget(&binding_widget);
            }
        }
        ptr_as_mut(self._layout_widget)
            ._ui_component
            .set_size_y(widget_height + ITEM_PADDING * 2.0);
        self._duration = duration;
    }

    pub fn set_animation_state(&mut self, state: TextBoxAnimationState) {
        if self._animation_state != state {
            self._animation_state = state;
            self._animation_timer = 0.0;
        }
    }
}

impl<'a> TextBoxWidget<'a> {
    pub fn create_text_box_widget(
        audio_manager: *const AudioManager<'a>,
        engine_resources: &EngineResources<'a>,
        root_widget: &mut WidgetDefault<'a>,
    ) -> TextBoxWidget<'a> {
        let text_box_root_widget =
            UIManager::create_widget("TextBoxRootWidget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(text_box_root_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        root_widget.add_widget(&text_box_root_widget);

        let mut layers: Vec<*const WidgetDefault<'a>> = Vec::new();
        for layer_type in TextBoxLayerType::iter() {
            let text_box_widget_layout =
                UIManager::create_widget(layer_type.to_string().as_str(), UIWidgetTypes::Default);
            let ui_component = ptr_as_mut(text_box_widget_layout.as_ref()).get_ui_component_mut();
            ui_component.set_size_hint_x(Some(1.0));
            ui_component.set_size_hint_y(Some(1.0));
            ui_component.set_color(get_color32(0, 0, 0, 0));
            ptr_as_mut(text_box_root_widget.as_ref()).add_widget(&text_box_widget_layout);
            layers.push(text_box_widget_layout.as_ref())
        }

        TextBoxWidget {
            _audio_manager: audio_manager,
            _engine_resources: engine_resources,
            _root_widget: text_box_root_widget.as_ref(),
            _layers: layers,
            _text_box_items: HashMap::new(),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn set_text_box_visible(&mut self, visible: bool) {
        ptr_as_mut(self._root_widget).get_ui_component_mut().set_visible(visible);
    }

    pub fn set_text_box_layer_visible(&mut self, layer: TextBoxLayerType, visible: bool) {
        ptr_as_mut(self._layers[layer as usize]).get_ui_component_mut().set_visible(visible);
    }

    pub fn has_text_box_item(&self, key: *const c_void) -> bool {
        self._text_box_items.contains_key(&key)
    }

    pub fn add_text_box_item(
        &mut self,
        layer_type: TextBoxLayerType,
        actor: ActorWrapper<'a>,
        contents: &Vec<TextBoxContent>,
        duration: Option<f32>,
    ) {
        if let Some(item) = self._text_box_items.get_mut(&actor.get_key()) {
            item.update_text_box_item(
                self._audio_manager,
                self._engine_resources,
                contents,
                duration,
                true,
            );
            item.set_animation_state(TextBoxAnimationState::None);
        } else {
            self._text_box_items.insert(
                actor.get_key(),
                TextBoxItem::create_text_box_item(
                    self._audio_manager,
                    self._engine_resources,
                    ptr_as_mut(self._layers[layer_type as usize]),
                    actor,
                    contents,
                    duration,
                ),
            );
        }
    }

    pub fn remove_text_box_item(&mut self, key: *const c_void) {
        if let Some(item) = self._text_box_items.get_mut(&key) {
            item.set_animation_state(TextBoxAnimationState::Shrinking);
        }
    }

    pub fn update_text_box_widget(
        &mut self,
        game_scene_manager: &GameSceneManager,
        _game_controller: &GameController,
        delta_time: f32,
    ) {
        let mut remove_items: Vec<*const c_void> = Vec::new();
        for (_, text_box_item) in self._text_box_items.iter_mut() {
            let mut is_enable_text_box = false;
            let mut position;
            let key = text_box_item._actor.get_key();

            match &text_box_item._actor {
                ActorWrapper::Prop(prop) => {
                    position = *prop.borrow().get_position();
                    position.y += MAX_TEXT_BOX_HEIGHT
                        .min(prop.borrow().get_bounding_box()._max.y - position.y);
                    if prop.borrow().is_alive() {
                        is_enable_text_box = true;
                    }
                }
                ActorWrapper::Character(character) => {
                    position = *character.borrow().get_center();
                    position.y += MAX_TEXT_BOX_HEIGHT
                        .min(character.borrow().get_bounding_box()._max.y - position.y);
                    if character.borrow().is_alive() {
                        is_enable_text_box = true;
                    }
                }
                ActorWrapper::RenderObject(render_object) => {
                    position = *render_object.borrow().get_position();
                    position.y += MAX_TEXT_BOX_HEIGHT
                        .min(render_object.borrow().get_bounding_box()._max.y - position.y);
                    is_enable_text_box = true;
                }
            }

            if is_enable_text_box {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let ui_size =
                    ptr_as_ref(text_box_item._layout_widget).get_ui_component().get_ui_size();
                let mut screen_position = main_camera.convert_world_to_screen(&position, true);
                screen_position.x -= ui_size.x * 0.5;
                screen_position.y -= ui_size.y * 0.5;
                screen_position.x = 0f32
                    .max((main_camera._window_size.x as f32 - ui_size.x).min(screen_position.x));
                screen_position.y = 0f32
                    .max((main_camera._window_size.y as f32 - ui_size.y).min(screen_position.y));

                let ui_component = &mut ptr_as_mut(text_box_item._layout_widget)._ui_component;
                ui_component.set_pos(screen_position.x, screen_position.y);

                match text_box_item._animation_state {
                    TextBoxAnimationState::None => {
                        ui_component.set_opacity(0.0);
                        text_box_item.set_animation_state(TextBoxAnimationState::Growing);
                    }
                    TextBoxAnimationState::Growing => {
                        let opacity =
                            (text_box_item._animation_timer / TEXT_BOX_ANIMATION_DURATION).min(1.0);
                        ui_component.set_opacity(opacity);
                        if 1.0 <= opacity {
                            text_box_item.set_animation_state(TextBoxAnimationState::Idle);
                        }
                        text_box_item._animation_timer += delta_time;
                    }
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
                        let opacity = 1.0
                            - (text_box_item._animation_timer / TEXT_BOX_ANIMATION_DURATION)
                                .min(1.0);
                        ui_component.set_opacity(opacity);
                        if opacity <= 0.0 {
                            is_enable_text_box = false;
                        }
                        text_box_item._animation_timer += delta_time;
                    }
                }
            }

            if !is_enable_text_box {
                remove_items.push(key);
            }
        }

        for character_name in remove_items {
            if let Some(item) = self._text_box_items.remove(&character_name) {
                ptr_as_mut(self._root_widget).remove_widget(item._layout_widget);
            }
        }
    }
}
