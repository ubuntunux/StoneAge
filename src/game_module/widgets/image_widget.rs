use crate::game_module::game_service_locator::{get_game_scene_manager, is_application_valid};
use nalgebra::Vector2;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

const HALF_FADE_PROGRESS: f32 = 0.5;
const FULL_FADE_OPACITY: f32 = 1.0;

pub struct ImageLayout<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _image_layout: Rc<WidgetDefault<'a>>,
    pub _material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
    pub _next_material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
    pub _initial_fade_time: f32,
    pub _auto_fade_inout: bool,
    pub _done_manual_fade_out: bool,
    pub _fade_time: f32,
    pub _fade_speed: f32,
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
        material_instance_name: &str,
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
            _auto_fade_inout: false,
            _done_manual_fade_out: false,
            _fade_time: 0.0,
            _fade_speed: 1.0,
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
            _image_size_hint: 1.0,
            _window_size: *window_size,
        })
    }

    pub fn set_game_image_fade_speed(&mut self, fade_speed: f32) {
        self._fade_speed = fade_speed;
    }

    pub fn set_game_image(
        &mut self,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        fade_time: f32,
        auto_fade_inout: bool,
    ) {
        if let Some(ref material_instance_refcell) = material_instance {
            let texture_name = material_instance_refcell
                .borrow()
                ._material_parameters
                .get("texture_color")
                .and_then(|param| param.as_str())
                .map(|s| s.to_string());

            let aspect = if let Some(ref name) = texture_name {
                let engine_resources = get_engine_resources();
                if let Some(texture_rc) = engine_resources.get_loaded_texture_data(name) {
                    let tex_borrow = texture_rc.borrow();
                    if tex_borrow._image_height > 0 {
                        tex_borrow._image_width as f32 / tex_borrow._image_height as f32
                    } else {
                        1.0
                    }
                } else {
                    engine_resources.request_texture_data(name);
                    1.0
                }
            } else {
                1.0
            };
            self._next_image_aspect = aspect;
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

        let progress = self.get_progress(self._fade_time);
        self._initial_fade_time = fade_time;
        self._auto_fade_inout = auto_fade_inout;
        self._fade_time = fade_time * if progress <= 0.5 { progress } else { 1.0 - progress };

        self._prev_opacity = self._opacity;
        self._prev_image_brightness = self._image_brightness;

        if fade_time == 0.0 {
            self.update_game_image(0.0, true);
        }
    }

    pub fn change_game_image(&mut self) {
        self._material_instance = self._next_material_instance.clone();
        self._next_material_instance = None;
        self._image_aspect = self._next_image_aspect;
        self._prev_opacity = self._opacity;
        self._prev_image_brightness = self._image_brightness;

        let window_aspect: f32 = self._window_size.x as f32 / self._window_size.y as f32;
        let image_widget = ptr_as_mut(self._image_layout.as_ref());
        let ui_component = image_widget.get_ui_component_mut();
        ui_component.set_material_instance(self._material_instance.clone());
        ui_component.set_visible(self._material_instance.is_some());
        if window_aspect < self._image_aspect {
            ui_component.set_size_hint_x(Some(self._image_size_hint));
            ui_component.set_size_hint_y(Some(window_aspect / self._image_aspect * self._image_size_hint));
        } else {
            ui_component.set_size_hint_x(Some(self._image_aspect / window_aspect * self._image_size_hint));
            ui_component.set_size_hint_y(Some(self._image_size_hint));
        }
    }

    pub fn is_done_manual_fade_out(&self) -> bool {
        self._done_manual_fade_out
    }

    pub fn is_done_game_image_progress(&self) -> bool {
        self.get_progress(self._fade_time) == 1.0
    }

    pub fn set_auto_fade_inout(&mut self, auto_fade_inout: bool) {
        self._auto_fade_inout = auto_fade_inout
    }

    pub fn get_progress(&self, fade_time: f32) -> f32 {
        if 0.0 != self._initial_fade_time {
            1.0_f32.min(fade_time / self._initial_fade_time)
        } else {
            1.0
        }
    }

    pub fn is_visible(&self) -> bool {
        self._background_layout.as_ref().get_ui_component()._visible
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        if window_size.x <= 0 || window_size.y <= 0 {
            return;
        }
        self._window_size = *window_size;
        let window_aspect: f32 = window_size.x as f32 / window_size.y as f32;
        let image_widget = ptr_as_mut(self._image_layout.as_ref());
        let ui_component = image_widget.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(self._image_aspect / window_aspect * self._image_size_hint));
        ui_component.set_size_hint_y(Some(self._image_size_hint));
    }

    pub fn update_game_image(&mut self, delta_time: f64, force: bool) {
        let is_scene_loading = is_application_valid() && get_game_scene_manager().is_scene_loading();
        let prev_progress = self.get_progress(self._fade_time);
        if prev_progress < 1.0 || force {
            // progress
            self._done_manual_fade_out = false;
            let mut next_fade_time = self._fade_time + self._fade_speed * delta_time as f32;
            let mut progress = self.get_progress(next_fade_time);
            if prev_progress <= HALF_FADE_PROGRESS && HALF_FADE_PROGRESS < progress || force {
                if self._auto_fade_inout && (!is_scene_loading || force) {
                    self.change_game_image();
                } else {
                    progress = HALF_FADE_PROGRESS;
                    next_fade_time = self._initial_fade_time * HALF_FADE_PROGRESS;
                    if !self._auto_fade_inout {
                        self._done_manual_fade_out = true;
                    }
                }
            }
            self._fade_time = next_fade_time;

            // calc opacity
            if progress == 1.0 {
                self._fade_speed = 1.0;
                self._opacity = self._fadein_opacity;
                self._image_brightness = self._fadein_image_brightness;
            } else if progress <= HALF_FADE_PROGRESS {
                let curved_progress = 0f32.max(1f32.min((progress * 2.0).powf(HALF_FADE_PROGRESS)));
                self._opacity = math::lerp(self._prev_opacity, self._fadeout_opacity, curved_progress);
                self._image_brightness = math::lerp(
                    self._prev_image_brightness,
                    self._fadeout_image_brightness,
                    progress * 2.0,
                );
            } else {
                let curved_progress = 0f32.max(1f32.min(((progress - HALF_FADE_PROGRESS) * 2.0).powf(2.0)));
                self._opacity = math::lerp(self._prev_opacity, self._fadein_opacity, curved_progress);
                self._image_brightness = math::lerp(
                    self._prev_image_brightness,
                    self._fadein_image_brightness,
                    curved_progress,
                );
            }

            // set opacity
            let effective_opacity = if is_scene_loading && !force {
                FULL_FADE_OPACITY
            } else {
                self._opacity
            };
            let background_layout = ptr_as_mut(self._background_layout.as_ref());
            let ui_component = background_layout.get_ui_component_mut();
            ui_component.set_opacity(effective_opacity);
            ui_component.set_visible(0.0 < effective_opacity);

            let image_widget = ptr_as_mut(self._image_layout.as_ref());
            let ui_component = image_widget.get_ui_component_mut();
            let r = (self._image_brightness * 255.0) as u32;
            ui_component.set_color(get_color32(r, r, r, 255));
        } else {
            let effective_opacity = if is_scene_loading {
                FULL_FADE_OPACITY
            } else {
                self._opacity
            };
            let background_layout = ptr_as_mut(self._background_layout.as_ref());
            let ui_component = background_layout.get_ui_component_mut();
            ui_component.set_opacity(effective_opacity);
            ui_component.set_visible(0.0 < effective_opacity);
        }

        // Dynamically update aspect ratio when async-loaded texture becomes available
        if let Some(ref mat_cell) = self._material_instance {
            let texture_name = mat_cell
                .borrow()
                ._material_parameters
                .get("texture_color")
                .and_then(|param| param.as_str())
                .map(|s| s.to_string());
            if let Some(ref name) = texture_name {
                let engine_resources = get_engine_resources();
                if let Some(texture_rc) = engine_resources.get_loaded_texture_data(name) {
                    let tex_borrow = texture_rc.borrow();
                    if tex_borrow._image_height > 0 {
                        let aspect = tex_borrow._image_width as f32 / tex_borrow._image_height as f32;
                        if (aspect - self._image_aspect).abs() > 0.001 && self._window_size.y > 0 {
                            self._image_aspect = aspect;
                            let window_aspect: f32 = self._window_size.x as f32 / self._window_size.y as f32;
                            let image_widget = ptr_as_mut(self._image_layout.as_ref());
                            let ui_component = image_widget.get_ui_component_mut();
                            if window_aspect < self._image_aspect {
                                ui_component.set_size_hint_x(Some(self._image_size_hint));
                                ui_component
                                    .set_size_hint_y(Some(window_aspect / self._image_aspect * self._image_size_hint));
                            } else {
                                ui_component
                                    .set_size_hint_x(Some(self._image_aspect / window_aspect * self._image_size_hint));
                                ui_component.set_size_hint_y(Some(self._image_size_hint));
                            }
                        }
                    }
                }
            }
        }
    }
}
