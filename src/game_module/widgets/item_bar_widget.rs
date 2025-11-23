use crate::game_module::actors::items::ItemDataType;
use nalgebra::Vector2;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

const ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM: f32 = 50.0;
const MAX_ITEM_TYPE_COUNT: usize = 10;
pub const ITEM_UI_SIZE: f32 = 64.0;
const WIDGET_UI_MARGIN: f32 = 5.0;
const INVALID_ITEM_INDEX: usize = usize::MAX;

pub struct ItemWidget<'a> {
    pub _item_type: ItemDataType,
    pub _item_index: usize,
    pub _item_count: usize,
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemSelectionWidget<'a> {
    pub _item_index: usize,
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemBarWidget<'a> {
    pub _engine_resources: *const EngineResources<'a>,
    pub _layer: *const WidgetDefault<'a>,
    pub _select_previous_item_widget: *const WidgetDefault<'a>,
    pub _select_next_item_widget: *const WidgetDefault<'a>,
    pub _item_widgets: Vec<ItemWidget<'a>>,
    pub _selected_item_widget: ItemSelectionWidget<'a>,
    pub _item_type_count: usize,
    pub _max_item_type_count: usize,
}

impl<'a> ItemWidget<'a> {
    pub fn create_item_widget(
        parent_widget: &mut WidgetDefault<'a>,
        item_index: usize,
    ) -> ItemWidget<'a> {
        let item_widget = UIManager::create_widget("item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_pos_x(ITEM_UI_SIZE * item_index as f32);
        ui_component.set_round(2.0);
        ui_component.set_margin(WIDGET_UI_MARGIN);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(30.0);
        ui_component.set_visible(false);
        parent_widget.add_widget(&item_widget);

        ItemWidget {
            _item_type: ItemDataType::None,
            _item_index: item_index,
            _item_count: 0,
            _widget: item_widget.as_ref(),
        }
    }

    pub fn set_item_data(
        &mut self,
        item_data_type: &ItemDataType,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        item_count: usize,
    ) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_material_instance(material_instance);
        self._item_type = *item_data_type;
        self.set_item_count(item_count);
    }

    pub fn get_item_count(&self) -> usize {
        self._item_count
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        self._item_count = item_count;
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_text(format!("{}", self._item_count).as_str());
        if 0 < self._item_count {
            ui_component.set_visible(true);
        } else {
            self._item_type = ItemDataType::None;
            ui_component.set_visible(false);
        }
    }

    pub fn add_item_count(&mut self, item_count: usize) -> usize {
        self.set_item_count(self._item_count + item_count);
        self._item_count
    }

    pub fn remove_item_count(&mut self, item_count: usize) -> usize {
        if item_count <= self._item_count {
            self.set_item_count(self._item_count - item_count);
        }
        self._item_count
    }
}

impl<'a> ItemSelectionWidget<'a> {
    pub fn get_item_index(&self) -> usize {
        self._item_index
    }

    pub fn update_selected_item_widget(&mut self, item_index: usize) {
        self._item_index = item_index;
        let widget = ptr_as_mut(self._widget);
        let ui_component = widget.get_ui_component_mut();
        if self._item_index != INVALID_ITEM_INDEX {
            ui_component.set_pos_x(ITEM_UI_SIZE * item_index as f32);
            ui_component.set_visible(true);
        } else {
            ui_component.set_visible(false);
        }
    }
}

impl<'a> ItemBarWidget<'a> {
    pub fn create_item_bar_widget(
        engine_resources: &EngineResources<'a>,
        parent_widget: &mut WidgetDefault<'a>
    ) -> ItemBarWidget<'a> {
        let layer = UIManager::create_widget("item_bar_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(MAX_ITEM_TYPE_COUNT as f32 * ITEM_UI_SIZE);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        parent_widget.add_widget(&layer);

        let select_previous_item_widget = UIManager::create_widget("select_previous_item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(select_previous_item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(ITEM_UI_SIZE * 3.0, ITEM_UI_SIZE);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        parent_widget.add_widget(&select_previous_item_widget);

        let select_next_item_widget = UIManager::create_widget("select_next_item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(select_next_item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(ITEM_UI_SIZE * 3.0, ITEM_UI_SIZE);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        parent_widget.add_widget(&select_next_item_widget);

        let selected_item_widget = UIManager::create_widget("selected_item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(selected_item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_margin(WIDGET_UI_MARGIN);
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_visible(false);
        ptr_as_mut(layer.as_ref()).add_widget(&selected_item_widget);

        let mut item_bar_widget = ItemBarWidget {
            _engine_resources: engine_resources,
            _layer: layer.as_ref(),
            _select_previous_item_widget: select_previous_item_widget.as_ref(),
            _select_next_item_widget: select_next_item_widget.as_ref(),
            _item_widgets: Vec::new(),
            _selected_item_widget: ItemSelectionWidget {
                _item_index: INVALID_ITEM_INDEX,
                _widget: selected_item_widget.as_ref(),
            },
            _item_type_count: 0,
            _max_item_type_count: MAX_ITEM_TYPE_COUNT,
        };

        for item_index in 0..MAX_ITEM_TYPE_COUNT {
            let item_widget = ItemWidget::create_item_widget(ptr_as_mut(layer.as_ref()), item_index);
            item_bar_widget._item_widgets.push(item_widget);
        }

        item_bar_widget.update_item_bar_widget();
        item_bar_widget
    }

    pub fn get_select_previous_item_widget(&self) -> *const WidgetDefault<'a> {
        self._select_previous_item_widget
    }

    pub fn get_select_next_item_widget(&self) -> *const WidgetDefault<'a> {
        self._select_next_item_widget
    }

    pub fn get_selected_item_widget(&self) -> &ItemSelectionWidget<'a> {
        &self._selected_item_widget
    }

