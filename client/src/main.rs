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
};

mod screens;
use screens::*;

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}

#[derive(Default, Debug)]
struct Layout {
    screen: Screen,
    state: AppState,
    example: Example,
    debug: bool,
    theme: Theme,
}

#[derive(Default, Debug, Clone)]
struct AppState {
    modules: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    ThemeSelected(Theme),
    ScreenSelected(Screen),
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - chain-tool", self.screen)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.example = self.example.next();
            }
            Message::Previous => {
                self.example = self.example.previous();
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::ScreenSelected(screen) => {
                self.screen = screen;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_release(|key, _modifiers| {
            match key {
                keyboard::Key::Named(key::Named::ArrowLeft) => { Some(Message::Previous) }
                keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
                _ => None,
            }
        })
    }

    fn view(&self) -> Element<'_, Message> {
        // let header = row![
        //     text(self.example.title).size(20).font(Font::MONOSPACE),
        //     horizontal_space(),
        //     checkbox("Explain", self.explain)
        //         .on_toggle(Message::ExplainToggled),
        //     pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        // ]
        // .spacing(20)
        // .align_y(Center);

        // let controls = row([
        //     (!self.example.is_first()).then_some(
        //         button("← Previous")
        //             .padding([5, 10])
        //             .on_press(Message::Previous)
        //             .into(),
        //     ),
        //     Some(horizontal_space().into()),
        //     (!self.example.is_last()).then_some(
        //         button("Next →")
        //             .padding([5, 10])
        //             .on_press(Message::Next)
        //             .into(),
        //     ),
        // ]
        // .into_iter()
        // .flatten());

        // column![header, example, controls]
        //     .spacing(10)
        //     .padding(20)
        //     .into()
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

        let screens = Screen::ALL;
        let sidebar_buttons = column(
            screens.iter().filter_map(|screen| {
                match screen {
                    Screen::Settings(_) => None,
                    _ =>
                        Some(
                            button(text(format!("{}", screen)))
                                .on_press(Message::ScreenSelected(screen.clone()))
                                .padding([5, 10])
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
                    .padding([5, 10])
                    .width(Fill)
            ]
                .spacing(40)
                .padding(10)
                .width(200)
            // .align_x(Center)
        )
            .style(container::rounded_box)
            .center_y(Fill);

        let screen_content = center(
            if self.debug {
                self.screen.view().explain(color!(0x0000ff))
            } else {
                self.screen.view()
            }
        )
            .style(|theme| {
                let palette = theme.extended_palette();

                container::Style
                    ::default()
                    .border(border::color(palette.background.strong.color).width(4))
            })
            .padding(4);

        let content = container(
            column![screen_content].spacing(40).align_x(Center).width(Fill)
        ).padding(10);

        column![header, row![sidebar, content]].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Example {
    title: &'static str,
    view: fn() -> Element<'static, Message>,
}

impl Example {
    const LIST: &'static [Self] = &[
        Self {
            title: "Centered",
            view: centered,
        },
        Self {
            title: "Column",
            view: column_,
        },
        Self {
            title: "Row",
            view: row_,
        },
        Self {
            title: "Space",
            view: space,
        },
        Self {
            title: "Application",
            view: application,
        },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST.get(index.saturating_sub(1)).copied().unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST.get(index + 1)
            .copied()
            .unwrap_or(self)
    }

    fn view(&self) -> Element<'static, Message> {
        (self.view)()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn centered<'a>() -> Element<'a, Message> {
    center(text("I am centered!").size(50)).into()
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically.",
        square(50),
        square(50),
        square(50),
        "The amount of space between",
        "elements can be configured!"
    ]
        .spacing(40)
        .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![
        "A row works like a column...",
        square(50),
        square(50),
        square(50),
        "but lays out widgets horizontally!"
    ]
        .spacing(40)
        .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![square(40), horizontal_space(), "Header!", horizontal_space(), square(40)]
            .padding(10)
            .align_y(Center)
    ).style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default().border(border::color(palette.background.strong.color).width(1))
    });

    let sidebar = container(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_x(Center)
    )
        .style(container::rounded_box)
        .center_y(Fill);

    let content = container(
        scrollable(
            column![
                "Content!",
                row((1..10).map(|i| square(if i % 2 == 0 { 80 } else { 160 })))
                    .spacing(20)
                    .align_y(Center)
                    .wrap(),
                "The end"
            ]
                .spacing(40)
                .align_x(Center)
                .width(Fill)
        ).height(Fill)
    ).padding(10);

    column![header, row![sidebar, content]].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(Point::ORIGIN, bounds.size(), palette.background.strong.color);

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
