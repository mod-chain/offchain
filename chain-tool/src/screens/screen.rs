use super::{ ModulesScreen, WeightsScreen, UsageScreen, WalletsScreen, SettingsScreen };
use crate::{ AppState, Message };
use iced::Element;

pub trait ScreenId {
  fn id(&self) -> &'static str;
}

pub trait ScreenView {
    fn view(&self, state: &AppState) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Screen {
    Modules(ModulesScreen),
    Weights(WeightsScreen),
    Usage(UsageScreen),
    Wallets(WalletsScreen),
    Settings(SettingsScreen),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Modules(ModulesScreen::default())
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Screen::Modules(_) => write!(f, "Modules"),
            Screen::Weights(_) => write!(f, "Weights"),
            Screen::Usage(_) => write!(f, "Usage"),
            Screen::Wallets(_) => write!(f, "Wallets"),
            Screen::Settings(_) => write!(f, "Settings"),
        }
    }
}

impl Screen {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Modules(ModulesScreen::default()),
            Self::Weights(WeightsScreen {}),
            Self::Usage(UsageScreen {}),
            Self::Wallets(WalletsScreen::default()),
            Self::Settings(SettingsScreen {}),
        ]
    }

    pub fn view<'a>(&self, state: AppState) -> Element<'_, Message> {
        match self {
            Screen::Modules(screen) => screen.view(&state),
            Screen::Weights(screen) => screen.view(&state),
            Screen::Usage(screen) => screen.view(&state),
            Screen::Wallets(screen) => screen.view(&state),
            Screen::Settings(screen) => screen.view(&state),
        }
    }
}

impl ScreenId for Screen {
  fn id(&self) -> &'static str {
    match self {
      Screen::Modules(screen) => screen.id(),
      Screen::Weights(screen) => screen.id(),
      Screen::Usage(screen) => screen.id(),
      Screen::Wallets(screen) => screen.id(),
      Screen::Settings(screen) => screen.id(),
    }
  }
}
