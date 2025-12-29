use crate::charactersheet::stattypes::StatTypes;



#[derive(Clone)]
pub enum Message {
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),

    LevelChanged(String),
    ExperienceAdd(i32),
    ExperienceRemoved(i32),

    SkillProficencyChanged(StatTypes),
    SkillExpertieseChanged(StatTypes),

    SaveToFile,
    LoadFromFile,
}