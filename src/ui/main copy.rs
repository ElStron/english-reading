use iced::mouse;
use iced::widget::{
    button, canvas, center, checkbox, column, horizontal_space,
    pick_list, row, text, vertical_rule,
};
use iced::{
    color, Center, Element, Font, Length, Point, Rectangle, Renderer, Subscription, Theme,
};
mod ui;
use ui::reader_view;

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}

#[derive(Default, Debug)]
struct Layout {
    example: Example,
    explain: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(&'static str),
    ExplainToggled(bool),
    ThemeSelected(Theme),
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - Layout - Iced", self.example.title)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(title) => {
                if let Some(example) = Example::find_by_title(title) {
                    self.example = example;
                }
            }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none() // Eliminamos la navegación con teclado
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text(self.example.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain)
                .on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_y(Center);

        let content = center(if self.explain {
            self.example.view().explain(color!(0x0000ff))
        } else {
            self.example.view()
        })
        .padding(4);

        let controls = column(
            Example::LIST
                .iter()
                .map(|example| {
                    button(example.title)
                        .style(|theme: &Theme, status| {
                            let palette = theme.extended_palette();
                    
                            match status {
                                button::Status::Active => {
                                    button::Style::default()
                                    .with_background(
                                        iced::Background::Color(iced::Color::TRANSPARENT)
                                    )
                                },
                                button::Status::Hovered => {
                                    button::Style::default()
                                    .with_background(
                                        palette.background.strong.color
                                    )
                                },
                                button::Status::Pressed => {
                                    button::Style::default()
                                    .with_background(palette.background.strong.color)
                                },
                                _ => button::text(theme, status)
                            }
                        })
                        .width(Length::Fill)
                        .padding([5, 10])
                        .on_press(Message::NavigateTo(example.title))
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        ).spacing(10).width(200);

        let footer = text("This is a footer").size(10);

        let content_and_sidebar = row![
            controls,
            content
        ].spacing(10).padding(20);

        column![header, content_and_sidebar, footer]
            .spacing(10)
            .padding(20)
            .into()
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
            view: reader_view,
        },
        Self {
            title: "Quotes",
            view: quotes,
        },
        Self {
            title: "Pinning",
            view: pinning,
        }
    ];

    fn find_by_title(title: &str) -> Option<Self> {
        Self::LIST.iter().copied().find(|example| example.title == title)
    }

    fn view(&self) -> Element<Message> {
        (self.view)()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::LIST[0]
    }
}

// Definimos las vistas de ejemplo
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
        "elements can be configured!",
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
        "but lays out widgets horizontally!",
    ]
    .spacing(40)
    .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn quotes<'a>() -> Element<'a, Message> {
    column![
        "Quotes example...",
        row!["Quote 1", vertical_rule(2), "Reply 1"],
        row!["Quote 2", vertical_rule(2), "Reply 2"]
    ]
    .spacing(10)
    .into()
}

fn pinning<'a>() -> Element<'a, Message> {
    column![
        "Example of pinning elements...",
        square(50),
        square(100),
        square(150)
    ]
    .spacing(10)
    .into()
}

// Widget cuadrado de ejemplo
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
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());
            let palette = theme.extended_palette();
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );
            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}

// let image_widget = if book.imagen.is_empty() { // Check for empty path
//     text("No Image").into() // Or a placeholder image
// } else {
//     match image::Image::new(image::Source::from_path(&book.imagen)) {
//         Ok(image) => image
//             .width(Length::Units(100)) // Set desired width
//             .height(Length::Units(150)) // Set desired height
//             .into(),
//         Err(err) => {
//             eprintln!("Error loading image: {}", err);
//             text(format!("Error: {}", err)).into()
//         }
//     }
// };
