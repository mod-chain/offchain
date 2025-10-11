use iced::widget::{center, text};
use iced::Element;
use crate::Message;
use super::ScreenView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModulesScreen {}

impl ScreenView for ModulesScreen {
    fn view(&self) -> Element<'_, Message> {
        center(text("Hello from ModulesScreen").size(20)).into()
    }
}