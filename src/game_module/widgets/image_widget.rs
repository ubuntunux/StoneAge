use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_resource::GameResources;

pub struct ImageLayout<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _image_layout: Rc<WidgetDefault<'a>>,
    pub _material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
    pub _next_material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
    pub _initial_fade_time: f32,
    pub _fade_time: f32,
    pub _opacity: f32,
    pub _prev_opacity: f32,
    pub _fadein_opacity: f32,
    pub _fadeout_opacity: f32,
    pub _image_brightness: f32,
    pub _prev_image_brightness: f32,
    pub _fadein_image_brightness: f32,
    pub _fadeout_image_brightness: f32,
    pub _image_aspect: f32,
    pub _next_image_aspect: f32,
    pub _image_size_hint: f32,
    pub _window_size: Vector2<i32>,
}

// Image layout
impl<'a> ImageLayout<'a> {
    pub fn create_image_layout(
        root_widget: &mut WidgetDefault<'a>,
        window_size: &Vector2<i32>,
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
        let ui_component = ptr_as_mut(image_widget.as_ref()).get_ui_component_mut();
        ui_component.set_color(get_color32(0, 0, 0, 255));
        ui_component.set_visible(false);
        background_layout_mut.add_widget(&image_widget);

        Box::new(ImageLayout {
            _background_layout: background_layout.clone(),
            _image_layout: image_widget.clone(),
            _material_instance: None,
            _next_material_instance: None,
            _initial_fade_time: 0.0,
            _fade_time: 0.0,
            _opacity: 0.0,
            _prev_opacity: 0.0,
            _fadein_opacity: 0.0,
            _fadeout_opacity: 0.0,
            _image_brightness: 0.0,
            _prev_image_brightness: 0.0,
            _fadein_image_brightness: 1.0,
            _fadeout_image_brightness: 0.0,
            _image_aspect: 1.0,
            _next_image_aspect: 1.0,
            _image_size_hint: 0.9,
            _window_size: window_size.clone(),
        })
    }

    pub fn set_game_image(
        &mut self,
        game_resources: &GameResources<'a>,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        fade_time: f32,
    ) {
        if self._material_instance.is_none() && material_instance.is_none() {
            return;
        }

        if material_instance.is_some() {
            let material_instance_refcell = material_instance.as_ref().unwrap();
            let material_instance_ref = material_instance_refcell.borrow();
            let texture_parameter = material_instance_ref._material_parameters.get("texture_color").unwrap();
            let texture_name = texture_parameter.as_str().unwrap();
            let texture = game_resources.get_engine_resources().get_texture_data(texture_name);
            self._next_image_aspect = texture.borrow()._image_width as f32 / texture.borrow()._image_height as f32;
            self._fadeout_opacity = 1.0;
            self._fadeout_image_brightness = 0.0;
            self._fadein_opacity = 1.0;
            self._fadein_image_brightness = 1.0;
        } else {
            self._next_image_aspect = 1.0;
            self._fadeout_opacity = 1.0;
            self._fadeout_image_brightness = 0.0;
            self._fadein_opacity = 0.0;
            self._fadein_image_brightness = 0.0;
        }
        self._next_material_instance = material_instance;

        let progress = self.get_progress();
        self._initial_fade_time = fade_time;
        self._fade_time = fade_time * if progress <= 0.5 { progress } else { 1.0 - progress };

        self._prev_opacity = self._opacity;
        self._prev_image_brightness = self._image_brightness;

        self.changed_window_size(&self._window_size.clone());

        if fade_time == 0.0 {
            self.update_game_image(0.0, true);
        }
    }

    pub fn change_game_image(&mut self) {
        let image_widget = ptr_as_mut(self._image_layout.as_ref());
        let ui_component = image_widget.get_ui_component_mut();
        ui_component.set_material_instance(self._next_material_instance.clone());
        ui_component.set_visible(self._next_material_instance.is_some());
        self._material_instance = self._next_material_instance.clone();
        self._next_material_instance = None;
        self._image_aspect = self._next_image_aspect;
        self._prev_opacity = self._opacity;
        self._prev_image_brightness = self._image_brightness;

        self.changed_window_size(&self._window_size.clone());
    }

    pub fn is_done_game_image_progress(&self) -> bool {
        self.get_progress() == 1.0
    }

    pub fn get_progress(&self) -> f32 {
        if 0.0 != self._initial_fade_time {
            1.0_f32.min(self._fade_time / self._initial_fade_time)
        } else {
            1.0
        }
    }

    pub fn is_visible(&self) -> bool {
        self._background_layout.as_ref().get_ui_component()._visible
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let window_aspect: f32 = window_size.x as f32 / window_size.y as f32;
        let image_widget = ptr_as_mut(self._image_layout.as_ref());
        let ui_component = image_widget.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(self._image_aspect / window_aspect * self._image_size_hint));
        ui_component.set_size_hint_y(Some(self._image_size_hint));
    }

    pub fn update_game_image(&mut self, delta_time: f64, force: bool) {
        let prev_progress = self.get_progress();
        if prev_progress < 1.0 || force {
            // progress
            self._fade_time += delta_time as f32;
            let progress = self.get_progress();
            if prev_progress <= 0.5 && 0.5 < progress || force {
                self.change_game_image();
            }

            // calc opacity
            if progress == 1.0 {
                self._opacity = self._fadein_opacity;
                self._image_brightness = self._fadein_image_brightness;
            } else if progress <= 0.5 {
                self._opacity = math::lerp(self._prev_opacity, self._fadeout_opacity, progress * 2.0);
                self._image_brightness = math::lerp(self._prev_image_brightness, self._fadeout_image_brightness, progress * 2.0);
            } else {
                self._opacity = math::lerp(self._prev_opacity, self._fadein_opacity, (progress - 0.5) * 2.0);
                self._image_brightness = math::lerp(self._prev_image_brightness, self._fadein_image_brightness, (progress - 0.5) * 2.0);
            }

            // set opacity
            let background_layout = ptr_as_mut(self._background_layout.as_ref());
            let ui_component = background_layout.get_ui_component_mut();
            ui_component.set_opacity(self._opacity);
            ui_component.set_visible(0.0 < self._opacity);

            let image_widget = ptr_as_mut(self._image_layout.as_ref());
            let ui_component = image_widget.get_ui_component_mut();
            let r = (self._image_brightness * 255.0) as u32;
            ui_component.set_color(get_color32(r, r, r, 255));
        }
    }
}
