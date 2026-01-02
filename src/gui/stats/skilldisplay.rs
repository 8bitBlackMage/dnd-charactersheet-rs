use crate::charactersheet::abilities;
use crate::Message;

use iced::{Element, Length::{self}, widget::{row, text,checkbox,space}};


pub fn view (skill_id: abilities::SkillTypes , skill: abilities::Skill) -> Element<'static, Message> {
    row![
        text(abilities::get_skill_name(skill_id)).width(Length::FillPortion(3)), 
        text(skill.get_modifier_as_string()).width(Length::FillPortion(1)),
        checkbox(skill.proficient).on_toggle(move |_| Message::SkillProficencyChanged(skill_id)).width(Length::FillPortion(1)),
        space::horizontal(),
        checkbox(skill.expert).on_toggle(move |_| Message::SkillExpertieseChanged(skill_id)).width(Length::FillPortion(1)),
        ].into()
}