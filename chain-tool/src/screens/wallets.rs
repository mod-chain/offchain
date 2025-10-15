use iced::widget::{ button, column, container, row, scrollable, text, text_input };
use iced::{
    Element,
    Task,
    Theme,
    border,
    Length::{ self, Fill },
    Alignment::Center,
    alignment::Vertical::Top,
};
use crate::{ AppState, Message, Wallet, WalletDerivationMethod };
use super::{ ScreenView, ScreenId };
use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WalletsScreen {
    pub selected_wallet: Wallet,
}

#[derive(Debug, Clone)]
pub enum WalletsMessage {
    DataReceived(WalletDataReceived),
    RefreshData,
}

#[derive(Debug, Clone)]
pub struct WalletDataReceived {
    pub wallets: Option<Vec<Wallet>>,
}

impl ScreenView for WalletsScreen {
    fn view(&self, state: &AppState) -> Element<'_, Message> {
        let wallet_header = self.wallet_header(state);
        let wallets_list = self.wallets_list(state);
        let wallet_editor = self.wallet_editor(state);

        column![wallet_header, row![wallets_list, wallet_editor].align_y(Top).spacing(8.0)]
            .spacing(4.0)
            .height(Fill)
            .into()
    }
}

impl ScreenId for WalletsScreen {
    fn id(&self) -> &'static str {
        "wallets"
    }
}

impl WalletsScreen {
    pub async fn get_wallets() -> Result<Vec<Wallet>> {
        println!("get_wallets");
        Ok(Wallet::load_wallets(None).unwrap())
    }

    pub fn update(state: &AppState, message: WalletsMessage) -> Task<Message> {
        match message {
            WalletsMessage::DataReceived(data_received) => {
                let mut new_state = state.clone();
                new_state.wallets = data_received.wallets.or(new_state.wallets);
                Task::done(Message::StateUpdated(new_state))
            }
            WalletsMessage::RefreshData => {
                Task::perform(WalletsScreen::get_wallets(), |value| {
                    match value {
                        Ok(wallets) =>
                            Message::Wallets(
                                WalletsMessage::DataReceived(WalletDataReceived {
                                    wallets: Some(wallets),
                                })
                            ),
                        Err(e) => Message::Error(e.to_string()),
                    }
                })
            }
        }
    }

    fn wallet_header(&self, _state: &AppState) -> Element<'_, Message> {
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
                .on_press_maybe(match self.selected_wallet.id {
                    Some(_) =>
                        Some(
                            Message::ScreenSelected(
                                crate::screens::Screen::Wallets(WalletsScreen {
                                    selected_wallet: Wallet::new(),
                                })
                            )
                        ),
                    None => None,
                })
                .width(200)
        ).padding(8.0);
        let wallet_title = match self.selected_wallet.id {
            Some(_) => self.selected_wallet.name.to_string(),
            None => String::from("New Wallet"),
        };

        row![new_button, text(wallet_title).size(20.0)].spacing(8.0).align_y(Center).into()
    }

    fn wallets_list(&self, state: &AppState) -> Element<'_, Message> {
        let wallets_state = state.wallets.clone();

        if let Some(wallets) = wallets_state {
            if wallets.is_empty() {
                column![text("No Wallets available.")].into()
            } else {
                let mut col = column![];
                for (i, w) in wallets.iter().enumerate() {
                    col = col.push(
                        button(
                            container(
                                row![
                                    text(format!("{}", i)).width(40.0),
                                    text(w.name.clone())
                                ].align_y(Center)
                            )
                        )
                            .on_press(
                                Message::ScreenSelected(
                                    crate::screens::Screen::Wallets(WalletsScreen {
                                        selected_wallet: w.clone(),
                                    })
                                )
                            )
                            .style(
                                if self.selected_wallet.public_key == w.public_key {
                                    button::primary
                                } else {
                                    button::secondary
                                }
                            )
                            .width(Length::Fixed(200.0))
                    );
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
            }
        } else {
            column![text("Loading wallets from config directory...")].into()
        }
    }

    fn wallet_editor(&self, state: &AppState) -> Element<'_, Message> {
        let wallets_state = state.wallets.clone();

        if let Some(_wallets) = wallets_state {
            container(
                column([
                    row![
                        text("ID").width(120.0),
                        text_input(
                            "ID",
                            &self.selected_wallet.id.clone().unwrap_or("Not Created".to_string())
                        )
                    ]
                        .align_y(Center)
                        .spacing(4.0)
                        .into(),
                    row![text("Name").width(120.0), text_input("Name", &self.selected_wallet.name)]
                        .align_y(Center)
                        .spacing(4.0)
                        .into(),
                    row![
                        text("Address").width(120.0),
                        text_input("Address", &self.selected_wallet.public_key)
                    ]
                        .align_y(Center)
                        .spacing(4.0)
                        .into(),
                    row![
                        text("Derivation Method").width(120.0),
                        text_input("Derivation Method", match &self.selected_wallet.derivation {
                            WalletDerivationMethod::None => "None",
                            WalletDerivationMethod::Mnemonic(mnemonic) => mnemonic,
                            WalletDerivationMethod::SecretURI(secret_uri) => secret_uri,
                        })
                    ]
                        .align_y(Center)
                        .spacing(4.0)
                        .into(),
                ])
            )
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
            column![text("Loading wallets from config directory...")].into()
        }
    }
}
