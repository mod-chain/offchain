use iced::widget::{ center, text, column };
use iced::{ Element, Subscription, Task };
use crate::{ AppState, ChainConfig, Message, Module };
use super::ScreenView;
use anyhow::Result;
use subxt::OnlineClient;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModulesScreen {}

#[derive(Debug, Clone)]
pub enum ModulesMessage {
    DataReceived(ModuleDataReceived),
    RefreshData,
}

#[derive(Debug, Clone)]
pub struct ModuleDataReceived {
    pub modules: Option<Vec<Module>>,
}

impl ScreenView for ModulesScreen {
    fn view(&self, state: &AppState) -> Element<'_, Message> {
        let modules_list = self.modules_list(state);
        column![modules_list].into()
    }
}

impl ModulesScreen {
    pub async fn get_modules() -> Result<Vec<Module>> {
        println!("In get_modules");
        let api = OnlineClient::<ChainConfig>::from_url("ws://127.0.0.1:9944").await?;
        let modules = Module::iter(&api).await?;

        Ok(modules)
    }

    pub fn update(state: &AppState, message: ModulesMessage) -> Task<Message> {
        match message {
            ModulesMessage::DataReceived(data_received) => {
                let mut new_state = state.clone();
                new_state.modules = data_received.modules.or(new_state.modules);
                Task::done(Message::StateUpdated(new_state))
            }
            ModulesMessage::RefreshData => {
                Task::perform(ModulesScreen::get_modules(), |value| {
                    match value {
                        Ok(modules) =>
                            Message::Modules(
                                ModulesMessage::DataReceived(ModuleDataReceived {
                                    modules: Some(modules),
                                })
                            ),
                        Err(e) => Message::Error(e.to_string()),
                    }
                })
            }
        }
    }

    pub fn subscription(state: &AppState) -> Subscription<Message> {
        Subscription::none()
    }

    fn modules_list(&self, state: &AppState) -> Element<'_, Message> {
        let modules_state = state.modules.clone();

        if let Some(modules) = modules_state {
            if modules.is_empty() {
                column![text("No Modules in network.")].into()
            } else {
                let mut col = column![];
                for m in modules {
                    col = col.push(text(m.name.to_string()));
                }
                col.into()
            }
        } else {
            column![text("Loading modules from chain...")].into()
        }
    }
}
