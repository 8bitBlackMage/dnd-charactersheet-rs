use crate::character;
use crate::messages::Message;
use iced::Element;
use iced::widget::{button, column, row, text, text_input};
use iced::Center;
use iced::alignment::Horizontal::Left;




pub fn view(character: &'_  character::Character) -> Element<'_, Message> {
        column![
            row![
                text("name: "),

                text_input("",&character.name).on_input(Message::NameChanged)
                ].align_y(Center), 

            row![
                text("Class: "), 
                text_input("",&character.class).on_input(Message::ClassChanged)
                ].align_y(Center), 

            row![
                text("SubClass: "), 
                text_input("",&character.subclass).on_input(Message::SubclassChanged)
                ].align_y(Center), 
            row![
                text("Species: "), 
                text_input("",&character.species).on_input(Message::SpeciesChanged)
                ].align_y(Center), 
            row![
            button("save").width(100).on_press(Message::SaveToFile),
            button("load").width(100).on_press(Message::LoadFromFile)

            ]
        ].padding(5).into()

}