
#[derive(Clone)]
pub enum Message{
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),

    levelChanged(String),
    experienceAdd(i32),
    experienceRemoved(i32),

    SaveToFile,
    LoadFromFile,
}