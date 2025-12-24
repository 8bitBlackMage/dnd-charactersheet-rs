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
                    .width(iced::Length::FillPortion(2)),

                text_input("",&level.level.to_string())
                    .width(iced::Length::FillPortion(3))
                    .on_input(Message::levelChanged)
                ].align_y(Center),  
            row![
                text("Experience ")
                    .width(Length::FillPortion(1)),
                    

                button(text("+").align_x(Center))
                    .width(Length::FillPortion(1)),

                text(level.experience.to_string())
                    .align_x(Center)
                    .width(Length::FillPortion(1)), 

                button(text("-").align_x(Center))
                    .width(Length::FillPortion(1))
            ].align_y(Center),
            row![ text(  "Proficiency Bonus: +{}".replace("{}",&level.profiency_bonus.to_string())).align_x(Center)  ]
    ].padding(5).width(350)
    ).into()
}