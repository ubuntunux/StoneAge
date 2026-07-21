use crate::game_module::game_constants::MATERIAL_MOUSE_DEFAULT;
use crate::game_module::game_resource::GameResources;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

pub struct CrossHairWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
}

impl<'a> CrossHairWidget<'a> {
    pub fn create_cross_hair(
        root_widget: &mut WidgetDefault<'a>,
        game_resources: &GameResources<'a>,
    ) -> CrossHairWidget<'a> {
        let material_instance =
            game_resources.get_engine_resources().get_material_instance_data(MATERIAL_MOUSE_DEFAULT);
        let cross_hair_widget = UIManager::create_widget("cross_hair_widget", UIWidgetTypes::Default);
        let cross_hair_widget_ptr = ptr_as_mut(cross_hair_widget.as_ref());
        let ui_component = ptr_as_mut(cross_hair_widget.as_ref()).get_ui_component_mut();
        ui_component.set_material_instance(Some(material_instance.clone()));
        ui_component.set_size(100.0, 100.0);
        root_widget.add_widget(&cross_hair_widget);

        CrossHairWidget {
            _widget: cross_hair_widget_ptr,
        }
    }

    pub fn update_cross_hair_visible(&mut self, visible: bool) {
        ptr_as_mut(self._widget).get_ui_component_mut().set_visible(visible);
    }

    pub fn get_cross_hair_visible(&self) -> bool {
        ptr_as_ref(self._widget).get_ui_component().get_visible()
    }

    pub fn update_cross_hair(&mut self, pos: &Vector2<i32>) {
        let cross_hair_widget = ptr_as_mut(self._widget);
        let ui_component = cross_hair_widget.get_ui_component_mut();
        ui_component.set_pos_with_dpi(pos.x as f32, pos.y as f32);
    }
}
