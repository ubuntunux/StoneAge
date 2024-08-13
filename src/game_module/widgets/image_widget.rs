use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_resource::GameResources;

pub struct ImageLayout<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _image_layout: Rc<WidgetDefault<'a>>,
    pub _start_fadeout: bool,
    pub _initial_fadeout_time: f32,
    pub _fadeout_time: f32
}

// Image layout
impl<'a> ImageLayout<'a> {
    pub fn create_image_layout(
        root_widget: &mut WidgetDefault<'a>,
        material_instance_name: &str
    ) -> Box<ImageLayout<'a>> {
        // background layout
        let background_layout = UIManager::create_widget("background image layout", UIWidgetTypes::Default);
        let background_layout_mut = ptr_as_mut(background_layout.as_ref());
        let ui_component = ptr_as_mut(background_layout.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_color(get_color32(0, 0, 0, 255));
        ui_component.set_visible(false);
        root_widget.add_widget(&background_layout);

        // image layout
        let image_widget = UIManager::create_widget(material_instance_name, UIWidgetTypes::Default);
        background_layout_mut.add_widget(&image_widget);

        Box::new(ImageLayout {
            _background_layout: background_layout.clone(),
            _image_layout: image_widget.clone(),
            _start_fadeout: false,
            _initial_fadeout_time: 0.0,
            _fadeout_time: 0.0,
        })
    }

    pub fn set_material_instance(
        &mut self,
        game_resources: &GameResources<'a>,
        window_size: &Vector2<i32>,
        material_instance_name: &str,
        fadeout_time: f32,
    ) {
        // texture aspect
        let material_instance = game_resources.get_engine_resources().get_material_instance_data(material_instance_name);
        let material_instance_ref = material_instance.borrow();
        let texture_name = material_instance_ref._material_parameters.get("texture_color").unwrap().as_str().unwrap();
        let texture = game_resources.get_engine_resources().get_texture_data(texture_name);
        let window_aspect: f32 = window_size.x as f32 / window_size.y as f32;
        let image_aspect: f32 = texture.borrow()._image_width as f32 / texture.borrow()._image_height as f32;

        // image layout
        let image_widget = ptr_as_mut(self._image_layout.as_ref());
        let ui_component = image_widget.get_ui_component_mut();
        ui_component.set_material_instance(&material_instance);
        let image_size_hint = 0.9;
        ui_component.set_size_hint_x(Some(image_aspect / window_aspect * image_size_hint));
        ui_component.set_size_hint_y(Some(image_size_hint));

        // background layout
        let background_layout = ptr_as_mut(self._background_layout.as_ref());
        let ui_component = background_layout.get_ui_component_mut();
        ui_component.set_visible(true);
        ui_component.set_opacity(1.0);

        self._initial_fadeout_time = fadeout_time;
        self._fadeout_time = fadeout_time;
        self._start_fadeout = false;
    }

    pub fn is_visible(&self) -> bool {
        self._background_layout.as_ref().get_ui_component()._visible
    }

    pub fn start_fadeout(&mut self, start_fadeout: bool) {
        self._start_fadeout = start_fadeout;
    }

    pub fn update_image_layout(&mut self, delta_time: f64) {
        if self._start_fadeout {
            let opacity = if 0.0 < self._initial_fadeout_time {
                0f32.max(self._fadeout_time / self._initial_fadeout_time)
            } else {
                0.0
            };

            let background_layout = ptr_as_mut(self._background_layout.as_ref());
            let ui_component = background_layout.get_ui_component_mut();
            ui_component.set_opacity(opacity);

            if opacity <= 0.0 {
                ui_component.set_visible(false);
                self._start_fadeout = false;
            }

            self._fadeout_time -= delta_time as f32;
        }
    }
}
