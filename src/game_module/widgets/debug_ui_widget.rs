use crate::game_module::game_service_locator::get_character_manager;
use rust_engine_3d::core::engine_service_locator::get_scene_manager;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, UIComponentInstance, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub struct DebugUIWidget<'a> {
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _debug_ui_widgets: Vec<*const WidgetDefault<'a>>,
}

impl<'a> DebugUIWidget<'a> {
    pub fn create_debug_ui_widget(root_widget: &mut WidgetDefault<'a>) -> Box<DebugUIWidget<'a>> {
        let ui_layout = UIManager::create_widget("DebugUIWidget", UIWidgetTypes::Default);
        let ui_layout_mut: &mut WidgetDefault = ptr_as_mut(ui_layout.as_ref());
        let ui_component: &mut UIComponentInstance = ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);
        ui_component.set_enable(false);
        root_widget.add_widget(&ui_layout);

        Box::new(DebugUIWidget {
            _layer: ui_layout,
            _debug_ui_widgets: Vec::new(),
        })
    }

    pub fn set_enable(&self, enable: bool) {
        ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(enable);
    }

    pub fn update_debug_ui_widget(&mut self) {
        if !self._layer.get_ui_component().get_enable() {
            return;
        }

        let main_camera = get_scene_manager().get_main_camera();
        let characters = get_character_manager().get_characters();
        let character_count = characters.len();
        let debug_ui_count = self._debug_ui_widgets.len();
        if debug_ui_count < characters.len() {
            for _ in debug_ui_count..character_count {
                let ui_layout = UIManager::create_widget("actor position", UIWidgetTypes::Default);
                let ui_layout_mut: &mut WidgetDefault = ptr_as_mut(ui_layout.as_ref());
                let ui_component: &mut UIComponentInstance = ui_layout_mut.get_ui_component_mut();
                ui_component.set_expandable(true);
                ui_component.set_size_y(20.0);
                ui_component.set_halign(HorizontalAlign::LEFT);
                ui_component.set_valign(VerticalAlign::TOP);
                ui_component.set_font_color(get_color32(255, 255, 255, 255));
                ui_component.set_color(get_color32(255, 255, 255, 0));
                ptr_as_mut(self._layer.as_ref()).add_widget(&ui_layout);
                self._debug_ui_widgets.push(ui_layout.as_ref());
            }
        } else {
            for i in character_count..debug_ui_count {
                let widget = ptr_as_mut(self._debug_ui_widgets[i]);
                widget._ui_component.set_visible(false);
            }
        }

        for (i, character) in characters.iter().enumerate() {
            let character = character.1.borrow();
            let position = character.get_position();
            let screen_position =
                main_camera.convert_world_to_screen(position, true) / rust_engine_3d::scene::ui::get_global_dpi_scale();
            let ui_component = ptr_as_mut(self._debug_ui_widgets[i]).get_ui_component_mut();
            let debug_info = character.get_debug_info();
            ui_component.set_text(debug_info.as_str());
            ui_component.set_pos(screen_position.x, screen_position.y);
            ui_component.set_visible(true);
        }
    }
}
