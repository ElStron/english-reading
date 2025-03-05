mod messages;
mod ui;
mod utils;
//use ui::about

use iced::widget::{button, center, checkbox, column, horizontal_space, pick_list, row, text};
use iced::{color, window, Center, Element, Font, Length, Subscription, Task, Theme};

use messages::Message;
use ui::books::BooksList;
use ui::books_details::BookDetails;
use ui::{books_details_view, books_list_view, reader_view};

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .window(window::Settings {
            icon: Some(utils::load_icon()),
            transparent: true,
            ..Default::default()
        })
        .run()
}

#[derive(Default, Debug)]
struct Layout {
    views: Views,
    title: String,
    explain: bool,
    theme: Theme,
    books_list: BooksList,
    loading_books: bool,
    loading_data: bool,
    current_book: Option<BookDetails>,
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - LexiLearn", self.title)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(title) => {
                self.title = title.to_string();
                if let Some(view) = Views::find_by_title(title) {
                    self.views = view;
                    if title == "Books" && self.books_list.get_books().is_empty()
                        || self
                            .books_list
                            .get_books()
                            .iter()
                            .all(|b| b.handle_imagen.is_none())
                    {
                        self.loading_books = true; // ⏳ Activar el estado de carga

                        let future = async move {
                            let mut books = BooksList::new();
                            books.fetch_images().await;
                            books
                        };
                        return Task::perform(future, Message::ImagesLoaded);
                    }
                }
                Task::none()
            }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
                Task::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::ImagesLoaded(books) => {
                println!("las imagenes se han cargado");
                self.books_list = books;
                self.loading_books = false;
                Task::none()
            }
            Message::SetCurrentBook(book_id) => {
                self.loading_data = true;

                let future = {
                    async move {
                        let mut book_details = BookDetails::new(book_id);
                        book_details.get_book_details().await;
                        book_details
                    }
                };

                Task::perform(future, |book_details| {
                    Message::CurrentBookLoaded(book_details)
                })
            }
            Message::CurrentBookLoaded(book) => {
                self.loading_data = false;
                self.title = format!("{} - {}", book.title, self.title);
                self.current_book = Some(book);
                Task::done(Message::NavigateTo("Books Details"))
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text(self.views.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_y(Center);

        let content = center(if self.explain {
            self.views.view(self).explain(color!(0x0000ff)) // Pasamos 'self'
        } else {
            self.views.view(self) // Pasamos 'self'
        })
        .padding(4);

        let controls = column(
            Views::LIST
                .iter()
                .filter(|view| !view.exclude)
                .map(|view| {
                    button(view.title)
                        .style(|theme: &Theme, status| {
                            let palette = theme.extended_palette();
                            match status {
                                button::Status::Active => button::Style {
                                    background: Some(iced::Background::Color(
                                        palette.background.base.color,
                                    )),
                                    text_color: palette.background.weak.text,
                                    ..Default::default()
                                },
                                button::Status::Hovered => button::Style {
                                    background: Some(iced::Background::Color(
                                        palette.background.strong.color,
                                    )),
                                    ..Default::default()
                                },
                                _ => button::Style::default(),
                            }
                        })
                        .width(Length::Fill)
                        .padding([5, 10])
                        .on_press(Message::NavigateTo(view.title))
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .spacing(10)
        .width(150);

        let footer = text("This is a footer").size(10);

        let content_and_sidebar = row![controls, content].spacing(10).padding(20);

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
struct Views {
    title: &'static str,
    view: fn(&Layout) -> Element<'static, Message>,
    exclude: bool,
}

impl Views {
    const LIST: &'static [Self] = &[
        Self {
            title: "Reader",
            view: reader_view,
            exclude: false,
        },
        Self {
            title: "Books",
            view: books_list_view,
            exclude: false,
        },
        Self {
            title: "Books Details",
            view: books_details_view,
            exclude: true,
        },
    ];

    fn find_by_title(title: &str) -> Option<Self> {
        Self::LIST
            .iter()
            .copied()
            .find(|example| example.title == title)
    }

    fn view(&self, layout: &Layout) -> Element<Message> {
        // Recibe una referencia a 'Layout'
        (self.view)(layout) // Llama a la función vista pasándole 'layout'
    }
}

impl Default for Views {
    fn default() -> Self {
        Self::LIST[0]
    }
}
