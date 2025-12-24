use crate::messages::Message;

use iced::{Alignment::Center, Element, widget::{button, row, text}};


pub fn view() -> Element<'static, Message>{
    row![
            button( text("Save").align_x(Center)).width(100).on_press(Message::SaveToFile),
            button( text("Load").align_x(Center)).width(100).on_press(Message::LoadFromFile)
    ].into()
}