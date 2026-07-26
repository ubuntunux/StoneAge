use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::widgets::world_map::api::{WorldMapBridge, WorldMapDirection, WorldMapStage, WorldMapWidget};
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;

impl<'a> WorldMapStage<'a> {
    pub fn callback_touch_down(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        if !ui_component.get_user_data().is_null() {
            let world_map_stage = ptr_as_ref(ui_component.get_user_data() as *const WorldMapStage<'a>);
            world_map_stage
                .get_world_map_widget_mut()
                .set_selected_world_map_stage(world_map_stage.get_stage_data_name());
        }
        true
    }

    pub fn create_world_map_stage(
        world_map_stages: &mut HashMap<String, Rc<WorldMapStage<'a>>>,
        world_map_widget: &WorldMapWidget<'a>,
        _game_resources: &GameResources<'a>,
        root_layout: &mut WidgetDefault<'a>,
        stage: Stages,
    ) -> Rc<WorldMapStage<'a>> {
        let world_map_stage = UIManager::create_widget("world_map_stage", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_border(4.0);
        ui_component.set_border_color(get_color32(128, 128, 128, 255));
        ui_component.set_round(10.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text(stage.get_stage_display_name());
        ui_component.set_font_size(32.0);
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_color(get_color32(255, 255, 255, 255));

        const STAGE_SIZE: f32 = 100.0;
        ui_component.set_size(STAGE_SIZE, STAGE_SIZE);
        root_layout.add_widget(&world_map_stage);

        let world_map_stage = Rc::new(WorldMapStage {
            _world_map_widget: world_map_widget,
            _stage_data_name: String::from(stage.get_stage_data_name()),
            _selected: false,
            _world_map_stage: world_map_stage,
            _linked_stages: [None, None, None, None, None],
            _linked_bridges: [None, None, None, None, None],
        });

        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(WorldMapStage::callback_touch_down)));
        ui_component.set_user_data(world_map_stage.as_ref() as *const WorldMapStage<'a> as *const c_void);

        world_map_stages.insert(String::from(stage.get_stage_data_name()), world_map_stage.clone());

        world_map_stage
    }

    pub fn get_world_map_widget(&self) -> &WorldMapWidget<'a> {
        ptr_as_ref(self._world_map_widget)
    }

    pub fn get_world_map_widget_mut(&self) -> &mut WorldMapWidget<'a> {
        ptr_as_mut(self._world_map_widget)
    }

    pub fn get_selected(&self) -> bool {
        self._selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_color(if selected {
            get_color32(255, 255, 0, 255)
        } else {
            get_color32(255, 255, 255, 255)
        });
        self._selected = selected;
    }

    pub fn get_center_pos(&self) -> Vector2<f32> {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.get_center()
    }

    pub fn set_center_pos(&mut self, center_x: f32, center_y: f32) {
        let ui_component = ptr_as_mut(self._world_map_stage.as_ref()).get_ui_component_mut();
        ui_component.set_center(center_x, center_y);
    }

    pub fn get_stage_data_name(&self) -> &String {
        &self._stage_data_name
    }

    pub fn get_linked_stage(&self, direction: WorldMapDirection) -> &Option<Rc<WorldMapStage<'a>>> {
        &self._linked_stages[direction as usize]
    }

    pub fn set_linked_stage(
        &mut self,
        game_resources: &GameResources<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        direction: WorldMapDirection,
        linked_stage: &Rc<WorldMapStage<'a>>,
        map_size: &Vector2<f32>,
        make_bridge: bool,
    ) {
        let linked_stage_mut = ptr_as_mut(linked_stage.as_ref());
        let pos_step_x = map_size.x * 0.15;
        let pos_step_y = map_size.y * 0.15;
        let pos = self.get_center_pos();
        if direction == WorldMapDirection::LEFT {
            linked_stage_mut.set_center_pos(pos.x - pos_step_x, pos.y);
        } else if direction == WorldMapDirection::RIGHT {
            linked_stage_mut.set_center_pos(pos.x + pos_step_x, pos.y);
        } else if direction == WorldMapDirection::UP {
            linked_stage_mut.set_center_pos(pos.x, pos.y - pos_step_y);
        } else if direction == WorldMapDirection::DOWN {
            linked_stage_mut.set_center_pos(pos.x, pos.y + pos_step_y);
        }
        self._linked_stages[direction as usize] = Some(linked_stage.clone());

        if make_bridge {
            let bridge_widget = WorldMapBridge::create_world_map_bridge(
                game_resources,
                bridge_layer,
                &pos,
                &linked_stage_mut.get_center_pos(),
            );
            self._linked_bridges[direction as usize] = Some(bridge_widget);
        }
    }
}
