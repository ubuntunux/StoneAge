use std::os::raw::c_void;

use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, ApplicationUIManagerBase, UIComponentInstance, UIManager,
    UIWidgetTypes, VerticalAlign, Widget, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::application::application::Application;
use crate::game_module::game_client::GameClient;

use crate::game_module::game_ui_manager::{GameUIManager, UISwitch, UIWorldAxis};
use crate::game_module::widgets::hud::{CrossHair, PlayerHud, SelectionArea, TargetHud};

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _ui_manager: std::ptr::null(),
            _game_client: std::ptr::null(),
            _root_widget: std::ptr::null() as *const WidgetDefault,
            _game_ui_layout: std::ptr::null() as *const WidgetDefault,
            _ui_switch: None,
            _ui_world_axis: None,
            _cross_hair: None,
            _target_hud: None,
            _player_hud: None,
            _selection_area: None,
        })
    }

    pub fn game_ui_layout(&self) -> *const dyn Widget {
        self._game_ui_layout
    }
}

impl ApplicationUIManagerBase for GameUIManager {
    fn get_ui_manager(&self) -> &UIManager {
        ptr_as_ref(self._ui_manager)
    }
    fn get_ui_manager_mut(&self) -> &mut UIManager {
        ptr_as_mut(self._ui_manager)
    }
    fn get_root_widget(&self) -> &dyn Widget {
        ptr_as_ref(self._root_widget)
    }
    fn get_root_widget_mut(&self) -> &mut dyn Widget {
        ptr_as_mut(self._root_widget as *mut dyn Widget)
    }
    fn initialize_application_ui_manager(&mut self, ui_manager: &UIManager) {
        self._ui_manager = ui_manager;
        self._root_widget = self.get_ui_manager().get_root_ptr();
    }
    fn build_ui(&mut self, _renderer_context: &RendererContext, engine_resources: &EngineResources) {
        let game_ui_layout = UIManager::create_widget("game ui layout", UIWidgetTypes::Default);
        let game_ui_layout_mut = ptr_as_mut(game_ui_layout.as_ref());
        let ui_component = game_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);

        self._game_ui_layout = game_ui_layout.as_ref();

        let root_widget_mut = ptr_as_mut(self._root_widget);
        root_widget_mut.add_widget(&game_ui_layout);

        self._ui_switch = Some(UISwitch::create_ui_switch(
            engine_resources,
            root_widget_mut,
            game_ui_layout_mut,
        ));

        self._ui_world_axis = Some(UIWorldAxis::create_ui_world_axis(
            engine_resources,
            root_widget_mut,
        ));
    }

    fn update_ui_manager(&mut self, engine_core: &EngineCore, _delta_time: f64) {
        let main_camera = engine_core.get_scene_manager().get_main_camera();
        let window_height: f32 = main_camera._window_size.y as f32;
        let size: f32 = window_height * 0.05;
        let border: f32 = 20.0;
        let start_pos_x: f32 = size * 2.0 + border;
        let start_pos_y: f32 = window_height - (size * 2.0 + border);
        let camera_up = main_camera._transform_object.get_up();
        let camera_right = main_camera._transform_object.get_right();
        let axis_x: Vector2<f32> = Vector2::new(camera_right.x, -camera_up.x) * size;
        let axis_y: Vector2<f32> = Vector2::new(camera_right.y, -camera_up.y) * size;
        let axis_z: Vector2<f32> = Vector2::new(camera_right.z, -camera_up.z) * size;

        let ui_world_axis = self._ui_world_axis.as_mut().unwrap();
        let ui_component_x =
            ptr_as_mut(ui_world_axis._widget_axis_x.as_ref()).get_ui_component_mut();
        ui_component_x.set_pos_x(start_pos_x + axis_x.x);
        ui_component_x.set_pos_y(start_pos_y + axis_x.y);

        let ui_component_y =
            ptr_as_mut(ui_world_axis._widget_axis_y.as_ref()).get_ui_component_mut();
        ui_component_y.set_pos_x(start_pos_x + axis_y.x);
        ui_component_y.set_pos_y(start_pos_y + axis_y.y);

        let ui_component_z =
            ptr_as_mut(ui_world_axis._widget_axis_z.as_ref()).get_ui_component_mut();
        ui_component_z.set_pos_x(start_pos_x + axis_z.x);
        ui_component_z.set_pos_y(start_pos_y + axis_z.y);

        let debug_line_manager = engine_core.get_debug_line_manager_mut();
        debug_line_manager.add_debug_line_2d(
            &Vector2::new(start_pos_x, start_pos_y),
            &Vector2::new(start_pos_x + axis_x.x, start_pos_y + axis_x.y),
            get_color32(255, 0, 0, 255),
        );

        debug_line_manager.add_debug_line_2d(
            &Vector2::new(start_pos_x, start_pos_y),
            &Vector2::new(start_pos_x + axis_y.x, start_pos_y + axis_y.y),
            get_color32(0, 255, 0, 255),
        );

        debug_line_manager.add_debug_line_2d(
            &Vector2::new(start_pos_x, start_pos_y),
            &Vector2::new(start_pos_x + axis_z.x, start_pos_y + axis_z.y),
            get_color32(0, 0, 255, 255),
        );
    }
}

