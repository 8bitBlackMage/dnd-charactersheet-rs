use crate::messages::Message;

use iced::{Element, widget::{button, row}};


pub fn view() -> Element<'static, Message>{
    row![
            button("save").width(100).on_press(Message::SaveToFile),
            button("load").width(100).on_press(Message::LoadFromFile)
    ].into()
}