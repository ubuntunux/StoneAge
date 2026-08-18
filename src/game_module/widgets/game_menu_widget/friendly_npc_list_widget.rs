use crate::game_module::widgets::game_menu_widget::character_list_helper::{
    collect_all_characters, get_affinity_tier,
};
use nalgebra::Vector2;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub struct FriendlyNpcListWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _is_opened_friendly_npc_list_widget: bool,
}

impl<'a> FriendlyNpcListWidget<'a> {
    pub fn create_friendly_npc_list_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<FriendlyNpcListWidget<'a>> {
        let layer = UIManager::create_widget("friendly_npc_list_widget", UIWidgetTypes::Default);
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
        ui_component.set_color(get_color32(40, 45, 55, 220));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);

        // Title Header
        let title_widget = UIManager::create_widget("friendly_npc_title", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(300.0, 30.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text("Friendly NPCs");
        ui_component.set_font_size(25.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&title_widget);

        // List Container for Cards
        let list_container = UIManager::create_widget("friendly_npc_list_container", UIWidgetTypes::Default);
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
        layer_mut.add_widget(&list_container);

        Box::new(FriendlyNpcListWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _list_container: list_container,
            _is_opened_friendly_npc_list_widget: false,
        })
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_friendly_npc_list_widget(&self) -> bool {
        self._is_opened_friendly_npc_list_widget
    }

    pub fn open_friendly_npc_list_widget(&mut self) {
        if !self._is_opened_friendly_npc_list_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._is_opened_friendly_npc_list_widget = true;
            self.refresh_friendly_npc_list();
        }
    }

    pub fn close_friendly_npc_list_widget(&mut self) {
        if self._is_opened_friendly_npc_list_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened_friendly_npc_list_widget = false;
        }
    }

    pub fn refresh_friendly_npc_list(&mut self) {
        let container_mut = ptr_as_mut(self._list_container.as_ref());
        container_mut.clear_widgets();

        let all_characters = collect_all_characters();
        let mut npc_list: Vec<_> = all_characters
            .into_iter()
            .filter(|c| c.is_civilian)
            .collect();

        // Sort by intimacy descending, then by name
        npc_list.sort_by(|a, b| {
            b.intimacy
                .partial_cmp(&a.intimacy)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.name.cmp(&b.name))
        });

        let npc_count = npc_list.len();

        for item in npc_list.iter() {
            let card_widget = UIManager::create_widget("friendly_npc_card", UIWidgetTypes::Default);
            let card_mut = ptr_as_mut(card_widget.as_ref());
            let ui_comp = card_mut.get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(620.0, 48.0);
            ui_comp.set_margin(4.0);
            ui_comp.set_padding(8.0);
            ui_comp.set_color(get_color32(50, 60, 75, 240));
            ui_comp.set_border_color(get_color32(90, 110, 140, 255));
            ui_comp.set_round(5.0);

            let status_str = if item.is_alive { "ALIVE" } else { "DEAD" };
            let status_color = if item.is_alive {
                get_color32(100, 255, 100, 255)
            } else {
                get_color32(255, 100, 100, 255)
            };

            let affinity_tier = get_affinity_tier(item.intimacy);
            let tier_str = affinity_tier.get_display_name();

            let info_text = format!(
                "[{}] [{}] {} ({}) | HP: {}/{} | Intimacy: {:.0} | Pos: ({:.0}, {:.0})",
                status_str,
                item.scene_display_name,
                item.name,
                tier_str,
                item.hp,
                item.max_hp,
                item.intimacy,
                item.position.x,
                item.position.z
            );

            ui_comp.set_text(&info_text);
            ui_comp.set_font_size(20.0);
            ui_comp.set_font_color(status_color);

            container_mut.add_widget(&card_widget);
        }

        if npc_count == 0 {
            let empty_widget = UIManager::create_widget("friendly_npc_empty_info", UIWidgetTypes::Default);
            let ui_comp = ptr_as_mut(empty_widget.as_ref()).get_ui_component_mut();
            ui_comp.set_size(400.0, 60.0);
            ui_comp.set_margin(20.0);
            ui_comp.set_text("No friendly NPCs found.");
            ui_comp.set_font_size(24.0);
            ui_comp.set_font_color(get_color32(180, 180, 180, 255));
            container_mut.add_widget(&empty_widget);
        }
    }

    pub fn update_friendly_npc_list_widget(
        &mut self,
        _joystick_input_data: &JoystickInputData,
        _keyboard_input_data: &KeyboardInputData,
    ) {
        if self._is_opened_friendly_npc_list_widget {
            self.refresh_friendly_npc_list();
        }
    }
}
