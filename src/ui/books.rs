use iced::futures::future::join_all;
use iced::widget::{
    button, column, container, image, row, scrollable, text, center
};
use iced::{
    Center, Element, Fill
};
use crate::{Message, Layout};

pub fn books_list_view<'a>(layout: &Layout) -> Element<'static, Message>  {

    let books_mapped = match layout.loading_books {
        true => (0..30).map(|_| {
            //
            column![
                center("Loading").style(container::rounded_box),
            ].width(200).height(300).into()
        }).collect::<Vec<Element<'_, Message>>>(),
        false => layout.books_list.books.iter().map(|book| {
            let image_widget = match &book.handle_imagen {
                Some(handle) => handle.clone(),
                None => image::Handle::from_path("./45.png"),
            };
            
            let button = button(
                column![image(image_widget).width(200),
                text(book.title.clone()).size(12), 
            ]
            .max_width(120)
            ).on_press(Message::NavigateTo("Reader"));

            button.into()
        }).collect::<Vec<Element<'_, Message>>>()
    };

    let content = container(
        scrollable(
            column![
                row(books_mapped)
                .spacing(15)
                .wrap(),
            ]
            .spacing(40)
            .align_x(Center)
            .width(Fill),
        )
        .height(Fill),
    ).width(Fill);
    content.into()
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

#[derive(PartialEq, Clone, Eq, Debug,serde::Deserialize)]
struct ApiBook {
    title: String,
    url: String,
}

impl BooksList {
    pub fn new() -> Self {
        Self {
            books: vec![]
        }
    }

    pub fn get_books(&self) -> Vec<Book> {
        
        self.books.clone()
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>, reqwest::Error> {
        let url = "https://jsonplaceholder.typicode.com/albums/1/photos";
    
        #[cfg(not(target_arch = "wasm32"))]
        {
            let books: Vec<ApiBook> = reqwest::get(url).await?.json().await?;
            let books_with_images = join_all(
                books.into_iter().map(|book| async {
                let handle_imagen = self
                    .fetch_image("https://picsum.photos/200/300").await; 

                let imagen = match handle_imagen {
                    Ok(handle) => handle,
                    Err(_) => image::Handle::from_path("./45.png"),
                };
                Book {
                    title: book.title.clone(),
                    imagen: "https://picsum.photos/200/300".to_string(),
                    description: book.title,
                    handle_imagen: Some(imagen),
                }
            })).await;
            Ok(books_with_images)
        }
        #[cfg(target_arch = "wasm32")]
        Ok(vec![])

    } 

    pub async fn fetch_images(&mut self) {

        match self.fetch_books().await {
            Ok(books_with_images) => {
                self.books = books_with_images;
            }
            Err(_) => {
                println!("Error loading images");
            }
        }
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
        Ok(image::Handle::from_path("./45.png"))
    }
}

