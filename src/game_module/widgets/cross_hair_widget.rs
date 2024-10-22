use nalgebra::Vector2;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};

pub struct CrossHairWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
}

impl<'a> CrossHairWidget<'a> {
    pub fn create_cross_hair(root_widget: &mut WidgetDefault<'a>, material_instance: &RcRefCell<MaterialInstanceData<'a>>) -> CrossHairWidget<'a> {
        let cross_hair_widget = UIManager::create_widget("cross_hair_widget", UIWidgetTypes::Default);
        let cross_hair_widget_ptr = ptr_as_mut(cross_hair_widget.as_ref());
        let ui_component = ptr_as_mut(cross_hair_widget.as_ref()).get_ui_component_mut();
        ui_component.set_material_instance(&material_instance);
        ui_component.set_size(50.0, 50.0);
        root_widget.add_widget(&cross_hair_widget);

        CrossHairWidget {
            _widget: cross_hair_widget_ptr,
        }
    }

    pub fn update_cross_hair_visible(&mut self, visible: bool) {
        ptr_as_mut(self._widget).get_ui_component_mut().set_visible(visible);
    }

    pub fn update_cross_hair(&mut self, pos: &Vector2<i32>) {
        let cross_hair_widget = ptr_as_mut(self._widget);
        let ui_component = cross_hair_widget.get_ui_component_mut();
        ui_component.set_pos_x(pos.x as f32);
        ui_component.set_pos_y(pos.y as f32);
    }
}
