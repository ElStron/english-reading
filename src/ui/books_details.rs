use crate::{Layout, Message};
use iced::widget::{button, column, image, text};
use iced::Element;

pub fn books_details_view<'a>(layout: &Layout) -> Element<'static, Message> {
    if layout.loading_data {
        return column![text("Cargando...").size(40)].into();
    };

    let book_details = layout.current_book.as_ref().unwrap();
    let title = book_details.title.clone();

    column![
        text(title).size(40),
        text(book_details.description.clone()).size(20),
        button("Volver").on_press(Message::NavigateTo("Books")),
    ]
    .into()
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BookDetails {
    pub id: i32,
    pub title: String,
    pub imagen: String,
    pub description: String,
    #[serde(skip)]
    pub handle_imagen: Option<image::Handle>,
}
#[derive(PartialEq, Clone, Eq, Debug, serde::Deserialize)]
struct ApiBookDetails {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

impl BookDetails {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            title: "".to_string(),
            imagen: "".to_string(),
            description: "".to_string(),
            handle_imagen: None,
        }
    }

    pub async fn get_book_details(&mut self) {
        let details = match self.fetch_book_details().await {
            Ok(details) => details,
            Err(_) => {
                print!("Error al cargar los detalles del libro");
                BookDetails::new(0)
            }
        };

        self.clone_from(&details);
    }

    async fn fetch_book_details(&self) -> Result<BookDetails, reqwest::Error> {
        let url = format!(
            "https://jsonplaceholder.typicode.com/posts/{}",
            self.id.clone()
        );
        println!("url: {}", url);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let response: ApiBookDetails = reqwest::get(url).await?.json().await?;
            Ok(BookDetails {
                id: response.id,
                title: response.title,
                imagen: "https://picsum.photos/200/300".to_string(),
                description: response.body,
                handle_imagen: None,
            })
        }
        #[cfg(target_arch = "wasm32")]
        Ok(BookDetails {
            id: self.id.clone(),
            title: "example".to_string(),
            imagen: "example".to_string(),
            description: "example".to_string(),
            handle_imagen: None,
        })
    }
}
