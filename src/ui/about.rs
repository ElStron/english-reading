use iced::widget::text;
use iced::{Element, Theme};
#[derive(Debug, Clone)]
pub enum Message {
    ShowAboutDialog,
}

struct About;

impl About {
    pub fn view<'a>() -> Element<'a, Message> {
        // About dialog
        text("About dialog").into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowAboutDialog => {
                // Show about dialog
            }
        }
    }
}