impl UISwitch {
    pub fn create_ui_switch(
        _engine_resources: &EngineResources,
        root_widget: &mut dyn Widget,
        game_ui_widget: &dyn Widget,
    ) -> UISwitch {
        let ui_switch_widget = UIManager::create_widget("ui_switch", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(ui_switch_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text("UI On/Off");
        ui_component.set_pos_hint_x(Some(0.5));
        ui_component.set_pos_hint_y(Some(0.0));
        ui_component.set_size(150.0, 50.0);
        ui_component.set_font_size(20.0);
        ui_component.set_color(get_color32(128, 128, 255, 128));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(2.0);
        ui_component.set_touchable(true);
        //ui_component.set_material_instance(&engine_resources.get_material_instance_data("ui/render_ui_test"));

        static TOUCH_DOWN: CallbackTouchEvent = UISwitch::touch_down;
        ui_component.set_callback_touch_down(&TOUCH_DOWN);
        ui_component.set_user_data(
            game_ui_widget.get_ui_component() as *const UIComponentInstance as *const c_void,
        );
        root_widget.add_widget(&ui_switch_widget);

        let ui_switch = UISwitch {
            _ui_switch_widget: ui_switch_widget,
        };

        ui_switch
    }

    pub fn touch_down(
        ui_component: &mut UIComponentInstance,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_ui_component =
            ptr_as_mut(ui_component.get_user_data() as *const UIComponentInstance);
        game_ui_component.set_visible(!game_ui_component.get_visible());
        true
    }
}

impl UIWorldAxis {
    pub fn create_ui_world_axis(
        _engine_resources: &EngineResources,
        root_widget: &mut dyn Widget,
    ) -> UIWorldAxis {
        let widget_axis_x = UIManager::create_widget("ui_axis_x", UIWidgetTypes::Default);
        let ui_component_axis_x = ptr_as_mut(widget_axis_x.as_ref()).get_ui_component_mut();
        ui_component_axis_x.set_text("X");
        ui_component_axis_x.set_size(10.0, 10.0);
        ui_component_axis_x.set_font_size(20.0);
        ui_component_axis_x.set_color(get_color32(255, 255, 255, 0));
        ui_component_axis_x.set_font_color(get_color32(255, 0, 0, 255));
        ui_component_axis_x.set_halign(HorizontalAlign::CENTER);
        ui_component_axis_x.set_valign(VerticalAlign::CENTER);
        root_widget.add_widget(&widget_axis_x);

        let widget_axis_y = UIManager::create_widget("ui_axis_y", UIWidgetTypes::Default);
        let ui_component_axis_y = ptr_as_mut(widget_axis_y.as_ref()).get_ui_component_mut();
        ui_component_axis_y.set_text("Y");
        ui_component_axis_y.set_size(10.0, 10.0);
        ui_component_axis_y.set_font_size(20.0);
        ui_component_axis_y.set_color(get_color32(255, 255, 255, 0));
        ui_component_axis_y.set_font_color(get_color32(0, 255, 0, 255));
        ui_component_axis_y.set_halign(HorizontalAlign::CENTER);
        ui_component_axis_y.set_valign(VerticalAlign::CENTER);
        root_widget.add_widget(&widget_axis_y);

        let widget_axis_z = UIManager::create_widget("ui_axis_z", UIWidgetTypes::Default);
        let ui_component_axis_z = ptr_as_mut(widget_axis_z.as_ref()).get_ui_component_mut();
        ui_component_axis_z.set_text("Z");
        ui_component_axis_z.set_size(10.0, 10.0);
        ui_component_axis_z.set_font_size(20.0);
        ui_component_axis_z.set_color(get_color32(255, 255, 255, 0));
        ui_component_axis_z.set_font_color(get_color32(0, 0, 255, 255));
        ui_component_axis_z.set_halign(HorizontalAlign::CENTER);
        ui_component_axis_z.set_valign(VerticalAlign::CENTER);
        root_widget.add_widget(&widget_axis_z);

        UIWorldAxis {
            _widget_axis_x: widget_axis_x,
            _widget_axis_y: widget_axis_y,
            _widget_axis_z: widget_axis_z,
        }
    }
}

impl GameUIManager {
    pub fn get_game_client(&self) -> &GameClient {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient {
        ptr_as_mut(self._game_client)
    }
    pub fn initialize_game_ui_manager(&mut self, application: &Application) {
        log::info!("initialize_game_ui_manager");
        self._game_client = application.get_game_client()
    }

    pub fn destroy_game_ui_manager(&mut self) {

    }

    pub fn build_game_ui(&mut self) {
        log::info!("build_game_ui");
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        let game_ui_layout_mut = ptr_as_mut(game_client.get_game_ui_manager().game_ui_layout());
        let window_size = &game_client
            .get_application()
            .get_engine_core()
            ._window_size;
        let window_center =
            Vector2::<f32>::new(window_size.x as f32 * 0.5, window_size.y as f32 * 0.5);

        self._cross_hair = Some(CrossHair::create_cross_hair(
            game_resources,
            game_ui_layout_mut,
            &window_center,
        ));
        self._target_hud = Some(TargetHud::create_target_hud(
            game_ui_layout_mut,
            &window_center,
        ));
        self._player_hud = Some(PlayerHud::create_player_hud(
            game_ui_layout_mut,
            &Vector2::new(window_size.x as f32 - 200.0, window_center.y),
        ));
        self._selection_area = Some(SelectionArea::create_selection_area(
            game_ui_layout_mut,
            window_size,
        ));
    }

    pub fn get_cross_hair_widget_mut(&mut self) -> &mut WidgetDefault {
        ptr_as_mut(self._cross_hair.as_ref().unwrap()._widget)
    }

    pub fn show_selection_area(&mut self, show: bool) {
        let selection_area_widget = self
            ._selection_area
            .as_ref()
            .unwrap()
            ._selection_area_layout
            .as_ref();
        let ui_component = ptr_as_mut(selection_area_widget.get_ui_component());
        ui_component.set_visible(show);
    }

    pub fn show_cross_hair(&mut self, show: bool) {
        let ui_component = self.get_cross_hair_widget_mut().get_ui_component_mut();
        ui_component.set_visible(show);
    }

    pub fn set_cross_hair_tracking_mouse(&mut self, tracking: bool) {
        self._cross_hair.as_mut().unwrap()._tracking_mouse = tracking;
    }

    pub fn set_cross_hair_pos(&mut self, pos: &Vector2<i32>) {
        self._cross_hair.as_mut().unwrap()._pos.clone_from(pos);
    }

    pub fn update_game_ui(&mut self, _delta_time: f32) {
        let game_client = ptr_as_ref(self._game_client);
        let window_size = &game_client
            .get_application()
            .get_engine_core()
            ._window_size;

        // Cross Hair
        let crosshair = self._cross_hair.as_ref().unwrap();
        let crosshair_widget = ptr_as_mut(crosshair._widget);
        if crosshair_widget.get_ui_component().get_visible() {
            let crosshair_pos_x: i32;
            let crosshair_pos_y: i32;

            if crosshair._tracking_mouse {
                crosshair_pos_x = crosshair._pos.x;
                crosshair_pos_y = crosshair._pos.y;
            } else {
                crosshair_pos_x = window_size.x / 2;
                crosshair_pos_y = window_size.y / 2;
            }
            let ui_component = crosshair_widget.get_ui_component_mut();
            ui_component.set_center(crosshair_pos_x as f32, crosshair_pos_y as f32);
        }
    }
}
