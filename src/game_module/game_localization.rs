use crate::game_module::game_option::{Language, get_global_language};
use crate::game_module::game_service_locator::{get_game_resources, is_application_valid};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr, EnumIter)]
#[repr(usize)]
pub enum TextId {
    MenuTabInventory,
    MenuTabCraft,
    MenuTabRecords,
    MenuTabTaming,
    MenuTabFriendlyNpc,
    MenuTabSaveLoad,
    MenuTabOption,
    MenuTabDebug,
    StartMenuNewGame,
    StartMenuLoadGame,
    StartMenuOptions,
    StartMenuExit,
    ExitConfirmPrompt,
    PopupYes,
    PopupNo,
}

impl TextId {
    pub fn as_key(&self) -> &'static str {
        match self {
            TextId::MenuTabInventory => "MenuTabInventory",
            TextId::MenuTabCraft => "MenuTabCraft",
            TextId::MenuTabRecords => "MenuTabRecords",
            TextId::MenuTabTaming => "MenuTabTaming",
            TextId::MenuTabFriendlyNpc => "MenuTabFriendlyNpc",
            TextId::MenuTabSaveLoad => "MenuTabSaveLoad",
            TextId::MenuTabOption => "MenuTabOption",
            TextId::MenuTabDebug => "MenuTabDebug",
            TextId::StartMenuNewGame => "StartMenuNewGame",
            TextId::StartMenuLoadGame => "StartMenuLoadGame",
            TextId::StartMenuOptions => "StartMenuOptions",
            TextId::StartMenuExit => "StartMenuExit",
            TextId::ExitConfirmPrompt => "ExitConfirmPrompt",
            TextId::PopupYes => "PopupYes",
            TextId::PopupNo => "PopupNo",
        }
    }

    pub fn get_default_text_by_language(&self, language: Language) -> &'static str {
        match (self, language) {
            (TextId::MenuTabInventory, Language::English) => "Inventory",
            (TextId::MenuTabInventory, Language::Korean) => "인벤토리",

            (TextId::MenuTabCraft, Language::English) => "Craft",
            (TextId::MenuTabCraft, Language::Korean) => "제작",

            (TextId::MenuTabRecords, Language::English) => "Records",
            (TextId::MenuTabRecords, Language::Korean) => "기록",

            (TextId::MenuTabTaming, Language::English) => "Taming",
            (TextId::MenuTabTaming, Language::Korean) => "테이밍",

            (TextId::MenuTabFriendlyNpc, Language::English) => "NPC Friends",
            (TextId::MenuTabFriendlyNpc, Language::Korean) => "동료 NPC",

            (TextId::MenuTabSaveLoad, Language::English) => "Save / Load",
            (TextId::MenuTabSaveLoad, Language::Korean) => "저장 / 로드",

            (TextId::MenuTabOption, Language::English) => "Option",
            (TextId::MenuTabOption, Language::Korean) => "옵션",

            (TextId::MenuTabDebug, Language::English) => "Debug",
            (TextId::MenuTabDebug, Language::Korean) => "디버그",

            (TextId::StartMenuNewGame, Language::English) => "New Game",
            (TextId::StartMenuNewGame, Language::Korean) => "새 게임",

            (TextId::StartMenuLoadGame, Language::English) => "Load Game",
            (TextId::StartMenuLoadGame, Language::Korean) => "불러오기",

            (TextId::StartMenuOptions, Language::English) => "Options",
            (TextId::StartMenuOptions, Language::Korean) => "옵션",

            (TextId::StartMenuExit, Language::English) => "Exit",
            (TextId::StartMenuExit, Language::Korean) => "종료",

            (TextId::ExitConfirmPrompt, Language::English) => "Are you sure you want to exit?",
            (TextId::ExitConfirmPrompt, Language::Korean) => "게임을 종료하시겠습니까?",

            (TextId::PopupYes, Language::English) => "Yes",
            (TextId::PopupYes, Language::Korean) => "예",

            (TextId::PopupNo, Language::English) => "No",
            (TextId::PopupNo, Language::Korean) => "아니오",
        }
    }

    pub fn get_text(&self) -> String {
        get_text(*self)
    }

    pub fn get_text_by_language(&self, language: Language) -> String {
        get_text_by_language(*self, language)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
pub struct LocalizationData {
    pub _language: Language,
    pub _texts: HashMap<String, String>,
}

impl LocalizationData {
    pub fn new(language: Language) -> Self {
        Self {
            _language: language,
            _texts: HashMap::new(),
        }
    }

    pub fn get_text(&self, text_id: TextId) -> Option<&str> {
        self.get_text_by_key(text_id.as_key())
    }

    pub fn get_text_by_key(&self, key: &str) -> Option<&str> {
        self._texts.get(key).map(|s| s.as_str())
    }

    pub fn set_text(&mut self, text_id: TextId, text: impl Into<String>) {
        self._texts.insert(text_id.as_key().to_string(), text.into());
    }

    pub fn set_text_by_key(&mut self, key: impl Into<String>, text: impl Into<String>) {
        self._texts.insert(key.into(), text.into());
    }
}

pub fn get_text(text_id: TextId) -> String {
    get_text_by_language(text_id, get_global_language())
}

pub fn get_text_by_language(text_id: TextId, language: Language) -> String {
    if is_application_valid() {
        let game_resources = get_game_resources();
        return game_resources.get_text_by_language(text_id, language);
    }
    text_id.get_default_text_by_language(language).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_text_id_translations() {
        for text_id in TextId::iter() {
            let en = text_id.get_default_text_by_language(Language::English);
            let ko = text_id.get_default_text_by_language(Language::Korean);
            assert!(!en.is_empty(), "English translation missing for {:?}", text_id);
            assert!(!ko.is_empty(), "Korean translation missing for {:?}", text_id);
            assert_ne!(
                en, ko,
                "English and Korean translations should differ for {:?}",
                text_id
            );
        }

        assert_eq!(
            TextId::MenuTabInventory.get_default_text_by_language(Language::English),
            "Inventory"
        );
        assert_eq!(
            TextId::MenuTabInventory.get_default_text_by_language(Language::Korean),
            "인벤토리"
        );
        assert_eq!(
            TextId::MenuTabCraft.get_default_text_by_language(Language::Korean),
            "제작"
        );
        assert_eq!(
            TextId::MenuTabRecords.get_default_text_by_language(Language::Korean),
            "기록"
        );
        assert_eq!(
            TextId::MenuTabTaming.get_default_text_by_language(Language::Korean),
            "테이밍"
        );
        assert_eq!(
            TextId::MenuTabFriendlyNpc.get_default_text_by_language(Language::Korean),
            "동료 NPC"
        );
        assert_eq!(
            TextId::MenuTabSaveLoad.get_default_text_by_language(Language::Korean),
            "저장 / 로드"
        );
        assert_eq!(
            TextId::MenuTabOption.get_default_text_by_language(Language::Korean),
            "옵션"
        );
        assert_eq!(
            TextId::MenuTabDebug.get_default_text_by_language(Language::Korean),
            "디버그"
        );
    }

    #[test]
    fn test_localization_data_serde_roundtrip() {
        let mut loc = LocalizationData::new(Language::Korean);
        loc.set_text(TextId::MenuTabInventory, "인벤토리");
        loc.set_text(TextId::MenuTabCraft, "제작");
        loc.set_text_by_key("CustomKey", "커스텀");

        let json = serde_json::to_string_pretty(&loc).expect("Serialization failed");
        let deserialized: LocalizationData = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(deserialized._language, Language::Korean);
        assert_eq!(deserialized.get_text(TextId::MenuTabInventory), Some("인벤토리"));
        assert_eq!(deserialized.get_text(TextId::MenuTabCraft), Some("제작"));
        assert_eq!(deserialized.get_text_by_key("CustomKey"), Some("커스텀"));
    }
}
