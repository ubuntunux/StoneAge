use crate::game_module::actors::items::ItemEffect;
use crate::game_module::game_service_locator::get_game_resources;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_BOTTOM_LEFT, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

pub const ITEM_INFO_TITLE_FONT_SIZE: f32 = 30.0;
pub const ITEM_INFO_DESC_FONT_SIZE: f32 = 22.0;
pub const ITEM_INFO_HEADER_FONT_SIZE: f32 = 20.0;
pub const ITEM_INFO_STAT_FONT_SIZE: f32 = 20.0;

pub struct ItemStatEffect {
    pub _text: String,
    pub _color: u32,
}

pub struct ItemInfoWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _title_lbl: Rc<WidgetDefault<'a>>,
    pub _desc_lbl: Rc<WidgetDefault<'a>>,
    pub _stat_header_lbl: Rc<WidgetDefault<'a>>,
    pub _stat_lbls: Vec<Rc<WidgetDefault<'a>>>,
}

impl<'a> ItemInfoWidget<'a> {
    pub fn create_item_info_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<ItemInfoWidget<'a>> {
        let layer = UIManager::create_widget("inv_item_info_popup", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_pivot_preset(PIVOT_BOTTOM_LEFT);
        ui_component.set_size(0.0, 0.0);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(20, 25, 35, 255));
        ui_component.set_border(2.0);
        ui_component.set_border_color(get_color32(180, 200, 240, 255));
        ui_component.set_round(8.0);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        ui_component.set_visible(false);

