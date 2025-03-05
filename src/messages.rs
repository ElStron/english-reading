use crate::ui::{books::BooksList, books_details::BookDetails};
use iced::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(&'static str),
    ExplainToggled(bool),
    ThemeSelected(Theme),
    ImagesLoaded(BooksList),
    SetCurrentBook(i32),
    CurrentBookLoaded(BookDetails),
    //AboutPressed,
    //About(about::Message),
}
