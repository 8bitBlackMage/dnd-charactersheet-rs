use std::usize;

#[derive(Clone,Copy)]
pub enum StatTypes {
    Strength(StrengthSkills),
    Dexterity(DexteritySkills),
    Constitution(ConstitutionSkills),
    Intellegence(IntellegenceSkills),
    Wisdom(WisdomSkills), 
    Charisma(CharismaSkills),
}

impl Into<usize> for StatTypes {
    fn into(self) -> usize {
        match self {
            StatTypes::Strength(strength_skills) => strength_skills as usize,
            StatTypes::Dexterity(dexterity_skills) => dexterity_skills as usize,
            StatTypes::Constitution(constitution_skills) => constitution_skills as usize,
            StatTypes::Intellegence(intellegence_skills) => intellegence_skills as usize,
            StatTypes::Wisdom(wisdom_skills) => wisdom_skills as usize,
            StatTypes::Charisma(charisma_skills) => charisma_skills as usize,
        }
    }
}


#[derive(Clone,Copy)]
pub enum StrengthSkills {
    Athletics 
}
#[derive(Clone,Copy)]
pub enum DexteritySkills{
    Acrobatics,
    SlightOfHand,
    Stealth
}
#[derive(Clone,Copy)]
pub enum ConstitutionSkills {

}
#[derive(Clone,Copy)]
pub enum IntellegenceSkills {
    Arcana,
    History,
    Investigation,
    Nature,
    Relgion,
}
#[derive(Clone,Copy)]
pub enum WisdomSkills {
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
}
#[derive(Clone,Copy)]
pub enum CharismaSkills{
    Deeption,
    Intimidation,
    Performance,
    Persuasion,
}