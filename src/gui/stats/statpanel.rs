use iced::Length::FillPortion;
use iced::{Element};
use iced::widget::{column, row, rule, text};
use iced_aw::widget::LabeledFrame;

use crate::{statblocks, messages::Message};

use crate::gui::stats::skilldisplay;

pub fn view(stat_block: &'_  statblocks::StatBlock) -> Element<'_, Message> {

    LabeledFrame::new("Strength",
    column![
        row![
            column![
            text("Score"),
            text(stat_block.value.to_string()),
            ].width(FillPortion(1)),
            column![
            text("Modifier"),
            text(stat_block.get_modifier_as_string()),
            ].width(FillPortion(1)),
        ],
        rule::horizontal(2),
        column( stat_block.skills.iter().enumerate().map(
            |(_, skill)| {
                skilldisplay::view(skill.0,skill.1)
            }
        ))

        
    ].width(200).spacing(10)).into()
}