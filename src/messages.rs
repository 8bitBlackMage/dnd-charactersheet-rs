use crate::charactersheet::abilities::SkillTypes;



#[derive(Clone)]
pub enum Message {
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),

    LevelChanged(String),
    ExperienceAdd(i8),
    ExperienceRemoved(i32),

    SkillProficencyChanged(SkillTypes),
    SkillExpertieseChanged(SkillTypes),

    SaveToFile,
    LoadFromFile,
}