    pub fn update_item_bar_widget(&mut self) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_size_y(ITEM_UI_SIZE);
    }

    pub fn get_item_widget(&self, index: usize) -> &ItemWidget<'a> {
        &self._item_widgets[index]
    }

    pub fn get_item_widget_mut(&mut self, index: usize) -> &mut ItemWidget<'a> {
        &mut self._item_widgets[index]
    }

    pub fn find_item_widget(&self, item_data_type: &ItemDataType) -> Option<&ItemWidget<'a>> {
        self._item_widgets
            .iter()
            .find(|item_widget| item_widget._item_type == *item_data_type)
    }

    pub fn find_item_widget_mut(
        &mut self,
        item_data_type: &ItemDataType,
    ) -> Option<&mut ItemWidget<'a>> {
        self._item_widgets
            .iter_mut()
            .find(|item_widget| item_widget._item_type == *item_data_type)
    }

    pub fn get_item_count(&self, item_data_type: &ItemDataType) -> usize {
        if let Some(item_widget) = self.find_item_widget(item_data_type) {
            return item_widget.get_item_count();
        }
        0
    }

    pub fn add_item(&mut self, item_data_type: &ItemDataType, item_count: usize) -> bool {
        if *item_data_type != ItemDataType::None
            && self._item_type_count < self._max_item_type_count
        {
            if let Some(item_widget) = self.find_item_widget_mut(item_data_type) {
                item_widget.add_item_count(item_count);
            } else {
                for item_widget in self._item_widgets.iter_mut() {
                    if item_widget._item_type == ItemDataType::None {
                        let material = ptr_as_ref(self._engine_resources)
                            .get_material_instance_data(
                                ItemDataType::get_item_material_instance_name(&item_data_type),
                            );
                        item_widget.set_item_data(
                            item_data_type,
                            Some(material.clone()),
                            item_count,
                        );
                        self._item_type_count += 1;
                        break;
                    }
                }
            }

            self.update_item_bar_widget();
            return true;
        }
        false
    }

    pub fn get_selected_item_type(&self) -> ItemDataType {
        if self._selected_item_widget._item_index != INVALID_ITEM_INDEX {
            return self._item_widgets[self._selected_item_widget._item_index]._item_type;
        }
        ItemDataType::None
    }

    pub fn remove_item(&mut self, item_data_type: &ItemDataType, item_count: usize) -> bool {
        if *item_data_type != ItemDataType::None {
            if let Some(item_widget) = self.find_item_widget_mut(item_data_type) {
                let item_count = item_widget.remove_item_count(item_count);
                if item_count == 0 {
                    item_widget.set_item_data(&ItemDataType::None, None, 0);
                    self._item_type_count -= 1;
                }
                return true;
            }
        }
        false
    }

    pub fn select_item_by_index(&mut self, item_index: usize) {
        if item_index < self._item_widgets.len()
            && self._item_widgets[item_index]._item_type != ItemDataType::None
        {
            self._selected_item_widget
                .update_selected_item_widget(item_index);
        } else {
            self._selected_item_widget
                .update_selected_item_widget(INVALID_ITEM_INDEX);
        }
    }

    pub fn select_next_item(&mut self) {
        if 0 < self._item_type_count {
            let start_index = if self._selected_item_widget.get_item_index() == INVALID_ITEM_INDEX
                || self._selected_item_widget.get_item_index() == (self._item_widgets.len() - 1)
            {
                0
            } else {
                self._selected_item_widget.get_item_index() + 1
            };

            for n in 0..self._item_widgets.len() {
                let mut item_index = start_index + n;
                if self._item_widgets.len() <= item_index {
                    item_index -= self._item_widgets.len();
                }

                if self._item_widgets[item_index]._item_type != ItemDataType::None {
                    self._selected_item_widget
                        .update_selected_item_widget(item_index);
                    break;
                }
            }
        } else {
            self._selected_item_widget
                .update_selected_item_widget(INVALID_ITEM_INDEX);
        }
    }

    pub fn select_previous_item(&mut self) {
        if 0 < self._item_type_count {
            let start_index = if self._selected_item_widget.get_item_index() == INVALID_ITEM_INDEX
                || 0 == self._selected_item_widget.get_item_index()
            {
                self._item_widgets.len() - 1
            } else {
                self._selected_item_widget.get_item_index() - 1
            };

            for n in 0..self._item_widgets.len() {
                let item_index = if start_index < n {
                    (start_index + self._item_widgets.len()) - n
                } else {
                    start_index - n
                };

                if self._item_widgets[item_index]._item_type != ItemDataType::None {
                    self._selected_item_widget
                        .update_selected_item_widget(item_index);
                    break;
                }
            }
        } else {
            self._selected_item_widget
                .update_selected_item_widget(INVALID_ITEM_INDEX);
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_center_hint_x(Some(0.5));
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM);

        let ui_component = ptr_as_mut(self._select_previous_item_widget).get_ui_component_mut();
        ui_component.set_pos_x((window_size.x as f32 - MAX_ITEM_TYPE_COUNT as f32 * ITEM_UI_SIZE) * 0.5 - ui_component.get_size_x());
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM);

        let ui_component = ptr_as_mut(self._select_next_item_widget).get_ui_component_mut();
        ui_component.set_pos_x((window_size.x as f32 + MAX_ITEM_TYPE_COUNT as f32 * ITEM_UI_SIZE) * 0.5);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM);
    }
}
