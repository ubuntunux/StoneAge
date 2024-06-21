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
        material_instance_name: &str
    ) -> Box<ImageLayout<'a>> {
        let image_widget = UIManager::create_widget(material_instance_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(image_widget.as_ref()).get_ui_component_mut();
        ui_component.set_material_instance(&game_resources.get_engine_resources().get_material_instance_data(material_instance_name));
        ui_component.set_size_hint_x(Some(0.5));
        ui_component.set_size_hint_y(Some(0.5));
        root_widget.add_widget(&image_widget);

        Box::new(ImageLayout {
            _image_layout: image_widget.clone(),
            _opacity: 1.0
        })
    }
}
