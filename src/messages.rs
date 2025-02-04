use iced::Theme;


#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(&'static str),
    ExplainToggled(bool),
    ThemeSelected(Theme),
    ImagesLoaded(),
}