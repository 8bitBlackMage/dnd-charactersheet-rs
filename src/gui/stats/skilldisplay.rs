use crate::{messages::Message, statblocks};

use iced::{Element, Length::{self}, widget::{row, text}};


pub fn view<'a>(skill_name: &'a str , skill: &'a statblocks::Skill) -> Element<'a, Message> {
    row![text(skill_name).width(Length::FillPortion(1)), text(skill.value).width(Length::FillPortion(1))].into()
}