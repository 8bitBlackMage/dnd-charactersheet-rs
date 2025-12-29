use serde::{Deserialize, Serialize};



const LEVEL_UP_AMMOUNTS : [i32;20] = [
     0,
     300,
     900,
     2700, 
     6500,
     14000,
     23000,
     34000,
     48000,
     64000,
     85000,
     100000,
     120000,
     140000,
     165000,
     195000,
     225000,
     265000,
     305000,
     355000
];

fn calculate_current_level(current_experience: i32) -> i32 
{
    LEVEL_UP_AMMOUNTS.partition_point(|x| x < &current_experience).max(1).try_into().unwrap()
}

fn calculate_proficency_bonus(level: i32) -> i32
{
    match level {

        1_i32..=4_i32 => 2,
        5_i32..=8_i32 => 3,
        9_i32..=12_i32 => 4,
        13_i32..=16_i32 => 5,
        17_i32..=20_i32 => 6,
        _=> 6
    }
}


#[derive(Default,Debug,Serialize,Deserialize)]
pub struct Level {
    pub level: i32,
    pub is_milestone: bool,

    pub experience: i32,
    
    pub profiency_bonus: i32,
}


impl Level {

    pub fn new(level: i32, is_milestone: bool) -> Self {
        Level { level:level, is_milestone: is_milestone, experience: 0, profiency_bonus: calculate_proficency_bonus(level) }
    }

    pub fn level_from_str(&mut self, level: String){
        if level.is_empty() {self.level = 0;}
        match level.parse() {
            Ok(level_as_i32) => self.level = level_as_i32,
            Err(_) => return,
        }
    }

    pub fn add_experience(&mut self, ammount_to_add: i32) {
        if self.is_milestone {
            return
        }

        self.experience =  ammount_to_add;
    }
}


#[cfg(test)] 
mod tests{
    use super::*;

    #[test]
    fn test_level_calculation() {
        assert_eq!(calculate_current_level(0), 1);
        assert_eq!(calculate_current_level(500), 2);
        assert_eq!(calculate_current_level(500000), 20);
        assert_eq!(calculate_current_level(i32::MAX), 20);
    }

}