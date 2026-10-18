use crate::game_module::widgets::game_menu_widget::character_list_helper::{
    collect_all_characters, create_character_card, create_character_empty_card, create_character_list_panel, status_str,
};
use nalgebra::Vector2;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::WidgetDefault;
use rust_engine_3d::utilities::system::ptr_as_mut;
use std::rc::Rc;

pub struct TamingListWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _is_opened_taming_list_widget: bool,
}

impl<'a> TamingListWidget<'a> {
    pub fn create_taming_list_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<TamingListWidget<'a>> {
        let (layer, list_container) =
            create_character_list_panel("taming_list_widget", "taming_list_container", "Tamed Companions");

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

        let all_characters = collect_all_characters();
        let mut tamed_list: Vec<_> = all_characters.into_iter().filter(|c| c._is_tamed).collect();

        tamed_list.sort_by(|a, b| {
            b._is_alive
                .cmp(&a._is_alive)
                .then_with(|| b._intimacy.partial_cmp(&a._intimacy).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| a._name.cmp(&b._name))
        });

        for item in tamed_list.iter() {
            let status = status_str(item._is_alive);

            let info_text = format!(
                "[{}] [{}] {} | HP: {}/{} | Intimacy: {:.0} | Pos: ({:.0}, {:.0})",
                status,
                item._scene_display_name,
                item._name,
                item._hp,
                item._max_hp,
                item._intimacy,
                item._position.x,
                item._position.z
            );

            let card = create_character_card("tamed_card", &info_text, item._is_alive);
            container_mut.add_widget(&card);
        }

        if tamed_list.is_empty() {
            let empty_widget = create_character_empty_card("taming_empty_info", "No tamed companions found.");
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
