use crate::charactersheet::abilities::AbilityScoreTypes;



#[derive(Clone)]
pub enum Message {
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),

    LevelChanged(String),
    ExperienceAdd(i32),
    ExperienceRemoved(i32),

    SkillProficencyChanged(AbilityScoreTypes),
    SkillExpertieseChanged(AbilityScoreTypes),

    SaveToFile,
    LoadFromFile,
}