use crate::game_module::widgets::item_bar::{ItemSelectionWidget, ItemWidget};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

impl<'a> ItemSelectionWidget<'a> {
    pub fn get_item_index(&self) -> usize {
        self._item_index
    }

    pub fn update_selected_item_widget(&mut self, item_index: usize, item_widget: Option<&ItemWidget<'a>>) {
        self._item_index = item_index;
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        if let Some(item_widget) = item_widget {
            let item_ui_component = ptr_as_ref(item_widget._widget).get_ui_component();
            let item_render_area = item_ui_component.get_render_area();
            let center_x_scaled = (item_render_area.x + item_render_area.z) * 0.5;
            let center_y_scaled = (item_render_area.y + item_render_area.w) * 0.5;

            let parent_ui_component = ptr_as_ref(ui_component.get_parent());
            let parent_contents_area = &parent_ui_component._contents_area;

            let dpi_scale = if ui_component.get_enable_dpi_scale() {
                rust_engine_3d::scene::ui::get_global_dpi_scale()
            } else {
                1.0
            };

            let relative_center_x = (center_x_scaled - parent_contents_area.x) / dpi_scale;
            let relative_center_y = (center_y_scaled - parent_contents_area.y) / dpi_scale;

            ui_component.set_pivot_preset(rust_engine_3d::scene::ui::PIVOT_CENTER);
            ui_component.set_pos(relative_center_x, relative_center_y);
            ui_component.set_visible(true);
        } else {
            ui_component.set_visible(false);
        }
    }
}
