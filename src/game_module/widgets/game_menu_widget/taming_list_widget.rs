use crate::game_module::game_service_locator::get_character_manager;
use nalgebra::Vector2;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub struct TamingListWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _is_opened_taming_list_widget: bool,
}

impl<'a> TamingListWidget<'a> {
    pub fn create_taming_list_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<TamingListWidget<'a>> {
        let layer = UIManager::create_widget("taming_list_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(40, 40, 50, 220));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);

        // Title Header
        let title_widget = UIManager::create_widget("taming_title", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(300.0, 30.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text("Tamed Companions");
        ui_component.set_font_size(25.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&title_widget);

        // List Container for Cards
        let list_container = UIManager::create_widget("taming_list_container", UIWidgetTypes::Default);
        let list_container_mut = ptr_as_mut(list_container.as_ref());
        let ui_component = list_container_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_padding(5.0);
        layer_mut.add_widget(&list_container);

        Box::new(TamingListWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _list_container: list_container,
            _is_opened_taming_list_widget: false,
        })
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_taming_list_widget(&self) -> bool {
        self._is_opened_taming_list_widget
    }

    pub fn open_taming_list_widget(&mut self) {
        if !self._is_opened_taming_list_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._is_opened_taming_list_widget = true;
            self.refresh_taming_list();
        }
    }

    pub fn close_taming_list_widget(&mut self) {
        if self._is_opened_taming_list_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened_taming_list_widget = false;
        }
    }

    pub fn refresh_taming_list(&mut self) {
        let container_mut = ptr_as_mut(self._list_container.as_ref());
        container_mut.clear_widgets();

        let character_manager = get_character_manager();
        let characters = character_manager.get_characters();

        let mut tamed_count = 0;

        for (_id, char_ref) in characters.iter() {
            let char_borrow = char_ref.borrow();
            if char_borrow.is_tamed() {
                tamed_count += 1;

                let card_widget = UIManager::create_widget("tamed_card", UIWidgetTypes::Default);
                let card_mut = ptr_as_mut(card_widget.as_ref());
                let ui_comp = card_mut.get_ui_component_mut();
                ui_comp.set_layout_type(UILayoutType::BoxLayout);
                ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
                ui_comp.set_halign(HorizontalAlign::CENTER);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_size(560.0, 48.0);
                ui_comp.set_margin(4.0);
                ui_comp.set_padding(8.0);
                ui_comp.set_color(get_color32(60, 60, 75, 240));
                ui_comp.set_border_color(get_color32(100, 100, 130, 255));
                ui_comp.set_round(5.0);

                let status_str = if char_borrow.is_alive() { "ALIVE" } else { "DEAD" };
                let status_color = if char_borrow.is_alive() {
                    get_color32(100, 255, 100, 255)
                } else {
                    get_color32(255, 100, 100, 255)
                };

                let pos = char_borrow.get_position();
                let info_text = format!(
                    "[{}] {} | HP: {}/{} | Intimacy: {:.0} | Pos: ({:.0}, {:.0})",
                    status_str,
                    char_borrow._character_name,
                    char_borrow._character_stats.get_hp(),
                    char_borrow._character_stats.get_max_hp(),
                    char_borrow.get_intimacy(),
                    pos.x,
                    pos.z
                );

                ui_comp.set_text(&info_text);
                ui_comp.set_font_size(20.0);
                ui_comp.set_font_color(status_color);

                container_mut.add_widget(&card_widget);
            }
        }

        if tamed_count == 0 {
            let empty_widget = UIManager::create_widget("taming_empty_info", UIWidgetTypes::Default);
            let ui_comp = ptr_as_mut(empty_widget.as_ref()).get_ui_component_mut();
            ui_comp.set_size(400.0, 60.0);
            ui_comp.set_margin(20.0);
            ui_comp.set_text("No tamed companions found.");
            ui_comp.set_font_size(24.0);
            ui_comp.set_font_color(get_color32(180, 180, 180, 255));
            container_mut.add_widget(&empty_widget);
        }
    }

    pub fn update_taming_list_widget(
        &mut self,
        _joystick_input_data: &JoystickInputData,
        _keyboard_input_data: &KeyboardInputData,
    ) {
        if self._is_opened_taming_list_widget {
            self.refresh_taming_list();
        }
    }
}
