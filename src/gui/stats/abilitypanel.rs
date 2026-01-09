
use iced::Length::FillPortion;
use iced::{Element};
use iced::widget::{column, row, rule, text};
use iced_aw::widget::LabeledFrame;

use crate::charactersheet::abilities::{self, AbilityScoreTypes, SkillTypes};
use crate::charactersheet::character::Character;
use crate::gui::stats::skilldisplay;
use crate::messages::Message;


pub fn view( character: & Character, ability_id: AbilityScoreTypes,skill_ids: Vec<SkillTypes>, ) -> Element<'static, Message> {
    let ability = character.get_ability(ability_id);
     LabeledFrame::new(text(abilities::get_ability_name(ability_id)),
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
        column(skill_ids.iter().enumerate().map(
            |(_,skill_id)| {
                skilldisplay::view(*skill_id, character.get_skill(*skill_id))
            }
        ))

        
    ].width(270).spacing(10)).into()
}