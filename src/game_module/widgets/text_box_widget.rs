use nalgebra::{Vector2};
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_controller::{GameController};

const ITEM_SIZE: f32 = 50.0;
const TEXT_WIDGET_WIDTH: f32 = 130.0;
const FONT_SIZE: f32 = 30.0;

pub struct TextBoxWidget<'a> {
    pub _text_box_items:  Vec<TextBoxItem<'a>>
}

pub struct TextBoxItem<'a> {
    pub _layout_widget: *const WidgetDefault<'a>,
    pub _binding_name_widget: *const WidgetDefault<'a>,
    pub _binding_icon_widget: *const WidgetDefault<'a>
}

impl<'a> TextBoxItem<'a> {
    pub fn create_text_box_item(parent_widget: &mut WidgetDefault<'a>) -> TextBoxItem<'a> {
        let layout_widget = UIManager::create_widget("text_box", UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_expandable_x(true);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        parent_widget.add_widget(&layout_widget);

        let binding_name_widget = UIManager::create_widget("name_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_x(TEXT_WIDGET_WIDTH);
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_margin_right(20.0);
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text("TextBox");
        layout_widget_mut.add_widget(&binding_name_widget);

        let binding_icon_widget = UIManager::create_widget("icon_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin_left(-14.0);
        ui_component.set_size_x(ITEM_SIZE);
        ui_component.set_size_y(ITEM_SIZE);
        layout_widget_mut.add_widget(&binding_icon_widget);

        TextBoxItem {
            _layout_widget: layout_widget.as_ref(),
            _binding_name_widget: binding_name_widget.as_ref(),
            _binding_icon_widget: binding_icon_widget.as_ref()
        }
    }
}

impl<'a> TextBoxWidget<'a> {
    pub fn create_text_box_widget(engine_resources: &EngineResources<'a>, root_widget: &mut WidgetDefault<'a>) -> TextBoxWidget<'a> {
        TextBoxWidget {
            _text_box_items: vec![
                TextBoxItem::create_text_box_item(root_widget)
            ],
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
    }

    pub fn update_text_box_widget(&mut self, game_scene_manager: &GameSceneManager, _game_controller: &GameController) {
        for text_box_item in self._text_box_items.iter_mut() {
            if game_scene_manager.get_character_manager().is_valid_player() {
                let player = game_scene_manager.get_character_manager().get_player().borrow();
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera();
                let screen_position = main_camera.convert_world_to_screen(player.get_position(), false);
                ptr_as_mut(text_box_item._layout_widget)._ui_component.set_pos(screen_position.x, screen_position.y);
            }
        }
    }
}
