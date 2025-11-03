use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::{EditorUIManager, GameUIManager};
use crate::render_pass;
use ash::vk;
use log::LevelFilter;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::constants;
use rust_engine_3d::constants::DEVELOPMENT;
use rust_engine_3d::core::engine_core::{self, ApplicationBase, EngineCore, WindowMode};
use rust_engine_3d::core::input::ButtonState;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::CallbackLoadRenderPassCreateInfo;
use rust_engine_3d::utilities::logger;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use winit::keyboard::KeyCode;

pub struct Application<'a> {
    pub _engine_core: *const EngineCore<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _effect_manager: *const EffectManager<'a>,
    pub _renderer_data: *const RendererData<'a>,
    pub _game_resources: Box<GameResources<'a>>,
    pub _game_scene_manager: Box<GameSceneManager<'a>>,
    pub _game_ui_manager: Box<GameUIManager<'a>>,
    pub _editor_ui_manager: Box<EditorUIManager<'a>>,
    pub _game_controller: Box<GameController<'a>>,
    pub _game_client: Box<GameClient<'a>>,
    pub _is_game_mode: bool,
}

impl<'a> ApplicationBase<'a> for Application<'a> {
    fn initialize_application(
        &'a mut self,
        engine_core: &EngineCore<'a>,
        window_size: &Vector2<i32>,
    ) {
        // engine managers
        self._engine_core = engine_core;
        self._audio_manager = engine_core.get_audio_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._renderer_data = engine_core.get_renderer_context().get_renderer_data();

        // initialize project managers
        let application = ptr_as_ref(self);
        self.get_game_resources_mut()
            .initialize_game_resources(engine_core.get_engine_resources());
        self.get_game_resources_mut().load_game_resources();
        self.get_game_client_mut()
            .initialize_game_client(engine_core, application);
        self.get_game_controller_mut()
            .initialize_game_controller(application);
        self.get_game_ui_manager_mut()
            .initialize_game_ui_manager(engine_core, application);
        self.get_game_scene_manager_mut()
            .initialize_game_scene_manager(application, engine_core, window_size);
        self.get_editor_ui_manager_mut()
            .initialize_editor_ui_manager(engine_core, application);

        // start game
        self.get_game_ui_manager_mut().build_game_ui(window_size);
        self.get_editor_ui_manager_mut()
            .build_editor_ui(window_size);
        self.set_game_mode(true);
    }

    fn terminate_application(&mut self) {
        self._game_scene_manager.close_game_scene_data();
        self._game_client.destroy_game_client();
        self._game_scene_manager.destroy_game_scene_manager();
        self._game_resources.destroy_game_resources();
    }

    fn get_render_pass_create_info_callback(&self) -> *const CallbackLoadRenderPassCreateInfo {
        static CALLBACK: CallbackLoadRenderPassCreateInfo =
            render_pass::render_pass::get_render_pass_data_create_infos;
        &CALLBACK
    }

    fn focused(&mut self, focused: bool) {
        self.set_game_mode(focused);
    }

    fn update_event(&mut self) {
        let engine_core = ptr_as_ref(self._engine_core);
        let time_data = &engine_core._time_data;
        let mouse_move_data = &engine_core._mouse_move_data;
        let mouse_input_data = &engine_core._mouse_input_data;
        let keyboard_input_data = &engine_core._keyboard_input_data;
        let joystick_input_data = &engine_core._joystick_input_data;

        if unsafe { DEVELOPMENT } {
            let is_toggle_game_mode_by_joystick = joystick_input_data._btn_left_trigger
                == ButtonState::Hold
                && joystick_input_data._btn_right_trigger == ButtonState::Hold
                && joystick_input_data._btn_left_shoulder == ButtonState::Hold
                && joystick_input_data._btn_right_shoulder == ButtonState::Hold;

            if engine_core
                ._keyboard_input_data
                .get_key_pressed(KeyCode::Tab)
                || is_toggle_game_mode_by_joystick
            {
                self.toggle_game_mode();
            }
        }

        if false == self._is_game_mode {
            const MOUSE_DELTA_RATIO: f32 = 500.0;
            let delta_time = time_data._delta_time;
            let _mouse_pos = &mouse_move_data._mouse_pos;
            let mouse_delta_x = mouse_move_data._mouse_pos_delta.x as f32
                / engine_core._window_size.x as f32
                * MOUSE_DELTA_RATIO;
            let mouse_delta_y = mouse_move_data._mouse_pos_delta.y as f32
                / engine_core._window_size.y as f32
                * MOUSE_DELTA_RATIO;
            let btn_left: bool = mouse_input_data._btn_l_hold;
            let btn_right: bool = mouse_input_data._btn_r_hold;
            let btn_r_pressed: bool = mouse_input_data._btn_r_pressed;
            let btn_r_released: bool = mouse_input_data._btn_r_released;
            let _btn_middle: bool = mouse_input_data._btn_m_hold;

            if btn_r_pressed {
                self.get_engine_core_mut().set_grab_mode(true);
            } else if btn_r_released {
                self.get_engine_core_mut().set_grab_mode(false);
            }

            let pressed_key_a = keyboard_input_data.get_key_hold(KeyCode::KeyA);
            let pressed_key_d = keyboard_input_data.get_key_hold(KeyCode::KeyD);
            let pressed_key_w = keyboard_input_data.get_key_hold(KeyCode::KeyW);
            let pressed_key_s = keyboard_input_data.get_key_hold(KeyCode::KeyS);
            let pressed_key_q = keyboard_input_data.get_key_hold(KeyCode::KeyQ);
            let pressed_key_e = keyboard_input_data.get_key_hold(KeyCode::KeyE);
            let pressed_key_z = keyboard_input_data.get_key_hold(KeyCode::KeyZ);
            let pressed_key_c = keyboard_input_data.get_key_hold(KeyCode::KeyC);
            let pressed_key_comma = keyboard_input_data.get_key_hold(KeyCode::Comma);
            let pressed_key_period = keyboard_input_data.get_key_hold(KeyCode::Period);
            let released_key_left_bracket =
                keyboard_input_data.get_key_released(KeyCode::BracketLeft);
            let released_key_right_bracket =
                keyboard_input_data.get_key_released(KeyCode::BracketRight);
            let released_key_subtract = keyboard_input_data.get_key_released(KeyCode::Minus);
            let released_key_equals = keyboard_input_data.get_key_released(KeyCode::Equal);
            let modifier_keys_shift = keyboard_input_data.get_key_hold(KeyCode::ShiftLeft);
            let scene_manager = self.get_game_scene_manager().get_scene_manager();
            let main_camera = scene_manager.get_main_camera_mut();
            let main_light = ptr_as_mut(scene_manager.get_main_light().as_ptr());
            let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
            let move_speed: f32 = game_constants::EDITOR_CAMERA_MOVE_SPEED
                * camera_move_speed_multiplier
                * delta_time as f32;
            let pan_speed = game_constants::EDITOR_CAMERA_PAN_SPEED * camera_move_speed_multiplier;
            let rotation_speed = game_constants::EDITOR_CAMERA_ROTATION_SPEED;

            if released_key_left_bracket {
                self.get_renderer_data_mut().prev_debug_render_target();
            } else if released_key_right_bracket {
                self.get_renderer_data_mut().next_debug_render_target();
            }

            if released_key_subtract {
                self.get_renderer_data_mut()
                    .prev_debug_render_target_miplevel();
            } else if released_key_equals {
                self.get_renderer_data_mut()
                    .next_debug_render_target_miplevel();
            }

            if pressed_key_comma {
                main_light._transform_object.rotation_pitch(rotation_speed);
            } else if pressed_key_period {
                main_light._transform_object.rotation_pitch(-rotation_speed);
            }

            if btn_left && btn_right {
                main_camera
                    ._transform_object
                    .move_right(pan_speed * mouse_delta_x);
                main_camera
                    ._transform_object
                    .move_up(-pan_speed * mouse_delta_y);
            } else if btn_right {
                main_camera
                    ._transform_object
                    .rotation_pitch(rotation_speed * mouse_delta_y);
                main_camera
                    ._transform_object
                    .rotation_yaw(rotation_speed * mouse_delta_x);
            }

            if pressed_key_z {
                main_camera
                    ._transform_object
                    .rotation_roll(-rotation_speed * delta_time as f32 * 100.0);
            } else if pressed_key_c {
                main_camera
                    ._transform_object
                    .rotation_roll(rotation_speed * delta_time as f32 * 100.0);
            }

            if pressed_key_w {
                main_camera._transform_object.move_front(move_speed);
            } else if pressed_key_s {
                main_camera._transform_object.move_front(-move_speed);
            }

            if pressed_key_a {
                main_camera._transform_object.move_right(-move_speed);
            } else if pressed_key_d {
                main_camera._transform_object.move_right(move_speed);
            }

            if pressed_key_q {
                main_camera._transform_object.move_up(-move_speed);
            } else if pressed_key_e {
                main_camera._transform_object.move_up(move_speed);
            }
        }
    }

    fn update_application(&mut self, delta_time: f64) {
        let engine_core = ptr_as_ref(self._engine_core.clone());
        let font_manager = engine_core.get_font_manager_mut();
        font_manager.clear_logs();

        // update managers
        if self._is_game_mode {
            // delta time threshold: 0.1 sec
            let game_delta_time = 0.1_f64.min(delta_time);
            self._game_client.update_game_mode(game_delta_time);
            self.get_game_ui_manager_mut().update_game_ui(delta_time);
        } else {
            self.get_editor_ui_manager_mut()
                .update_editor_ui(delta_time);
        }
    }
}

