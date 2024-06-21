use std::rc::Rc;
use rust_engine_3d::scene::ui::{UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::ptr_as_mut;
use crate::game_module::game_resource::GameResources;

pub struct ImageLayout<'a> {
    pub _image_layout: Rc<WidgetDefault<'a>>,
    pub _opacity: f32
}

// Image layout
impl<'a> ImageLayout<'a> {
    pub fn create_image_layout(
        game_resources: &GameResources<'a>,
        root_widget: &mut WidgetDefault<'a>,
    ) -> ImageLayout<'a> {
        let image_widget = UIManager::create_widget("image_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(image_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_material_instance(&game_resources.get_engine_resources().get_material_instance_data("ui/intro_image"));
        root_widget.add_widget(&image_widget);

        ImageLayout {
            _image_layout: image_widget.clone(),
            _opacity: 1.0
        }
    }
}
