use crate::messages::Message;
use crate::gui::assets;
use iced::{Alignment::Center, Element, widget::{button, row, text}};


pub fn view() -> Element<'static, Message>{
    row![
            button(row! [assets::save_icon() ,text("Save").align_x(Center)]).width(100).on_press(Message::SaveToFile),
            button(row![assets::load_icon(),  text("Load").align_x(Center)]).width(100).on_press(Message::LoadFromFile),

    ].into()
}