impl<'a> Application<'a> {
    pub fn get_engine_core(&self) -> &EngineCore<'a> {
        ptr_as_ref(self._engine_core)
    }
    pub fn get_engine_core_mut(&self) -> &mut EngineCore<'a> {
        ptr_as_mut(self._engine_core)
    }
    pub fn get_effect_manager(&self) -> &EffectManager<'a> {
        ptr_as_ref(self._effect_manager)
    }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager<'a> {
        ptr_as_mut(self._effect_manager)
    }
    pub fn get_game_resources(&self) -> &GameResources<'a> {
        ptr_as_ref(self._game_resources.as_ref())
    }
    pub fn get_game_resources_mut(&self) -> &mut GameResources<'a> {
        ptr_as_mut(self._game_resources.as_ref())
    }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager<'a> {
        self._game_scene_manager.as_ref()
    }
    pub fn get_game_scene_manager_mut(&mut self) -> &mut GameSceneManager<'a> {
        self._game_scene_manager.as_mut()
    }
    pub fn get_renderer_data(&self) -> &RendererData<'a> {
        ptr_as_ref(self._renderer_data)
    }
    pub fn get_renderer_data_mut(&self) -> &mut RendererData<'a> {
        ptr_as_mut(self._renderer_data)
    }
    pub fn get_editor_ui_manager(&self) -> &EditorUIManager<'a> {
        ptr_as_ref(self._editor_ui_manager.as_ref())
    }
    pub fn get_editor_ui_manager_mut(&self) -> &mut EditorUIManager<'a> {
        ptr_as_mut(self._editor_ui_manager.as_ref())
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager<'a> {
        ptr_as_ref(self._game_ui_manager.as_ref())
    }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager<'a> {
        ptr_as_mut(self._game_ui_manager.as_ref())
    }
    pub fn get_audio_manager(&self) -> &AudioManager<'a> {
        ptr_as_ref(self._audio_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_game_controller(&self) -> &GameController<'a> {
        self._game_controller.as_ref()
    }
    pub fn get_game_controller_mut(&self) -> &mut GameController<'a> {
        ptr_as_mut(self._game_controller.as_ref())
    }
    pub fn get_game_client(&self) -> &GameClient<'a> {
        self._game_client.as_ref()
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient<'a> {
        ptr_as_mut(self._game_client.as_ref())
    }
    pub fn toggle_game_mode(&mut self) {
        self.set_game_mode(!self._is_game_mode);
    }
    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self._is_game_mode = is_game_mode;
        self.get_game_client_mut().set_game_mode(is_game_mode);
        self.get_engine_core_mut().set_grab_mode(is_game_mode);
        self.get_engine_core_mut()
            .get_ui_manager_mut()
            .set_visible_world_axis(!is_game_mode);
    }
}

