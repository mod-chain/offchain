use iced::widget::{ center, text };
use iced::Element;
use crate::{ AppState, Message };
use super::{ ScreenView, ScreenId };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsageScreen {}

impl ScreenView for UsageScreen {
    fn view(&self, state: &AppState) -> Element<'_, Message> {
        center(text("Hello from UsageScreen").size(20)).into()
    }
}

impl ScreenId for UsageScreen {
    fn id(&self) -> &'static str {
        "usage"
    }
}
