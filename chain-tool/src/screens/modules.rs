use iced::Alignment::Center;
use iced::Length::Fill;
use iced::alignment::Vertical::Top;
use iced::{ Length, border };
use iced::widget::{
    button,
    column,
    container,
    pick_list,
    row,
    scrollable,
    slider,
    text,
    text_input,
};
use iced::{ Element, Subscription, Task, Theme };
use sp_arithmetic::Percent;
use crate::chain::ModuleName;
use crate::{ AppState, ChainConfig, Message, Module, ModuleTier };
use super::{ ScreenView, ScreenId };
use anyhow::Result;
use subxt::OnlineClient;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModulesScreen {
    pub selected_module: Module,
}

#[derive(Debug, Clone)]
pub enum ModulesMessage {
    DataReceived(ModuleDataReceived),
    RefreshData,
    Register(Module),
}

#[derive(Debug, Clone)]
pub struct ModuleDataReceived {
    pub modules: Option<Vec<Module>>,
    pub authorized_module: u64,
}

impl ScreenId for ModulesScreen {
    fn id(&self) -> &'static str {
        "modules"
    }
}

impl ScreenView for ModulesScreen {
    fn view(&self, state: &AppState) -> Element<'_, Message> {
        let module_header = self.module_header(state);
        let modules_list = self.modules_list(state);
        let module_editor = self.module_editor(state);

        column![module_header, row![modules_list, module_editor].align_y(Top).spacing(8.0)]
            .spacing(4.0)
            .height(Fill)
            .into()
    }
}

impl ModulesScreen {
    pub fn new() -> Self {
        Self {
            selected_module: Module::new(),
        }
    }
    pub async fn refresh_data() -> Result<(Vec<Module>, u64)> {
        println!("get_modules");
        let api = OnlineClient::<ChainConfig>::from_url("ws://127.0.0.1:9944").await?;
        let modules = Module::iter(&api).await?;
        let authorized_module = Module::authorized_module(&api).await?;

        Ok((modules, authorized_module))
    }

    pub async fn register_module(&self, state: &AppState) -> Result<()> {
        println!("register_module");
        let api = OnlineClient::<ChainConfig>::from_url("ws://127.0.0.1:9944").await?;

        self.selected_module.register(&api, state).await
    }

    pub fn update(&self, state: &AppState, message: ModulesMessage) -> Task<Message> {
        match message {
            ModulesMessage::DataReceived(data_received) => {
                let mut new_state = state.clone();
                new_state.modules = data_received.modules.or(new_state.modules);
                new_state.authorized_module = data_received.authorized_module;
                Task::done(Message::StateUpdated(new_state))
            }
            ModulesMessage::RefreshData => {
                Task::perform(ModulesScreen::refresh_data(), |value| {
                    match value {
                        Ok((modules, authorized_module)) =>
                            Message::Modules(
                                ModulesMessage::DataReceived(ModuleDataReceived {
                                    modules: Some(modules),
                                    authorized_module,
                                })
                            ),
                        Err(e) => Message::Error(e.to_string()),
                    }
                })
            }
            ModulesMessage::Register(module) => {
                let current_state = state.clone();
                Task::perform(
                    async move {
                        let api = OnlineClient::<ChainConfig>::from_url(
                            "ws://127.0.0.1:9944"
                        ).await?;
                        module.register(&api, &current_state).await
                    },
                    |value| {
                        match value {
                            Ok(_) => { Message::Modules(ModulesMessage::RefreshData) }
                            Err(e) => Message::Error(e.to_string()),
                        }
                    }
                )
            }
        }
    }

    pub fn subscription(_: &AppState) -> Subscription<Message> {
        Subscription::none()
    }