pub fn run_application() {
    let app_name: String = "Stone Age".to_string();
    let app_version: u32 = 1;
    let initial_window_size: Option<Vector2<u32>> = None; // Some(Vector2::new(1024, 768));
    let window_mode = WindowMode::FullScreenBorderlessMode;

    // Graphics Settings
    unsafe {
        #[cfg(target_os = "android")]
        {
            constants::VULKAN_API_VERSION = vk::make_api_version(0, 1, 2, 0);
            constants::ENABLE_IMMEDIATE_MODE = false;
            constants::IS_CONCURRENT_MODE = false;
        }
        #[cfg(not(target_os = "android"))]
        {
            constants::VULKAN_API_VERSION = vk::make_api_version(0, 1, 3, 0);
            constants::ENABLE_IMMEDIATE_MODE = true;
            constants::IS_CONCURRENT_MODE = true;
        }

        if DEVELOPMENT {
            constants::DEBUG_MESSAGE_LEVEL = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
            constants::REQUIRED_INSTANCE_LAYERS = vec![
                "VK_LAYER_KHRONOS_validation".to_string(),
                "VK_LAYER_LUNARG_standard_validation".to_string(),
            ];
        } else {
            constants::DEBUG_MESSAGE_LEVEL = vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;
        }

        constants::REQUIRED_DEVICE_EXTENSIONS = vec![
            "VK_KHR_swapchain".to_string(),
            "VK_KHR_buffer_device_address".to_string(),
            "VK_KHR_deferred_host_operations".to_string(),
        ];

        // ray tracing
        constants::USE_RAY_TRACING = false;
        constants::REQUIRED_RAY_TRACING_EXTENSIONS = vec![
            "VK_NV_ray_tracing".to_string(),
            "VK_KHR_ray_query".to_string(),
            "VK_KHR_ray_tracing_pipeline".to_string(),
            "VK_KHR_acceleration_structure".to_string(),
        ];

        constants::ENABLE_UPSCALE = false;
    }

    // logger
    logger::initialize_logger(if unsafe { DEVELOPMENT } {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    });

    // create project application & managers
    let game_resources = GameResources::create_game_resources();
    let game_scene_manager = GameSceneManager::create_game_scene_manager();
    let game_ui_manager = GameUIManager::create_game_ui_manager();
    let editor_ui_manager = EditorUIManager::create_editor_ui_manager();
    let game_controller = GameController::create_game_controller();
    let game_client = GameClient::create_game_client();
    let application = Application {
        _engine_core: std::ptr::null(),
        _renderer_data: std::ptr::null(),
        _effect_manager: std::ptr::null(),
        _audio_manager: std::ptr::null(),
        _game_resources: game_resources,
        _game_scene_manager: game_scene_manager,
        _game_ui_manager: game_ui_manager,
        _editor_ui_manager: editor_ui_manager,
        _game_controller: game_controller,
        _game_client: game_client,
        _is_game_mode: false,
    };

    // run
    engine_core::run_application(
        app_name,
        app_version,
        initial_window_size,
        window_mode,
        &application, // TODO: Remove
    );
}
