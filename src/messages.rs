use iced::Theme;
use crate::ui::books::BooksList;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(&'static str),
    ExplainToggled(bool),
    ThemeSelected(Theme),
    ImagesLoaded( BooksList ),
}