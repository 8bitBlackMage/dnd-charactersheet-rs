use crate::character;
use crate::messages::Message;
use iced::Element;
use iced::widget::{column, row, text, text_input};
use iced::Center;
use iced_aw::widget::{LabeledFrame, };


pub fn view(character: &'_  character::Character) -> Element<'_, Message> {
        LabeledFrame::new("Character Info",
        column![
            row![
                text("Name: ")
                    .width(iced::Length::FillPortion(2)),

                text_input("",&character.name)
                    .width(iced::Length::FillPortion(3))
                    .on_input(Message::NameChanged)
                ].align_y(Center), 

            row![
                text("Class: ")
                    .width(iced::Length::FillPortion(2)), 

                text_input("",&character.class)
                    .width(iced::Length::FillPortion(3))
                    .on_input(Message::ClassChanged)
                ].align_y(Center), 

            row![
                text("Sub Class: ")
                    .width(iced::Length::FillPortion(2)), 

                text_input("",&character.subclass)
                    .width(iced::Length::FillPortion(3))
                    .on_input(Message::SubclassChanged)
                ].align_y(Center), 
            row![
                text("Species: ")
                    .width(iced::Length::FillPortion(2)), 

                text_input("",&character.species)
                    .width(iced::Length::FillPortion(3))
                    .on_input(Message::SpeciesChanged)
                ].align_y(Center), 
        ].padding(5).width(300).spacing(10)).into()

}