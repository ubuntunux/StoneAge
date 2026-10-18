use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::MATERIAL_UI_POINTER;
use crate::game_module::game_service_locator::{get_game_ui_manager, get_game_ui_manager_mut};
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::widgets::quest_widgets::quest_widget::QuestItemSaveData;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxItemOption, TextBoxLayerType};
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::utilities::system::RcRefCell;

pub const POINTER_ICON_SIZE: Vector2<f32> = Vector2::new(50.0, 50.0);

pub fn add_prop_pointer_text_box(prop: &Option<RcRefCell<Prop>>, check_existing: bool) {
    if let Some(prop_ref) = prop.as_ref() {
        let wrapper = ActorWrapper::Prop(prop_ref.clone());
        if check_existing && get_game_ui_manager().has_text_box_item(wrapper.get_key()) {
            return;
        }
        let contents = vec![TextBoxContent::MaterialInstance(
            MATERIAL_UI_POINTER.to_string(),
            Some(POINTER_ICON_SIZE),
        )];
        get_game_ui_manager_mut().add_text_box_item(
            wrapper,
            &contents,
            &TextBoxItemOption {
                _layer_type: TextBoxLayerType::GamePlayLayer,
                _bounce: true,
                _visible_background: false,
                ..Default::default()
            },
        );
    }
}

pub fn remove_prop_text_box(prop: &Option<RcRefCell<Prop>>) {
    if let Some(prop_ref) = prop.as_ref() {
        let wrapper = ActorWrapper::Prop(prop_ref.clone());
        get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
    }
}

pub fn is_quest_item_completed(quest_item: &Option<QuestItem>) -> bool {
    quest_item.as_ref().is_some_and(|q| q.borrow().is_completed_quest())
}

pub fn complete_quest_item(quest_item: &Option<QuestItem>) -> bool {
    if let Some(q) = quest_item
        && !q.borrow().is_completed_quest()
    {
        q.borrow_mut().set_completed_quest();
        return true;
    }
    false
}

pub fn get_quest_item_save_data(quest_item: &Option<QuestItem>) -> Option<QuestItemSaveData> {
    quest_item.as_ref().map(|q| q.borrow().get_quest_item_save_data())
}

pub fn load_quest_item_save_data(quest_item: &Option<QuestItem>, save_data: &Option<QuestItemSaveData>) {
    if let (Some(q), Some(data)) = (quest_item, save_data) {
        q.borrow_mut().load_quest_item_save_data(data);
    }
}

pub fn set_actors_behavior_none(actors: &[&Option<RcRefCell<Character>>], clear_action: bool) {
    for actor in actors.iter().copied().flatten() {
        let mut actor_mut = actor.borrow_mut();
        actor_mut.set_behavior_none();
        if clear_action {
            actor_mut.set_action_none();
        }
    }
}

pub fn set_actors_flying_sleep_mode(
    actors: &[&Option<RcRefCell<Character>>],
    clear_behavior: bool,
    position: Option<&Vector3<f32>>,
) {
    for actor in actors.iter().copied().flatten() {
        let mut actor_mut = actor.borrow_mut();
        actor_mut._controller.set_flying_mode(true);
        if clear_behavior {
            actor_mut.set_behavior_none();
        }
        actor_mut.set_action_sleep_no_snoring();
        if let Some(pos) = position {
            actor_mut.set_position(pos);
        }
    }
}

pub fn set_actors_visible(actors: &[&Option<RcRefCell<Character>>], visible: bool) {
    for actor in actors.iter().copied().flatten() {
        actor.borrow()._render_object.borrow_mut().set_visible(visible);
    }
}

pub fn wake_up_npc_on_bed(actor: &Option<RcRefCell<Character>>, bed: Option<&Option<RcRefCell<Prop>>>) {
    if let Some(actor_ref) = actor {
        let mut actor_mut = actor_ref.borrow_mut();
        actor_mut._controller.set_flying_mode(false);
        if let Some(Some(bed_ref)) = bed {
            actor_mut.set_position(bed_ref.borrow().get_position());
        }
        actor_mut.set_next_behavior(BehaviorState::WakeUp, true);
    }
}
