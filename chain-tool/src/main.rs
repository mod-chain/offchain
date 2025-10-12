// use subxt_signer::sr25519;

// fn main() {
//     let keypair = sr25519::dev::alice();
//     let message = b"123";

//     let signature = keypair.sign(message);
//     let public_key = keypair.public_key();

//     println!("{:?}", message);
//     println!("{:?}", signature.0);
//     println!("0x{:?}", hex::encode(signature.0));
//     println!("bytes length: {}", signature.0.len());
//     println!("{:?}", public_key.0);

use std::fmt::Display;

use iced::Background;
use iced::alignment::Vertical::Top;
//     assert!(sr25519::verify(&signature, message, &public_key));
// }
use iced::border;
use iced::keyboard;
use iced::mouse;
use iced::widget::{
    button,
    canvas,
    center,
    checkbox,
    column,
    container,
    horizontal_space,
    vertical_space,
    pick_list,
    row,
    scrollable,
    text,
};
use iced::{
    color,
    Center,
    Element,
    Fill,
    Font,
    Length,
    Point,
    Rectangle,
    Renderer,
    Subscription,
    Theme,
    Task,
};

mod ext;
pub use ext::*;
mod chain;
pub use chain::*;
mod screens;
use screens::*;

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        // .run()
        .run_with(|| (Layout::default(), Task::done(Message::Modules(ModulesMessage::RefreshData))))
}

#[derive(Default, Debug)]
struct Layout {
    screen: Screen,
    state: AppState,
    debug: bool,
    theme: Theme,
}

#[derive(Default, Debug, Clone)]
pub struct AppState {
    modules: Option<Vec<Module>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    ScreenSelected(Screen),
    StateUpdated(AppState),
    Modules(screens::ModulesMessage),
    Error(String),
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - chain-tool", self.screen)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("In primary update");
        println!("{:?}", message);
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::ScreenSelected(screen) => {
                self.screen = screen;
                Task::none()
            }
            Message::StateUpdated(state) => {
                self.state = state;
                Task::none()
            }
            Message::Modules(message) => ModulesScreen::update(&self.state, message),
            // TODO: Implement
            Message::Error(e) => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        Subscription::batch([
            ModulesScreen::subscription(&self.state),
            keyboard::on_key_release(|key, _modifiers| {
                match key {
                    // TODO: Hotkeys to refresh data/info
                    keyboard::Key::Named(key::Named::F5) =>
                        Some(Message::Modules(ModulesMessage::RefreshData)),
                    // keyboard::Key::Named(key::Named::ArrowLeft) => { Some(Message::Previous) }
                    // keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
                    _ => None,
                }
            }),
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        let header = container(
            row![
                text(self.title()).size(16).font(Font::MONOSPACE),
                horizontal_space(),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected)
            ]
                .padding(10)
                .spacing(8)
                .align_y(Center)
        ).style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style
                ::default()
                .border(border::color(palette.background.strong.color).width(1))
        });

        let screens = Screen::all();
        let sidebar_buttons = column(
            screens.iter().filter_map(|screen| {
                match screen {
                    Screen::Settings(_) => None,
                    _ =>
                        Some(
                            button(text(format!("{}", screen)))
                                .on_press(Message::ScreenSelected(screen.clone()))
                                .padding([5, 10])
                                .style(
                                    if self.screen.id() == screen.id() {
                                        button::primary
                                    } else {
                                        button::secondary
                                    }
                                )
                                .width(Fill)
                                .into()
                        ),
                }
            })
        ).spacing(4);

        let sidebar = container(
            column![
                sidebar_buttons,
                vertical_space(),
                button(text(format!("{}", Screen::Settings(SettingsScreen {}))))
                    .on_press(Message::ScreenSelected(Screen::Settings(SettingsScreen {})))
                    .style(
                        if self.screen.id() == "settings" {
                            button::primary
                        } else {
                            button::secondary
                        }
                    )
                    .padding([5, 10])
                    .width(Fill)
            ]
                .spacing(40)
                .padding(10)
                .width(200)
            // .align_x(Center)
        )
            .style(container::bordered_box)
            .center_y(Fill);

        let screen_content = center(
            if self.debug {
                self.screen.view(self.state.clone()).explain(color!(0x0000ff))
            } else {
                self.screen.view(self.state.clone())
            }
        )
            .style(|theme| {
                let palette = theme.extended_palette();

                container::Style::default()
                // .border(border::color(palette.background.strong.color).width(4))
            })
            .padding(4);

        let content = container(screen_content).padding(10);

        column![header, row![sidebar, content].align_y(Top)].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
