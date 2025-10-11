use super::{ ModulesScreen, WeightsScreen, UsageScreen, SettingsScreen };
use crate::Message;
use iced::Element;

pub trait ScreenView {
    fn view(&self) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Screen {
    Modules(ModulesScreen),
    Weights(WeightsScreen),
    Usage(UsageScreen),
    Settings(SettingsScreen),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Modules(ModulesScreen {})
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Screen::Modules(_) => write!(f, "Modules"),
            Screen::Weights(_) => write!(f, "Weights"),
            Screen::Usage(_) => write!(f, "Usage"),
            Screen::Settings(_) => write!(f, "Settings"),
        }
    }
}

impl Screen {
    pub const ALL: &'static [Self] = &[
        Self::Modules(ModulesScreen {}),
        Self::Weights(WeightsScreen {}),
        Self::Usage(UsageScreen {}),
        Self::Settings(SettingsScreen {}),
    ];

    pub fn view<'a>(&self) -> Element<'_, Message> {
        match self {
            Screen::Modules(screen) => screen.view(),
            Screen::Weights(screen) => screen.view(),
            Screen::Usage(screen) => screen.view(),
            Screen::Settings(screen) => screen.view(),
        }
    }
}
