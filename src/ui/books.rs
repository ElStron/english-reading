use iced::widget::{
    column, container, row, scrollable, text, image
};
use iced::{
    Center, Element, Fill
};
use crate::Message;

pub fn books_list_view<'a>() -> Element<'a, Message> {

    let get_books = BooksList::new();
    let book_list = get_books.books;

    let content = container(
        scrollable(
            column![
                row(
                    book_list.iter().map(|book| {

                        let image_widget = image::Handle::from_path(book.imagen.clone());
                        println!("Image: {:?}", image_widget); // return: Image: Path("https://...imge.jpg")
                        column![
                            image(image_widget).width(100).height(100),  // The image
                            text(book.title.clone()).size(20), 
                            text(book.description.clone())// Title, centered vertically
                        ]
                        .spacing(20)
                        .into()
                    })
                )
            ]
            .spacing(40)
            .align_x(Center)
            .width(Fill),
        )
        .height(Fill),
    )
    .padding(10);

    column![content].into()
}

#[derive(Default,Debug, Clone, PartialEq, Eq)]
struct BooksList {
    books: Vec<Book>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
struct Book {
    title: String,
    imagen: String,
    description: String,
}

impl BooksList {
    fn new() -> Self {
        Self {
            books: vec![
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://marketplace.canva.com/EAF55Kx4v24/1/0/1003w/canva-portada-libro-fantas%C3%ADa-ilustrativo-verde-_pp1hAU8znQ.jpg".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                },
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQjwEEekvhhMCqwDnajKcTseJ0HFCJ0sF012A&s".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                },
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://marketplace.canva.com/EAFI171fL0M/1/0/1003w/canva-portada-de-libro-de-novela-ilustrado-color-azul-aqua-PQeWaiiK0aA.jpg".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                }]
        }
    }
}