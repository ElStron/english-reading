use iced::futures;
use iced::widget::{
    button, center, checkbox, column, horizontal_space,
    pick_list, row, text,
};
use iced::{
    color, Center, Element, Font, Length, Subscription, Theme,
};
mod ui;
mod messages;
use messages::Message;
use ui::{books_list_view, reader_view};
use ui::books::BooksList;

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
    books_list: BooksList,
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - Layout - Iced", self.example.title)
    }

    fn update(&mut self, message: Message) {
        let mut books = BooksList::new();
        match message {
            Message::NavigateTo(title) => {
                if let Some(example) = Example::find_by_title(title) {
                    // if title == "Books" {
                    //     if self.books_list.get_books().is_empty() ||self.books_list.get_books().iter().all(|b| b.handle_imagen.is_none()) {
                    //         futures::executor::block_on(books.fetch_images());
                    //         self.books_list = books;
                    //     }
                    // } 
                    self.example = example;
                }
            }
            Message::ExplainToggled(explain) => {
               
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::ImagesLoaded() => {
                if self.books_list.get_books().is_empty() ||self.books_list.get_books().iter().all(|b| b.handle_imagen.is_none()) {
                    futures::executor::block_on(books.fetch_images());
                    self.books_list = books;
                }
                
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

        let content = center(
            if self.explain {
                self.example.view(self).explain(color!(0x0000ff)) // Pasamos 'self'
            } else {
                self.example.view(self) // Pasamos 'self'
            },
        )
        .padding(4);

        let controls = column(
            Example::LIST
                .iter()
                .map(|example| {
                    button(example.title)
                        .style(|theme: &Theme, status| {
                            let palette = theme.extended_palette();
                            match status {
                                button::Status::Active => button::Style {
                                    background: Some(iced::Background::Color(
                                        palette.background.base.color
                                    )), 
                                    text_color: palette.background.weak.text, 
                                    ..Default::default() 
                                },
                                button::Status::Hovered => button::Style {
                                    background: Some(iced::Background::Color(
                                        palette.background.strong.color
                                    )),
                                    ..Default::default()
                                },
                                _ => button::Style::default()
                            }
                        })
                        .width(Length::Fill)
                        .padding([5, 10])
                        .on_press(
                            Message::NavigateTo(example.title)
                        )
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        ).spacing(10).width(200);

        let footer = text(
            "This is a footer").size(10);

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
    view: fn(&Layout) -> Element<'static, Message>,
}

impl Example {
    const LIST: &'static [Self] = &[
        Self {
            title: "Reader",
            view: reader_view,
        },
        Self {
            title: "Books",
            view: books_list_view,
        },
    ];

    fn find_by_title(title: &str) -> Option<Self> {
        Self::LIST
            .iter()
            .copied()
            .find(|example| example.title == title)
    }

    fn view(&self, layout: &Layout) -> Element<Message> { // Recibe una referencia a 'Layout'
        (self.view)(layout)  // Llama a la función vista pasándole 'layout'
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::LIST[0]
    }
}


