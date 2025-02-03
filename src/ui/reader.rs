use iced::mouse;
use iced::widget::{
    canvas, center, column, container, row, scrollable
};
use iced::{
    Center, Element, Fill, Length, Point, Rectangle, Renderer, Theme
};
use crate::Message;


pub fn reader_view<'a>() -> Element<'a, Message> {
    
    let sidebar = center(
        column!["Sidebar!", "More sidebar!"]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_x(Center),
    )
    .style(container::rounded_box);

    let content = container(
        scrollable(
            column![
                "Content!",
                row((1..10).map(|i| square(if i % 2 == 0 { 80 } else { 160 })))
                    .spacing(20)
                    .align_y(Center)
                    .wrap(),
                "The end"
            ]
            .spacing(40)
            .align_x(Center)
            .width(Fill),
        )
        .height(Fill),
    )
    .padding(10);

    column![row![sidebar, content]].into()
}


fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());
            let palette = theme.extended_palette();
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );
            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
