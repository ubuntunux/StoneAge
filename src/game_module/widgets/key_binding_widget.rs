use crate::game_module::game_controller::KeyBindingType;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::WidgetDefault;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use std::collections::HashMap;
use std::rc::Rc;

pub const KEY_BINDING_UI_SIZE: f32 = 50.0;
pub const KEY_BINDING_FONT_SIZE: f32 = 30.0;
pub const KEY_BINDING_TEXT_MARGIN: f32 = 20.0;
pub const KEY_BINDING_ICON_MARGIN: f32 = -14.0;

pub struct KeyBindingWidget<'a> {
    pub _key_binding_type: KeyBindingType,
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _binding_name_widget: *const WidgetDefault<'a>,
    pub _binding_icon_widgets: Vec<*const WidgetDefault<'a>>,
    pub _key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    pub _joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
}

impl<'a> KeyBindingWidget<'a> {
    pub fn update_icon_material_instance(&mut self, is_keyboard_input_mode: bool) {
        let binding_icons = if is_keyboard_input_mode {
            &self._key_binding_icons
        } else {
            &self._joystick_binding_icons
        };

        ptr_as_mut(self._layout_widget)
            ._ui_component
            .set_visible(!binding_icons.is_empty());

        for (index, icon_widget) in self._binding_icon_widgets.iter().enumerate() {
            if index < binding_icons.len() {
                ptr_as_mut(*icon_widget)._ui_component.set_visible(true);
                ptr_as_mut(*icon_widget)
                    ._ui_component
                    .set_material_instance(binding_icons.get(index).cloned());
            } else {
                ptr_as_mut(*icon_widget)._ui_component.set_visible(false);
            }
        }
    }
}

#[derive(Default)]
pub struct KeyBindingWidgetMap<'a> {
    pub _key_binding_widget_map: HashMap<KeyBindingType, KeyBindingWidget<'a>>,
}

impl<'a> KeyBindingWidgetMap<'a> {
    pub fn get_key_binding_widget_mut(
        &mut self,
        key_binding_type: KeyBindingType,
    ) -> &mut KeyBindingWidget<'a> {
        self._key_binding_widget_map
            .get_mut(&key_binding_type)
            .unwrap()
    }

    pub fn get_key_binding_widget(
        &self,
        key_binding_type: KeyBindingType,
    ) -> &KeyBindingWidget<'a> {
        self._key_binding_widget_map.get(&key_binding_type).unwrap()
    }

    pub fn register_key_binding_widget(&mut self, key_binding_widget: KeyBindingWidget<'a>) {
        self._key_binding_widget_map
            .insert(key_binding_widget._key_binding_type, key_binding_widget);
    }

    pub fn update_key_binding_widgets(&mut self, is_keyboard_input_mode: bool) {
        for (_, key_binding_widget) in self._key_binding_widget_map.iter_mut() {
            key_binding_widget.update_icon_material_instance(is_keyboard_input_mode);
        }
    }
}

#[derive(Default)]
pub struct KeyBindingWidgetManager<'a> {
    pub _is_keyboard_input_mode: Option<bool>,
    pub _key_binding_widget_maps: Vec<Rc<KeyBindingWidgetMap<'a>>>,
}

impl<'a> KeyBindingWidgetManager<'a> {
    pub fn register_key_binding_widget_map(
        &mut self,
        key_binding_widget_map: &Rc<KeyBindingWidgetMap<'a>>,
    ) {
        self._key_binding_widget_maps
            .push(key_binding_widget_map.clone());
    }

    pub fn update_key_binding_widget_manager(&mut self, is_keyboard_input_mode: bool) {
        if self._is_keyboard_input_mode.is_none()
            || self._is_keyboard_input_mode.unwrap() != is_keyboard_input_mode
        {
            for key_binding_widget_map in self._key_binding_widget_maps.iter_mut() {
                ptr_as_mut(key_binding_widget_map.as_ref())
                    .update_key_binding_widgets(is_keyboard_input_mode);
            }
            self._is_keyboard_input_mode = Some(is_keyboard_input_mode);
        }
    }
}
