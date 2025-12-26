use iced::Alignment::Center;
use iced::Length;
use iced::widget::{button, column, row, text, text_input};
use iced_aw::widgets::LabeledFrame;

use crate::level;
use crate::Message;



pub fn view(level: &level::Level) -> iced::Element<'static, Message> {
    LabeledFrame::new("Level",
    column![
            row![
                text("Current Level: ")
                    .width(iced::Length::FillPortion(3)),

                text_input("",&level.level.to_string())
                    .width(iced::Length::FillPortion(1))
                    .on_input(Message::LevelChanged)
                ].align_y(Center),  
            row![
                text("Experience ")
                    .width(Length::FillPortion(3)),
                    

                button(text("+").align_x(Center))
                    .width(Length::FillPortion(1)),

                text(level.experience.to_string())
                    .align_x(Center)
                    .width(Length::FillPortion(1)), 

                button(text("-").align_x(Center))
                    .width(Length::FillPortion(1))
            ].align_y(Center),
            row![ 
                text("Proficiency Bonus:")
                    .width(iced::Length::FillPortion(3)),
                    
                text("+{}".replace("{}",&level.profiency_bonus.to_string()))
                    .align_x(Center)
                    .width(iced::Length::FillPortion(1))
                ]
    ].padding(5).width(250).spacing(10)
    ).into()
}