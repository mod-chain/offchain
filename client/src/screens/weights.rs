use iced::widget::{center, text};
use iced::Element;
use crate::Message;
use super::ScreenView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeightsScreen {}

impl ScreenView for WeightsScreen {
    fn view(&self) -> Element<'_, Message> {
        center(text("Hello from WeightsScreen").size(20)).into()
    }
}