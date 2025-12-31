use iced::Length::FillPortion;
use iced::{Element};
use iced::widget::{column, row, rule, text};
use iced_aw::widget::LabeledFrame;

use crate::charactersheet::abilities;
use crate::messages::Message;

use crate::gui::stats::skilldisplay;

pub fn view(name: String, ability: &'_  abilities::Ability) -> Element<'_, Message> {

    LabeledFrame::new(text(name),
    column![
        row![
            column![
            text("Score"),
            text(ability.value.to_string()),
            ].width(FillPortion(1)),
            column![
            text("Modifier"),
            text(ability.get_modifier_as_string()),
            ].width(FillPortion(1)),
        ],
        rule::horizontal(2),
        // column( ability.skills.iter().enumerate().map(
        //     |(_, skill)| {
        //         skilldisplay::view(&skill.0,&skill.1)
        //     }
        // ))

        
    ].width(250).spacing(10)).into()
}