    fn module_header(&self, state: &AppState) -> Element<'_, Message> {
        let new_button = container(
            button(
                text("Clear/New")
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        text::Style {
                            color: Some(palette.primary.strong.text),
                        }
                    })
                    .size(11.0)
                    .center()
            )
                .on_press_maybe(match self.selected_module.id {
                    Some(_) =>
                        Some(
                            Message::ScreenSelected(
                                crate::screens::Screen::Modules(ModulesScreen {
                                    selected_module: Module::new(),
                                })
                            )
                        ),
                    None => None,
                })
                .width(200)
        ).padding(8.0);
        let module_title = match self.selected_module.id {
            Some(_) => self.selected_module.name.to_string(),
            None => String::from("New Module"),
        };
        let authorized_module_flag = match self.selected_module.id {
            Some(id) => {
                if id == state.authorized_module {
                    text("Authorized Module").style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        text::Style {
                            color: Some(palette.secondary.base.text),
                        }
                    }).size(11.0)
                } else {
                    text("")
                }
            }
            None => text(""),
        };

        row![new_button, text(module_title).size(20.0), authorized_module_flag]
            .spacing(8.0)
            .align_y(Center)
            .into()
    }

    fn modules_list(&self, state: &AppState) -> Element<'_, Message> {
        let modules_state = state.modules.clone();

        if let Some(modules) = modules_state {
            let mut col = column![];
            if modules.is_empty() {
                // column![text("No Modules in network.")].into()
                col = col.push(
                    button(container(text("No Modules in Network")))
                        .style(button::secondary)
                        .width(Length::Fixed(200.0))
                );
            } else {
                for m in modules {
                    col = col.push(
                        button(
                            container(
                                row![
                                    text(m.id.unwrap()).width(40.0),
                                    text(m.name.to_string())
                                ].align_y(Center)
                            )
                        )
                            .on_press(
                                Message::ScreenSelected(
                                    crate::screens::Screen::Modules(ModulesScreen {
                                        selected_module: m.clone(),
                                    })
                                )
                            )
                            .style(
                                if self.selected_module.id == m.id {
                                    button::primary
                                } else {
                                    button::secondary
                                }
                            )
                            .width(Length::Fixed(200.0))
                    );
                }
            }
            container(scrollable(col))
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();

                    container::Style
                        ::default()
                        .border(
                            border::color(palette.background.strong.color).width(1).rounded(8.0)
                        )
                })
                .max_height(400.0)
                .padding(8.0)
                .into()
        } else {
            column![text("Loading modules from chain...")].into()
        }
    }

    fn module_editor(&self, state: &AppState) -> Element<'_, Message> {
        let modules_state = state.modules.clone();

        if let Some(_modules) = modules_state {
            let module_inputs = match &self.selected_module.id {
                Some(_v) => {
                    // Edit mode - show read-only fields for existing modules
                    column([
                        row![
                            text("Owner").width(120.0),
                            text_input("Owner", &self.selected_module.owner)
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("ID").width(120.0),
                            text_input("ID", &self.selected_module.id.unwrap().to_string())
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Name").width(120.0),
                            text_input("Name", &self.selected_module.name.to_string())
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Data").width(120.0),
                            text_input("Data", &format!("{:?}", &self.selected_module.data))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("URL").width(120.0),
                            text_input("URL", &format!("{:?}", &self.selected_module.url))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Collateral").width(120.0),
                            text_input(
                                "Collateral",
                                &format!("{:?}", &self.selected_module.collateral)
                            )
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Take").width(120.0),
                            container(
                                slider(
                                    0u8..=100u8,
                                    self.selected_module.take.deconstruct(),
                                    |value| {
                                        Message::ScreenSelected(
                                            crate::screens::Screen::Modules(ModulesScreen {
                                                selected_module: Module {
                                                    take: Percent::from_percent(value),
                                                    ..self.selected_module.clone()
                                                },
                                            })
                                        )
                                    }
                                )
                            ),
                            text(format!("{:?}", self.selected_module.take))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .height(32)
                            .into(),
                        row![
                            text("Tier").width(120.0),
                            pick_list(
                                ModuleTier::all(),
                                Some(self.selected_module.tier.clone()),
                                |value| {
                                    Message::ScreenSelected(
                                        crate::screens::Screen::Modules(ModulesScreen {
                                            selected_module: Module {
                                                tier: value,
                                                ..self.selected_module.clone()
                                            },
                                        })
                                    )
                                }
                            ).width(Fill)
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Created At").width(120.0),
                            text_input(
                                "Created At",
                                &format!("{}", &self.selected_module.created_at)
                            )
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Last Updated").width(120.0),
                            text_input(
                                "Last Updated",
                                &format!("{}", &self.selected_module.last_updated)
                            )
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                    ]).padding([4.0, 8.0])
                }
                None => {
                    // Create mode - show editable fields for new modules
                    column([
                        row![
                            text("Owner").width(120.0),
                            match &state.wallets {
                                Some(wallets) =>
                                    container(
                                        pick_list(
                                            wallets
                                                .iter()
                                                .map(|w| w.public_key.clone())
                                                .collect::<Vec<String>>(),
                                            Some(&self.selected_module.owner),
                                            |value| {
                                                Message::ScreenSelected(
                                                    crate::screens::Screen::Modules(ModulesScreen {
                                                        selected_module: Module {
                                                            owner: value,
                                                            ..self.selected_module.clone()
                                                        },
                                                    })
                                                )
                                            }
                                        )
                                    ),
                                None => container(text("Loading wallets from config...")),
                            }
                            // text_input("Owner", &self.selected_module.owner)
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Name").width(120.0),
                            text_input("Name", &self.selected_module.name.to_string()).on_input(
                                |value| {
                                    Message::ScreenSelected(
                                        crate::screens::Screen::Modules(ModulesScreen {
                                            selected_module: Module {
                                                name: ModuleName(value.as_bytes().to_vec()),
                                                ..self.selected_module.clone()
                                            },
                                        })
                                    )
                                }
                            )
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Data").width(120.0),
                            text_input("Data", &format!("{:?}", &self.selected_module.data))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("URL").width(120.0),
                            text_input("URL", &format!("{:?}", &self.selected_module.url))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Collateral").width(120.0),
                            text_input(
                                "Collateral",
                                &format!("{:?}", &self.selected_module.collateral)
                            )
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            text("Take").width(120.0),
                            container(
                                slider(
                                    0u8..=100u8,
                                    self.selected_module.take.deconstruct(),
                                    |value| {
                                        Message::ScreenSelected(
                                            crate::screens::Screen::Modules(ModulesScreen {
                                                selected_module: Module {
                                                    take: Percent::from_percent(value),
                                                    ..self.selected_module.clone()
                                                },
                                            })
                                        )
                                    }
                                )
                            ),
                            text(format!("{:?}", self.selected_module.take))
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .height(32)
                            .into(),
                        row![
                            text("Tier").width(120.0),
                            pick_list(
                                ModuleTier::all(),
                                Some(self.selected_module.tier.clone()),
                                |value| {
                                    Message::ScreenSelected(
                                        crate::screens::Screen::Modules(ModulesScreen {
                                            selected_module: Module {
                                                tier: value,
                                                ..self.selected_module.clone()
                                            },
                                        })
                                    )
                                }
                            ).width(Fill)
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                        row![
                            button(text("Submit")).on_press_maybe(match
                                !self.selected_module.name.to_string().is_empty()
                            {
                                true =>
                                    Some(
                                        Message::Modules(
                                            ModulesMessage::Register(self.selected_module.clone())
                                        )
                                    ),
                                false => None,
                            })
                        ]
                            .align_y(Center)
                            .spacing(4.0)
                            .into(),
                    ]).padding([4.0, 8.0])
                }
            };

            scrollable(
                container(module_inputs)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        container::Style
                            ::default()
                            .border(
                                border::color(palette.background.strong.color).width(1).rounded(8.0)
                            )
                    })
                    .padding([0.0, 8.0])
                    .max_height(400.0)
            ).into()
        } else {
            column![text("Loading modules from chain...")].into()
        }
    }
}
