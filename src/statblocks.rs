use std::collections::HashMap;

use serde::{Deserialize, Serialize};

fn calculate_modifier(ability_score:i32) -> i32 {
    let unrounded: f32 = (ability_score as f32 - 10.0)  / 2.0;
    println!("{}", unrounded);
    unrounded.floor() as i32
}




#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Skill {
    pub value: i32,
    pub proficient: bool,
    pub expert: bool
}
impl Skill {
    pub fn new(initial_value: i32) -> Self {
        Skill { value: initial_value, proficient: false, expert: false }
    }
    fn get_modifier(&self) -> i32 {
        self.value + if self.proficient {self.value} else {0} + if self.expert {self.value} else {0}
    }
} 

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatBlock {
    pub(crate) value: i32,
    modifier: i32,
    pub(crate) skills: HashMap<String, Skill>
}



impl StatBlock {
    pub fn new_strength_block(initial_value: i32) -> Self {
        StatBlock { 
            value: initial_value, 
            modifier: calculate_modifier(initial_value), 
            skills: HashMap::from([
                ("Athletics".to_string(), Skill::new(initial_value))
            ])
        }
    }
    pub fn new_intellegence_block(initial_value: i32) -> Self {
        StatBlock { 
            value: initial_value, 
            modifier: calculate_modifier(initial_value), 
            skills: HashMap::from([
                ("Arcana".to_string(), Skill::new(initial_value)),
                ("History".to_string(), Skill::new(initial_value)),
                ("Investigation".to_string(), Skill::new(initial_value)),
                ("Nature".to_string(), Skill::new(initial_value)),
                ("Relgion".to_string(), Skill::new(initial_value)),
            ])
        }
    }

    fn propogate_changes(&self) {
        
    }

    fn get_modifier(&self) -> i32 {
        calculate_modifier(self.value)
    }
    
    pub fn get_modifier_as_string(&self) -> String {
        let modifer = self.get_modifier();
        if modifer > 0 {
            return "+{}".replace("{}", &modifer.to_string());
        }
        else {
            return modifer.to_string();
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_skill_proficency_calculation() {
        let unproficient_skill : Skill = Skill { value: 3, proficient: false, expert: false };
        assert_eq!(unproficient_skill.get_modifier(), 3);

        let proficient_skill : Skill = Skill { value: 3, proficient: true, expert: false };
        assert_eq!(proficient_skill.get_modifier(), 6);

        let expert_skill : Skill = Skill { value: 3, proficient: true, expert: true };
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