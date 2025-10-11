use iced::widget::{center, text};
use iced::Element;
use crate::Message;
use super::ScreenView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsScreen {}

impl ScreenView for SettingsScreen {
    fn view(&self) -> Element<'_, Message> {
        center(text("Hello from SettingsScreen").size(20)).into()
    }
}