use iced::{
    alignment,
    widget::{button, container, row, text, Column, Container},
    Alignment, Element, Length,
};
use iced_aw::{modal, Card, Spinner};

use super::ui::Message;

pub fn tip<'a>(state: bool, c: Container<'a, Message>, body: &'a str) -> Container<'a, Message> {
    let overlay = if state {
        Some(
            Card::new(
                text("提示")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
                text(format!("{}", body)).horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .foot(
                row![button(
                    text("确认").horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .on_press(Message::CloseModal),]
                .spacing(10)
                .padding(5)
                .width(Length::Fill),
            )
            .max_width(300.0)
            .on_close(Message::CloseModal),
        )
    } else {
        None
    };
    let tip = modal(c, overlay)
        .backdrop(Message::OpenModal)
        .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center);

    container(tip).into()
}
pub fn loading_message<'a>() -> Element<'a, Message> {
    container(
        Card::new(
            text("软件自检中")
                .horizontal_alignment(alignment::Horizontal::Center)
                .width(Length::Fill),
            Column::new()
                .push(Spinner::new().circle_radius(2.0).width(Length::Fill))
                .push(text("正在自检....请稍后"))
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10)
                .width(450),
        )
        .max_width(450.0),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}
