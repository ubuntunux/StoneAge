use crate::game_module::game_service_locator::get_game_resources;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault, PIVOT_BOTTOM_LEFT, PIVOT_TOP_LEFT};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub const ITEM_INFO_TITLE_FONT_SIZE: f32 = 30.0;
pub const ITEM_INFO_DESC_FONT_SIZE: f32 = 22.0;

pub struct ItemInfoWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _title_lbl: Rc<WidgetDefault<'a>>,
    pub _desc_lbl: Rc<WidgetDefault<'a>>,
}

impl<'a> ItemInfoWidget<'a> {
    pub fn create_item_info_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<ItemInfoWidget<'a>> {
        let layer = UIManager::create_widget("inv_item_info_popup", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_pivot_preset(PIVOT_TOP_LEFT);
        ui_component.set_size(0.0, 0.0);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(20, 25, 35, 255));
        ui_component.set_border(2.0);
        ui_component.set_border_color(get_color32(180, 200, 240, 255));
        ui_component.set_round(8.0);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_visible(false);

        let title_lbl = UIManager::create_widget("inv_item_info_title", UIWidgetTypes::Default);
        let ui = ptr_as_mut(title_lbl.as_ref()).get_ui_component_mut();
        ui.set_size(0.0, 0.0);
        ui.set_font_size(ITEM_INFO_TITLE_FONT_SIZE);
        ui.set_font_color(get_color32(255, 255, 180, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_margin(4.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_expandable(true);

        let desc_lbl = UIManager::create_widget("inv_item_info_desc", UIWidgetTypes::Default);
        let ui = ptr_as_mut(desc_lbl.as_ref()).get_ui_component_mut();
        ui.set_size(0.0, 0.0);
        ui.set_font_size(ITEM_INFO_DESC_FONT_SIZE);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_margin(4.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_expandable(true);

        ptr_as_mut(layer.as_ref()).add_widget(&title_lbl);
        ptr_as_mut(layer.as_ref()).add_widget(&desc_lbl);

        parent_widget.add_widget(&layer);

        Box::new(ItemInfoWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _title_lbl: title_lbl,
            _desc_lbl: desc_lbl,
        })
    }

    pub fn show_item_info(
        &mut self,
        item_data_name: &str,
        item_name: &str,
        item_count: usize,
        slot_widget: &WidgetDefault<'a>,
    ) {
        let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
        let slot_ui = slot_widget.get_ui_component();
        let slot_area = slot_ui.get_ui_area();
        let parent_area = ptr_as_ref(self._parent_widget).get_ui_component().get_ui_area();

        let popup_x = (slot_area.x - parent_area.x) / dpi_scale;
        let popup_y = (slot_area.y - parent_area.y) / dpi_scale;

        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_pos(popup_x, popup_y);
        layer_ui.set_visible(true);

        let title_ui = ptr_as_mut(self._title_lbl.as_ref()).get_ui_component_mut();
        let display_title = if item_count > 1 {
            format!("{} x{}", item_name, item_count)
        } else {
            item_name.to_string()
        };
        title_ui.set_text(&display_title);

        let desc_ui = ptr_as_mut(self._desc_lbl.as_ref()).get_ui_component_mut();
        let item_desc = Self::get_item_description_from_resource(item_data_name);
        desc_ui.set_text(&item_desc);
    }

    pub fn hide_item_info(&mut self) {
        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_visible(false);
    }

    pub fn get_item_description_from_resource(item_code: &str) -> String {
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            let desc = resources.get_item_data(item_code).borrow()._description.clone();
            if !desc.is_empty() {
                return desc;
            }
        }
        item_code.to_string()
    }
}
