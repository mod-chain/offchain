use iced::widget::{ center, text };
use iced::Element;
use crate::{ AppState, Message };
use super::{ ScreenView, ScreenId };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsScreen {}

impl ScreenView for SettingsScreen {
    fn view(&self, _state: &AppState) -> Element<'_, Message> {
        center(text("Hello from SettingsScreen").size(20)).into()
    }
}

impl ScreenId for SettingsScreen {
    fn id(&self) -> &'static str {
        "settings"
    }
}

