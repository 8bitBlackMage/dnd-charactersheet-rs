use crate::{charactersheet::{statblocks, stattypes::{IntellegenceSkills, StatTypes}}, messages::Message};

use iced::{Element, Length::{self}, widget::{row, text,checkbox,space}};


pub fn view<'a>(skill_name: &'a str , skill: &'a statblocks::Skill) -> Element<'a, Message> {
    row![
        text(skill_name).width(Length::FillPortion(3)), 
        text(skill.get_modifier_as_string()).width(Length::FillPortion(1)),
        checkbox(skill.proficient).width(Length::FillPortion(1)),
        space::horizontal(),
        checkbox(skill.expert).on_toggle(|_| Message::SkillExpertieseChanged(StatTypes::Intellegence(IntellegenceSkills::Arcana))).width(Length::FillPortion(1)),
        ].into()
}