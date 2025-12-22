fn calculate_modifier(ability_score:i32) -> i32 {
    let unrounded: f32 = (ability_score as f32 - 10.0)  / 2.0;
    println!("{}", unrounded);
    unrounded.floor() as i32
}




#[derive(Default)]
pub struct Skill {
    pub value: i32,
    pub proficient: bool,
    pub expert: bool
}
impl Skill {
    fn get_modifier(&self) -> i32 {
        self.value + if self.proficient {self.value} else {0} + if self.expert {self.value} else {0}
    }
} 


trait StatBlock {
    fn propogate_changes(&self);
    fn get_modifier(&self) -> i32;
    
}

#[derive(Default)]
pub struct Strength{
    value: i32,
    athletics: Skill
}

#[derive(Default)]
pub struct Intellgence{
    value: i32,
    arcana: Skill,
    history: Skill,
    investigation: Skill,
    nature: Skill, 
    religion: Skill,

}

impl StatBlock for Strength{
    fn propogate_changes(&self) {
        
    }

    fn get_modifier(&self) -> i32 {
        calculate_modifier(self.value)
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