        let title_lbl = UIManager::create_widget("inv_item_info_title", UIWidgetTypes::Default);
        let ui = ptr_as_mut(title_lbl.as_ref()).get_ui_component_mut();
        ui.set_size(0.0, 0.0);
        ui.set_font_size(ITEM_INFO_TITLE_FONT_SIZE);
        ui.set_font_color(get_color32(255, 255, 180, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_margin(4.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_expandable(true);

        let desc_lbl = UIManager::create_widget("inv_item_info_desc", UIWidgetTypes::Default);
        let ui = ptr_as_mut(desc_lbl.as_ref()).get_ui_component_mut();
        ui.set_size(0.0, 0.0);
        ui.set_font_size(ITEM_INFO_DESC_FONT_SIZE);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_margin(4.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_expandable(true);

        let stat_header_lbl = UIManager::create_widget("inv_item_info_stat_header", UIWidgetTypes::Default);
        let ui = ptr_as_mut(stat_header_lbl.as_ref()).get_ui_component_mut();
        ui.set_size(0.0, 0.0);
        ui.set_font_size(ITEM_INFO_HEADER_FONT_SIZE);
        ui.set_font_color(get_color32(140, 200, 255, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_margin_top(8.0);
        ui.set_margin_bottom(2.0);
        ui.set_margin_left(4.0);
        ui.set_margin_right(4.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_expandable(true);
        ui.set_visible(false);

        let layer_mut = ptr_as_mut(layer.as_ref());
        layer_mut.add_widget(&title_lbl);
        layer_mut.add_widget(&desc_lbl);
        layer_mut.add_widget(&stat_header_lbl);

        parent_widget.add_widget(&layer);

        Box::new(ItemInfoWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _title_lbl: title_lbl,
            _desc_lbl: desc_lbl,
            _stat_header_lbl: stat_header_lbl,
            _stat_lbls: Vec::new(),
        })
    }

    pub fn show_item_info(
        &mut self,
        item_data_name: &str,
        item_name: &str,
        item_count: usize,
        slot_widget: &WidgetDefault<'a>,
    ) {
        let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
        let slot_ui = slot_widget.get_ui_component();
        let slot_area = slot_ui.get_ui_area();
        let parent_area = ptr_as_ref(self._parent_widget).get_ui_component().get_ui_area();

        let popup_x = (slot_area.x - parent_area.x) / dpi_scale;
        let popup_y = (slot_area.y - parent_area.y) / dpi_scale;

        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_pos(popup_x, popup_y);
        layer_ui.set_visible(true);

        let title_ui = ptr_as_mut(self._title_lbl.as_ref()).get_ui_component_mut();
        let display_title = if item_count > 1 {
            format!("{} x{}", item_name, item_count)
        } else {
            item_name.to_string()
        };
        title_ui.set_text(&display_title);

        let desc_ui = ptr_as_mut(self._desc_lbl.as_ref()).get_ui_component_mut();
        let item_desc = Self::get_item_description_from_resource(item_data_name);
        desc_ui.set_text(&item_desc);

        let effects = Self::get_item_stat_effects(item_data_name);
        let header_ui = ptr_as_mut(self._stat_header_lbl.as_ref()).get_ui_component_mut();
        header_ui.set_visible(!effects.is_empty());

        while self._stat_lbls.len() < effects.len() {
            let i = self._stat_lbls.len();
            let stat_lbl = UIManager::create_widget(&format!("inv_item_info_stat_{}", i), UIWidgetTypes::Default);
            let ui = ptr_as_mut(stat_lbl.as_ref()).get_ui_component_mut();
            ui.set_size(0.0, 0.0);
            ui.set_font_size(ITEM_INFO_STAT_FONT_SIZE);
            ui.set_font_color(get_color32(220, 220, 220, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_margin_left(8.0);
            ui.set_margin_right(4.0);
            ui.set_margin_top(2.0);
            ui.set_margin_bottom(2.0);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_valign(VerticalAlign::TOP);
            ui.set_expandable(true);
            ui.set_visible(false);

            ptr_as_mut(self._layer.as_ref()).add_widget(&stat_lbl);
            self._stat_lbls.push(stat_lbl);
        }

        while self._stat_lbls.len() > effects.len() {
            if let Some(removed_lbl) = self._stat_lbls.pop() {
                ptr_as_mut(self._layer.as_ref()).remove_widget(removed_lbl.as_ref());
            }
        }

        for (i, stat_lbl) in self._stat_lbls.iter().enumerate() {
            let ui = ptr_as_mut(stat_lbl.as_ref()).get_ui_component_mut();
            ui.set_text(&effects[i]._text);
            ui.set_font_color(effects[i]._color);
            ui.set_visible(true);
        }
    }

    pub fn hide_item_info(&mut self) {
        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_visible(false);
    }

    pub fn get_item_description_from_resource(item_code: &str) -> String {
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            let desc = resources.get_item_data(item_code).borrow()._description.clone();
            if !desc.is_empty() {
                return desc;
            }
        }
        item_code.to_string()
    }

    pub fn get_item_stat_effects(item_code: &str) -> Vec<ItemStatEffect> {
        let mut effects = Vec::new();
        let resources = get_game_resources();
        if !resources.has_item_data(item_code) {
            return effects;
        }

        let item_data_ref = resources.get_item_data(item_code);
        let item_data = item_data_ref.borrow();

        for effect in &item_data._item_effects {
            match effect {
                ItemEffect::WeaponDamage(damage) => {
                    if *damage > 0.0 {
                        effects.push(ItemStatEffect {
                            _text: format!("ATK: +{:.0}", damage),
                            _color: get_color32(255, 120, 120, 255),
                        });
                    }
                }
                ItemEffect::WeaponRange(range) => {
                    if *range > 0.0 {
                        effects.push(ItemStatEffect {
                            _text: format!("Range: {:.1}m", range),
                            _color: get_color32(200, 180, 255, 255),
                        });
                    }
                }
                ItemEffect::Hp(hp) => {
                    if *hp != 0 {
                        effects.push(ItemStatEffect {
                            _text: format!("HP: {:+}", hp),
                            _color: get_color32(100, 230, 120, 255),
                        });
                    }
                }
                ItemEffect::Stamina(stamina) => {
                    if *stamina != 0.0 {
                        effects.push(ItemStatEffect {
                            _text: format!("Stamina: {:+.0}", stamina),
                            _color: get_color32(255, 220, 100, 255),
                        });
                    }
                }
                ItemEffect::Hunger(hunger) => {
                    if *hunger != 0.0 {
                        effects.push(ItemStatEffect {
                            _text: format!("Hunger: {:+.1}", hunger),
                            _color: get_color32(240, 170, 100, 255),
                        });
                    }
                }
            }
        }

        effects
    }
}
