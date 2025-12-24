
#[derive(Clone)]
pub enum Message{
    NameChanged(String),
    ClassChanged(String),
    SubclassChanged(String),
    SpeciesChanged(String),
    SaveToFile,
    LoadFromFile,
}