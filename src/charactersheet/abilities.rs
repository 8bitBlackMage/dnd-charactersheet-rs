

use serde::{Deserialize, Serialize};


#[derive(Debug,PartialEq, Eq,Clone,Copy, Serialize,Deserialize,Hash)]
pub enum AbilityScoreTypes {
    Strength,
    Dexterity,
    Constitution,
    Intellegence,
    Wisdom, 
    Charisma,
}

#[derive(Debug,PartialEq, Eq,Clone,Copy, Serialize,Deserialize,Hash)]
pub enum SkillTypes {
    Athletics,
    Acrobatics,
    SlightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Relgion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Intimidation,
    Performance,
    Persuasion,
}


fn calculate_modifier(ability_score:i8) -> i8 {
    let unrounded: f32 = (ability_score as f32 - 10.0)  / 2.0;
    unrounded.floor() as i8
}

fn get_modifier_as_string(modifier:i8) -> String {
        if modifier >= 0 {
            return "+{}".replace("{}", &modifier.to_string());
        }
        else {
            return modifier.to_string();
        }
}



#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Skill {
    pub ability: AbilityScoreTypes,
    pub value: i8,
    pub proficient: bool,
    pub expert: bool
}
impl Skill {
    pub fn new(initial_value: i8, ability_Type: AbilityScoreTypes ) -> Self {
        Skill { value: initial_value, proficient: false, expert: false, ability: ability_Type }
    }
    pub fn get_modifier(&self) -> i8 {
        calculate_modifier(self.value)
    }
    pub fn get_modifier_as_string(&self)-> String {
        get_modifier_as_string(self.get_modifier())
    }
} 

#[derive(Debug, Default,Clone, Serialize, Deserialize)]
pub struct Ability {
    pub(crate) value: i8,
    modifier: i8,
    saving_throw: i8
}

impl Ability {
    pub fn new(initial_value: i8) -> Self {
        Ability { value: initial_value, modifier: calculate_modifier(initial_value), saving_throw: calculate_modifier(initial_value) }
    }
    pub fn get_modifier_as_string(&self) -> String {
            get_modifier_as_string(self.modifier)
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_skill_proficency_calculation() {
        let unproficient_skill : Skill = Skill { value: 3, proficient: false, expert: false, ability: AbilityScoreTypes::Charisma };
        assert_eq!(unproficient_skill.get_modifier(), 3);

        let proficient_skill : Skill = Skill { value: 3, proficient: true, expert: false, ability: AbilityScoreTypes::Charisma };
        assert_eq!(proficient_skill.get_modifier(), 6);

        let expert_skill : Skill = Skill { value: 3, proficient: true, expert: true, ability: AbilityScoreTypes::Charisma };
        assert_eq!(expert_skill.get_modifier(), 9);
    }

    #[test]
    fn test_ability_score_calculation(){
        assert_eq!(calculate_modifier(1), -5);
        assert_eq!(calculate_modifier(2), -4);
        assert_eq!(calculate_modifier(3), -4);
        assert_eq!(calculate_modifier(4), -3);
        assert_eq!(calculate_modifier(5), -3);
        assert_eq!(calculate_modifier(6), -2);
        assert_eq!(calculate_modifier(7), -2);
        assert_eq!(calculate_modifier(8), -1);
        assert_eq!(calculate_modifier(9), -1);
        assert_eq!(calculate_modifier(10), 0);
        assert_eq!(calculate_modifier(11), 0);
        assert_eq!(calculate_modifier(12), 1);
        assert_eq!(calculate_modifier(13), 1);
        assert_eq!(calculate_modifier(14), 2);
        assert_eq!(calculate_modifier(15), 2);
        assert_eq!(calculate_modifier(16), 3);
        assert_eq!(calculate_modifier(17), 3);
        assert_eq!(calculate_modifier(18), 4);
        assert_eq!(calculate_modifier(19), 4);
        assert_eq!(calculate_modifier(20), 5);
        assert_eq!(calculate_modifier(21), 5);
        assert_eq!(calculate_modifier(22), 6);
        assert_eq!(calculate_modifier(23), 6);
        assert_eq!(calculate_modifier(24), 7);
        assert_eq!(calculate_modifier(25), 7);
        assert_eq!(calculate_modifier(26), 8);
        assert_eq!(calculate_modifier(27), 8);
        assert_eq!(calculate_modifier(28), 9);
        assert_eq!(calculate_modifier(29), 9);
        assert_eq!(calculate_modifier(30), 10);

    
    }
}