use iced::futures;
use iced::widget::{
    column, container, row, scrollable, text, image
};
use iced::{
    Center, Element, Fill
};
use crate::{Message, Layout};

pub fn books_list_view<'a>(layout: &Layout) -> Element<'static, Message>  {

    if layout.books_list.books.iter().all(|b| b.handle_imagen.is_none()) {
        Message::ImagesLoaded();
        println!("Images not loaded");

    }

    let get_books = layout.books_list.books.clone();
    let content = container(
        scrollable(
            column![
                row(
                    get_books.iter().map(|book| {

                        let image_widget = match &book.handle_imagen {
                            Some(handle) => handle.clone(),
                            None => image::Handle::from_path("./45.png"),
                        };
                        column![
                            image(image_widget).width(100).height(200),  // The image
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
pub struct BooksList {
    books: Vec<Book>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct Book {
    title: String,
    imagen: String,
    description: String,
    pub handle_imagen: Option<image::Handle>,
}

impl BooksList {
    pub fn new() -> Self {
        Self {
            books: vec![
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://marketplace.canva.com/EAF55Kx4v24/1/0/1003w/canva-portada-libro-fantas%C3%ADa-ilustrativo-verde-_pp1hAU8znQ.jpg".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                    handle_imagen: None,
                },
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQjwEEekvhhMCqwDnajKcTseJ0HFCJ0sF012A&s".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                    handle_imagen: None,
                },
                Book {
                    title: "El principito".to_string(),
                    imagen: "https://marketplace.canva.com/EAFI171fL0M/1/0/1003w/canva-portada-de-libro-de-novela-ilustrado-color-azul-aqua-PQeWaiiK0aA.jpg".to_string(),
                    description: "este es un ejemplo de descripción".to_string(),
                    handle_imagen: None,
                }]
        }
    }

    pub fn get_books(&self) -> Vec<Book> {
        self.books.clone()
    }

    pub async fn fetch_images(&mut self) {
        let books = self.get_books();
        let mut books_with_images = vec![];

        for book in books {
            if book.handle_imagen.is_some() {
                println!("Image already loaded");
                books_with_images.push(book);
                continue;
            }

            let handle_imagen = self.fetch_image(&book.imagen).await; 
            let imagen = match handle_imagen {
                Ok(handle) => handle,
                Err(_) => image::Handle::from_path("https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png"),
                
            };
            books_with_images.push(Book {
                title: book.title,
                imagen: book.imagen,
                description: book.description,
                handle_imagen: Some(imagen),
            });
        }

        self.books = books_with_images;
    }

    async fn fetch_image(&self, id: &str) -> Result<image::Handle, reqwest::Error> {
        let url = format!(
            "{id}",
        );
    
        #[cfg(not(target_arch = "wasm32"))]
        {
            let bytes = reqwest::get(&url).await?.bytes().await?;
    
            Ok(image::Handle::from_bytes(bytes))
        }
    
        #[cfg(target_arch = "wasm32")]
        Ok(image::Handle::from_path(url))
    }
}

